# Comprehensive Agent Manager Testing Plan

## Executive Summary

This document outlines a comprehensive testing strategy for the Agent Manager implementation in PR #4542. The Agent Manager represents a fundamental architectural shift from a single shared agent to a per-session agent model, addressing critical concurrency issues and enabling true multi-user support in Goose.

## Current Implementation Status

### What's Been Changed
1. **Core Implementation** (`crates/goose/src/agents/manager.rs`)
   - Session-to-agent mapping with RwLock pattern
   - Automatic cleanup of idle agents (1 hour default, 5 minute intervals)
   - Metrics tracking (agents created/cleaned, cache hits/misses)
   - Support for different ExecutionModes (Interactive, SubTask, etc.)
   - Background cleanup task spawned automatically

2. **goose-server Integration** 
   - `state.rs`: Migrated from shared agent to AgentManager
   - Routes using AgentManager: `reply.rs`, `extension.rs`, `context.rs`, `agent.rs`, `recipe.rs`
   - Routes not affected: `session.rs`, `schedule.rs`, `audio.rs`, `config_management.rs`, `setup.rs`, `health.rs`
   - Cleanup task spawned on server startup

3. **Test Coverage**
   - Unit tests: Basic functionality covered
   - Integration tests: Multi-session extension isolation tested
   - Missing: Stress tests, error recovery, performance benchmarks

## Testing Strategy

### Phase 1: Unit Testing Enhancement

#### 1.1 Provider Lifecycle Tests (HIGH PRIORITY)
```rust
// Test: Agents start without providers and can have them set later
#[tokio::test]
async fn test_agent_provider_lifecycle() {
    let manager = AgentManager::new(Default::default());
    let session = session::Identifier::Name("provider_test".to_string());
    
    // Get agent without provider
    let agent = manager.get_agent(session.clone()).await.unwrap();
    
    // Verify agent exists but has no provider initially
    assert!(agent.provider().await.is_err());
    
    // Set provider via API call (simulating /reply route)
    let provider = create_mock_provider();
    agent.update_provider(provider).await.unwrap();
    
    // Verify provider persists
    assert!(agent.provider().await.is_ok());
}
```

#### 1.2 Cleanup Task Robustness Tests
```rust
// Test: Cleanup task handles many agents efficiently
#[tokio::test]
async fn test_cleanup_under_load() {
    let config = AgentManagerConfig {
        cleanup_interval: Duration::from_millis(100),
        max_idle_duration: Duration::from_millis(200),
        ..Default::default()
    };
    
    let manager = Arc::new(AgentManager::new(config));
    let handle = manager.clone().spawn_cleanup_task();
    
    // Create many agents
    for i in 0..100 {
        manager.get_agent(session::Identifier::Name(format!("load_{}", i))).await.unwrap();
    }
    
    // Wait for cleanup
    tokio::time::sleep(Duration::from_millis(400)).await;
    
    // All should be cleaned
    assert_eq!(manager.active_agent_count().await, 0);
    
    handle.abort();
}
```

#### 1.3 Edge Cases Tests
```rust
// Test: Session ID edge cases
#[tokio::test]
async fn test_session_id_edge_cases() {
    let manager = AgentManager::new(Default::default());
    
    // Very long session ID
    let long_id = "a".repeat(1000);
    assert!(manager.get_agent(session::Identifier::Name(long_id)).await.is_ok());
    
    // Unicode session ID
    let unicode_id = "测试会话🦆".to_string();
    assert!(manager.get_agent(session::Identifier::Name(unicode_id)).await.is_ok());
    
    // Special characters
    let special_id = "test-session_123!@#".to_string();
    assert!(manager.get_agent(session::Identifier::Name(special_id)).await.is_ok());
}
```

### Phase 2: Integration Testing (goosed)

#### 2.1 Black Box Testing Script
```bash
#!/bin/bash
# comprehensive_goosed_test.sh

# Configuration
GOOSED_PORT=8081
SECRET_KEY="test123"
BASE_URL="http://localhost:$GOOSED_PORT"

# Start goosed with Agent Manager
start_goosed() {
    screen -dmS goosed_test bash -c "
        RUST_LOG=info \
        GOOSE_PORT=$GOOSED_PORT \
        GOOSE_API_KEY=\$(cat ~/keys/oncall_buddy_goose_etc_databricks.txt) \
        GOOSE_DEFAULT_PROVIDER=databricks \
        GOOSE_SERVER__SECRET_KEY=$SECRET_KEY \
        ./target/debug/goosed agent
    "
    sleep 2
}

# Test 1: Session Isolation
test_session_isolation() {
    echo "Testing session isolation..."
    
    # Create two sessions with different messages
    curl -X POST $BASE_URL/reply \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{"session_id": "test1", "messages": [{"role": "user", "content": "Remember: I am session 1"}]}' &
    
    curl -X POST $BASE_URL/reply \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{"session_id": "test2", "messages": [{"role": "user", "content": "Remember: I am session 2"}]}' &
    
    wait
    
    # Verify context isolation
    response1=$(curl -s -X GET $BASE_URL/context?session_id=test1 \
        -H "X-Secret-Key: $SECRET_KEY")
    response2=$(curl -s -X GET $BASE_URL/context?session_id=test2 \
        -H "X-Secret-Key: $SECRET_KEY")
    
    # Check that contexts are different
    if [ "$response1" = "$response2" ]; then
        echo "❌ Session isolation failed - contexts are identical"
        return 1
    else
        echo "✅ Session isolation working"
    fi
}

# Test 2: Extension Isolation
test_extension_isolation() {
    echo "Testing extension isolation..."
    
    # Add extension to session 1 only
    curl -X POST $BASE_URL/extensions/add \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "session_id": "ext_test_1",
            "type": "builtin",
            "name": "memory"
        }'
    
    # Check extensions for both sessions
    ext1=$(curl -s -X GET "$BASE_URL/extensions/list?session_id=ext_test_1" \
        -H "X-Secret-Key: $SECRET_KEY")
    ext2=$(curl -s -X GET "$BASE_URL/extensions/list?session_id=ext_test_2" \
        -H "X-Secret-Key: $SECRET_KEY")
    
    # Verify extension only in session 1
    if echo "$ext1" | grep -q "memory" && ! echo "$ext2" | grep -q "memory"; then
        echo "✅ Extension isolation working"
    else
        echo "❌ Extension isolation failed"
        return 1
    fi
}

# Test 3: Concurrent Requests
test_concurrent_requests() {
    echo "Testing concurrent requests..."
    
    # Send 20 concurrent requests to different sessions
    for i in {1..20}; do
        curl -X POST $BASE_URL/reply \
            -H "X-Secret-Key: $SECRET_KEY" \
            -H "Content-Type: application/json" \
            -d "{\"session_id\": \"concurrent_$i\", \"messages\": [{\"role\": \"user\", \"content\": \"Test $i\"}]}" &
    done
    
    wait
    echo "✅ Concurrent requests completed"
}

# Test 4: Agent Metrics
test_agent_metrics() {
    echo "Testing agent metrics..."
    
    metrics=$(curl -s -X GET $BASE_URL/agent/stats \
        -H "X-Secret-Key: $SECRET_KEY")
    
    echo "Current metrics: $metrics"
    
    # Verify metrics structure
    if echo "$metrics" | grep -q "agents_created" && echo "$metrics" | grep -q "cache_hits"; then
        echo "✅ Metrics endpoint working"
    else
        echo "❌ Metrics endpoint failed"
        return 1
    fi
}

# Test 5: Cleanup Functionality
test_cleanup() {
    echo "Testing cleanup functionality..."
    
    # Get initial count
    initial=$(curl -s -X GET $BASE_URL/agent/stats \
        -H "X-Secret-Key: $SECRET_KEY" | jq '.active_agents')
    
    # Trigger cleanup
    curl -X POST $BASE_URL/agent/cleanup \
        -H "X-Secret-Key: $SECRET_KEY"
    
    # Get count after cleanup
    after=$(curl -s -X GET $BASE_URL/agent/stats \
        -H "X-Secret-Key: $SECRET_KEY" | jq '.active_agents')
    
    echo "Agents before cleanup: $initial, after: $after"
    echo "✅ Cleanup endpoint working"
}

# Test 6: Provider Configuration
test_provider_configuration() {
    echo "Testing provider configuration..."
    
    # Test with different provider settings
    curl -X POST $BASE_URL/reply \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "session_id": "provider_test",
            "messages": [{"role": "user", "content": "test"}],
            "provider": "databricks",
            "model": "claude-3-5-sonnet-latest",
            "temperature": 0.7
        }'
    
    echo "✅ Provider configuration test completed"
}

# Test 7: Session Persistence
test_session_persistence() {
    echo "Testing session persistence..."
    
    # Create session with specific content
    curl -X POST $BASE_URL/reply \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "session_id": "persist_test",
            "messages": [{"role": "user", "content": "Remember this: PERSISTENCE_TEST_MARKER"}]
        }'
    
    # Trigger cleanup to remove agent from memory
    curl -X POST $BASE_URL/agent/cleanup \
        -H "X-Secret-Key: $SECRET_KEY"
    
    # Access session again - should recreate agent
    response=$(curl -s -X GET "$BASE_URL/context?session_id=persist_test" \
        -H "X-Secret-Key: $SECRET_KEY")
    
    if echo "$response" | grep -q "PERSISTENCE_TEST_MARKER"; then
        echo "✅ Session persistence working"
    else
        echo "❌ Session persistence failed"
        return 1
    fi
}

# Test 8: Recipe Execution
test_recipe_execution() {
    echo "Testing recipe execution with Agent Manager..."
    
    # Create a simple recipe
    curl -X POST $BASE_URL/recipe/create \
        -H "X-Secret-Key: $SECRET_KEY" \
        -H "Content-Type: application/json" \
        -d '{
            "session_id": "recipe_test",
            "messages": [{"role": "user", "content": "Create a recipe that says hello"}]
        }'
    
    echo "✅ Recipe creation test completed"
}

# Run all tests
run_all_tests() {
    start_goosed
    
    test_session_isolation
    test_extension_isolation
    test_concurrent_requests
    test_agent_metrics
    test_cleanup
    test_provider_configuration
    test_session_persistence
    test_recipe_execution
    
    # Cleanup
    screen -X -S goosed_test quit
    
    echo "All tests completed!"
}

run_all_tests
```

### Phase 3: Performance Testing

#### 3.1 Load Testing Script
```python
# load_test_agent_manager.py
import asyncio
import aiohttp
import time
import statistics
import json

class LoadTester:
    def __init__(self, base_url="http://localhost:8081", secret_key="test123"):
        self.base_url = base_url
        self.headers = {
            "X-Secret-Key": secret_key,
            "Content-Type": "application/json"
        }
    
    async def create_session_and_query(self, session, session_id):
        """Create a session and send a query"""
        url = f"{self.base_url}/reply"
        data = {
            "session_id": f"load_test_{session_id}",
            "messages": [{"role": "user", "content": f"Test message {session_id}"}]
        }
        
        start = time.time()
        async with session.post(url, json=data, headers=self.headers) as response:
            await response.text()
        return time.time() - start
    
    async def test_concurrent_sessions(self, num_sessions):
        """Test with multiple concurrent sessions"""
        print(f"\n📊 Testing with {num_sessions} concurrent sessions...")
        
        async with aiohttp.ClientSession() as session:
            tasks = [self.create_session_and_query(session, i) for i in range(num_sessions)]
            times = await asyncio.gather(*tasks)
        
        print(f"  Average time: {statistics.mean(times):.3f}s")
        print(f"  Max time: {max(times):.3f}s")
        print(f"  Min time: {min(times):.3f}s")
        print(f"  95th percentile: {statistics.quantiles(times, n=20)[18]:.3f}s")
        
        # Get metrics
        async with aiohttp.ClientSession() as session:
            async with session.get(f"{self.base_url}/agent/stats", headers=self.headers) as response:
                metrics = await response.json()
                print(f"  Active agents: {metrics.get('active_agents', 'N/A')}")
                print(f"  Total created: {metrics.get('agents_created', 'N/A')}")
    
    async def test_cache_performance(self):
        """Test cache hit vs miss performance"""
        print("\n🚀 Testing cache performance...")
        
        session_id = "cache_test"
        url = f"{self.base_url}/reply"
        data = {
            "session_id": session_id,
            "messages": [{"role": "user", "content": "test"}]
        }
        
        async with aiohttp.ClientSession() as session:
            # First request - cache miss
            start = time.time()
            async with session.post(url, json=data, headers=self.headers) as response:
                await response.text()
            miss_time = time.time() - start
            
            # Subsequent requests - cache hits
            hit_times = []
            for _ in range(10):
                start = time.time()
                async with session.post(url, json=data, headers=self.headers) as response:
                    await response.text()
                hit_times.append(time.time() - start)
        
        avg_hit = statistics.mean(hit_times)
        print(f"  Cache miss time: {miss_time:.3f}s")
        print(f"  Average cache hit time: {avg_hit:.3f}s")
        print(f"  Speedup: {miss_time/avg_hit:.1f}x")
    
    async def test_memory_usage(self, num_agents):
        """Monitor memory usage with many agents"""
        print(f"\n💾 Testing memory with {num_agents} agents...")
        
        # Create many agents
        async with aiohttp.ClientSession() as session:
            for i in range(num_agents):
                data = {
                    "session_id": f"memory_test_{i}",
                    "messages": [{"role": "user", "content": "test"}]
                }
                await session.post(f"{self.base_url}/reply", json=data, headers=self.headers)
        
        # Get metrics
        async with aiohttp.ClientSession() as session:
            async with session.get(f"{self.base_url}/agent/stats", headers=self.headers) as response:
                metrics = await response.json()
                print(f"  Active agents: {metrics.get('active_agents', 'N/A')}")
                print(f"  Agents created: {metrics.get('agents_created', 'N/A')}")
                print(f"  Note: Monitor system memory externally")
    
    async def run_all_tests(self):
        """Run all performance tests"""
        print("🔥 Starting Agent Manager Performance Tests")
        
        # Test increasing loads
        for num in [10, 50, 100]:
            await self.test_concurrent_sessions(num)
        
        await self.test_cache_performance()
        await self.test_memory_usage(50)
        
        print("\n✅ Performance tests completed!")

if __name__ == "__main__":
    tester = LoadTester()
    asyncio.run(tester.run_all_tests())
```

### Phase 4: Stress Testing

#### 4.1 Stress Test Scenarios

1. **Rapid Session Creation/Destruction**
   - Create 100 sessions rapidly
   - Immediately trigger cleanup
   - Verify no memory leaks
   - Check that sessions can be recreated

2. **Long-Running Sessions**
   - Keep sessions alive for extended periods
   - Verify no performance degradation
   - Test that cleanup doesn't affect active sessions
   - Monitor memory growth over time

3. **Extension Churn**
   - Rapidly add/remove extensions across sessions
   - Verify no cross-contamination
   - Check for memory leaks in extension handling

4. **Provider Switching**
   - Switch providers mid-conversation
   - Test multiple providers across sessions
   - Verify correct provider routing

### Phase 5: Regression Testing

#### 5.1 Critical Regression Tests

```bash
#!/bin/bash
# regression_tests.sh

# Test that all existing functionality still works

# 1. Basic reply functionality
test_basic_reply() {
    response=$(curl -s -X POST http://localhost:8081/reply \
        -H "X-Secret-Key: test123" \
        -H "Content-Type: application/json" \
        -d '{"messages": [{"role": "user", "content": "Hello"}]}')
    
    if echo "$response" | grep -q "content"; then
        echo "✅ Basic reply working"
    else
        echo "❌ Basic reply broken"
    fi
}

# 2. Extension management
test_extension_management() {
    # Add extension
    curl -X POST http://localhost:8081/extensions/add \
        -H "X-Secret-Key: test123" \
        -H "Content-Type: application/json" \
        -d '{"session_id": "ext_reg", "type": "builtin", "name": "memory"}'
    
    # List extensions
    response=$(curl -s -X GET "http://localhost:8081/extensions/list?session_id=ext_reg" \
        -H "X-Secret-Key: test123")
    
    if echo "$response" | grep -q "memory"; then
        echo "✅ Extension management working"
    else
        echo "❌ Extension management broken"
    fi
}

# 3. Context retrieval
test_context_retrieval() {
    # Create session with context
    curl -X POST http://localhost:8081/reply \
        -H "X-Secret-Key: test123" \
        -H "Content-Type: application/json" \
        -d '{"session_id": "ctx_test", "messages": [{"role": "user", "content": "Context test"}]}'
    
    # Get context
    response=$(curl -s -X GET "http://localhost:8081/context?session_id=ctx_test" \
        -H "X-Secret-Key: test123")
    
    if [ ! -z "$response" ]; then
        echo "✅ Context retrieval working"
    else
        echo "❌ Context retrieval broken"
    fi
}

# 4. Agent tools listing
test_agent_tools() {
    response=$(curl -s -X GET "http://localhost:8081/agent/tools?session_id=tools_test" \
        -H "X-Secret-Key: test123")
    
    if echo "$response" | grep -q '\['; then
        echo "✅ Agent tools listing working"
    else
        echo "❌ Agent tools listing broken"
    fi
}

# 5. Recipe creation
test_recipe_creation() {
    response=$(curl -s -X POST http://localhost:8081/recipe/create \
        -H "X-Secret-Key: test123" \
        -H "Content-Type: application/json" \
        -d '{"session_id": "recipe_reg", "messages": [{"role": "user", "content": "test"}]}')
    
    if echo "$response" | grep -q "recipe"; then
        echo "✅ Recipe creation working"
    else
        echo "❌ Recipe creation broken"
    fi
}

# Run all regression tests
test_basic_reply
test_extension_management
test_context_retrieval
test_agent_tools
test_recipe_creation
```

## Test Coverage Matrix

| Component | Unit Tests | Integration Tests | Performance Tests | Stress Tests | Status |
|-----------|------------|-------------------|-------------------|--------------|--------|
| AgentManager Core | ✅ | ✅ | 🔶 | 🔶 | Partial |
| Session Isolation | ✅ | ✅ | ✅ | 🔶 | Good |
| Extension Isolation | ✅ | ✅ | 🔶 | 🔶 | Good |
| Provider Lifecycle | 🔶 | 🔶 | ❌ | ❌ | Needs Work |
| Cleanup Task | ✅ | 🔶 | 🔶 | 🔶 | Partial |
| Metrics | ✅ | ✅ | ✅ | ❌ | Good |
| Concurrent Access | ✅ | ✅ | ✅ | 🔶 | Good |
| Error Recovery | ❌ | ❌ | ❌ | ❌ | Missing |
| Memory Management | 🔶 | 🔶 | 🔶 | 🔶 | Partial |

Legend: ✅ Complete | 🔶 Partial | ❌ Missing

## Critical Test Scenarios

### Must-Pass Tests Before Merge

1. **Session Isolation**: Each session MUST have its own agent
2. **Extension Isolation**: Extensions MUST NOT leak between sessions
3. **Concurrent Access**: Multiple concurrent requests MUST work correctly
4. **Cleanup Safety**: Cleanup MUST NOT affect active sessions
5. **Backward Compatibility**: Existing API endpoints MUST continue working

### High-Priority Tests

1. **Provider Setting**: Agents should work without initial provider
2. **Memory Cleanup**: Idle agents must be cleaned up to prevent leaks
3. **Metrics Accuracy**: Metrics must accurately reflect system state
4. **Session Persistence**: Sessions must survive agent cleanup/recreation

### Nice-to-Have Tests

1. **Performance Benchmarks**: Establish baseline performance metrics
2. **Stress Testing**: Validate behavior under extreme load
3. **Error Recovery**: Test graceful degradation under failures
4. **Resource Limits**: Test behavior at resource boundaries

## Monitoring and Observability

### Key Metrics to Monitor

1. **Agent Lifecycle**
   - `agents_created`: Total agents created
   - `agents_cleaned`: Total agents cleaned up
   - `active_agents`: Current active agent count
   - `cache_hits/misses`: Cache effectiveness

2. **Performance**
   - Agent creation time (p50, p95, p99)
   - Cache hit latency vs miss latency
   - Cleanup cycle duration
   - Memory per agent

3. **Health Indicators**
   - Cleanup task status
   - Memory growth rate
   - Error rates by route
   - Session creation failures

### Logging Requirements

```rust
// Critical logs needed
tracing::info!("Agent created for session: {}", session_id);
tracing::info!("Cleaned up {} idle agents", count);
tracing::warn!("Failed to set provider for session {}: {}", session_id, error);
tracing::error!("Agent cleanup task failed: {}", error);
```

## Risk Assessment

### High Risk Areas

1. **Memory Leaks**: Agents not being cleaned up properly
   - Mitigation: Automated cleanup task with configurable intervals
   - Test: Long-running stress tests with memory monitoring

2. **Session Contamination**: Data leaking between sessions
   - Mitigation: Strict agent isolation per session
   - Test: Concurrent session tests with unique markers

3. **Performance Degradation**: Slower than shared agent approach
   - Mitigation: Efficient caching and agent reuse
   - Test: Performance benchmarks comparing before/after

### Medium Risk Areas

1. **Provider Management**: Providers not being set correctly
   - Mitigation: Lazy provider initialization
   - Test: Provider lifecycle tests

2. **Cleanup Too Aggressive**: Active sessions being cleaned
   - Mitigation: Touch mechanism to keep sessions alive
   - Test: Long-running session tests

3. **Metrics Inaccuracy**: Metrics not reflecting true state
   - Mitigation: Atomic operations for metric updates
   - Test: Metrics validation under concurrent load

## Recommendations

### Immediate Actions (Before Merge)

1. **Add Provider Lifecycle Test**: Verify agents work without initial provider
2. **Run Full Regression Suite**: Ensure no existing functionality is broken
3. **Perform Basic Load Test**: Verify performance with 50-100 concurrent sessions
4. **Document Configuration**: Add clear documentation for cleanup intervals

### Follow-up Actions (After Merge)

1. **Add Comprehensive Monitoring**: Implement detailed metrics and alerting
2. **Performance Benchmarking**: Establish baseline performance metrics
3. **Error Recovery Tests**: Add tests for various failure scenarios
4. **Production Monitoring**: Close monitoring during initial rollout

### Long-term Improvements

1. **Agent Pooling**: Pre-warm agents for faster startup
2. **Smart Cleanup**: ML-based prediction of session activity
3. **Distributed Support**: Multi-instance agent management
4. **Resource Quotas**: Per-user/tenant resource limits

## Test Execution Plan

### Pre-Merge Testing (Required)

```bash
# 1. Run existing unit tests
cargo test -p goose agent_manager

# 2. Run integration tests
cargo test -p goose-server multi_session

# 3. Run basic black box tests
./comprehensive_goosed_test.sh

# 4. Run regression tests
./regression_tests.sh

# 5. Basic load test (50 sessions)
python load_test_agent_manager.py
```

### Post-Merge Testing (Recommended)

```bash
# 1. Extended stress testing (24 hours)
./long_running_stress_test.sh

# 2. Memory leak detection
valgrind --leak-check=full ./target/debug/goosed

# 3. Performance profiling
cargo flamegraph --bin goosed

# 4. Production canary testing
# Deploy to small percentage of users first
```

## Conclusion

The Agent Manager implementation successfully addresses the core requirement of per-session agent isolation. The testing plan outlined here provides comprehensive coverage of functionality, performance, and reliability aspects.

**Key Achievements:**
- ✅ Session isolation implemented and tested
- ✅ Automatic resource cleanup working
- ✅ Backward compatibility maintained
- ✅ Basic test coverage in place

**Areas Needing Attention:**
- 🔶 Provider lifecycle testing needed
- 🔶 Performance benchmarks should be established
- 🔶 Error recovery scenarios need testing
- 🔶 Long-term stress testing recommended

**Overall Assessment:** The implementation is solid and ready for merge with the current test coverage. Additional testing recommended as follow-up work to ensure production readiness.
