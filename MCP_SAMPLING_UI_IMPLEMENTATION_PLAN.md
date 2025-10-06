# MCP Sampling UI Implementation Plan

## Overview
This plan details the implementation of a modal UI in the Electron desktop app to handle MCP sampling requests. When an MCP server needs user confirmation for a sampling request, the UI will display a modal allowing the user to approve or deny the request.

## Architecture Overview

### Flow Diagram
```
MCP Server → Sampling Request → Rust Backend → IPC → Electron Main → Renderer → Modal UI
                                       ↑                                           ↓
                                       └──── User Response (Approve/Deny) ←────────┘
```

### Key Components
1. **Rust Backend**: Handles MCP sampling requests and waits for UI response
2. **IPC Bridge**: Communication between Rust server and Electron app
3. **Modal UI**: React component for user confirmation
4. **State Management**: Handles pending requests and responses

## Implementation Steps

### Phase 1: Backend Integration

#### 1.1 Update OpenAPI Schema (crates/goose-server/src/openapi.rs)
- Add new types for sampling requests and responses
- Ensure types are properly exported to frontend

```rust
// Add to openapi.rs schemas
SamplingRequest,
SamplingResponse,
SamplingConfirmationRequest,
```

#### 1.2 Create Sampling Route Handler (crates/goose-server/src/routes/sampling.rs)
```rust
// New file: crates/goose-server/src/routes/sampling.rs
use axum::{Json, response::IntoResponse};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SamplingConfirmationRequest {
    pub request_id: String,
    pub extension_name: String,
    pub messages: Vec<Message>,
    pub model_preferences: Option<ModelPreferences>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SamplingConfirmationResponse {
    pub request_id: String,
    pub approved: bool,
    pub response_content: Option<String>,
}

pub async fn handle_sampling_confirmation(
    State(state): State<AppState>,
    Json(request): Json<SamplingConfirmationRequest>,
) -> impl IntoResponse {
    // Send request to frontend via WebSocket or SSE
    // Wait for user response
    // Return response to MCP client
}
```

#### 1.3 Update Agent to Handle Sampling (crates/goose/src/agents/agent.rs)
- Add sampling handler implementation
- Integrate with existing permission system
- Handle request queuing and response routing

```rust
impl SamplingHandler for AgentSamplingHandler {
    async fn handle_create_message(
        &self,
        params: CreateMessageRequestParam,
        extension_name: String,
    ) -> Result<CreateMessageResult, ServiceError> {
        // Send to UI for confirmation
        // Wait for response
        // Return result
    }
}
```

### Phase 2: Frontend Modal Implementation

#### 2.1 Create Sampling Modal Component (ui/desktop/src/components/ui/SamplingModal.tsx)
```typescript
import React from 'react';
import { BaseModal } from './BaseModal';
import { Button } from './button';
import MarkdownContent from '../MarkdownContent';

interface SamplingModalProps {
  isOpen: boolean;
  request: SamplingRequest | null;
  onApprove: (response?: string) => void;
  onDeny: () => void;
}

export function SamplingModal({
  isOpen,
  request,
  onApprove,
  onDeny,
}: SamplingModalProps) {
  if (!request) return null;

  return (
    <BaseModal
      isOpen={isOpen}
      title="MCP Sampling Request"
      actions={
        <div className="flex justify-end gap-2 p-4">
          <Button variant="outline" onClick={onDeny}>
            Deny
          </Button>
          <Button onClick={() => onApprove()}>
            Approve
          </Button>
        </div>
      }
    >
      <div className="space-y-4">
        <div className="text-sm text-gray-600 dark:text-gray-400">
          Extension: <strong>{request.extension_name}</strong>
        </div>
        
        <div className="bg-gray-50 dark:bg-gray-900 rounded-lg p-4">
          <h4 className="font-medium mb-2">Request Details:</h4>
          <div className="space-y-2">
            {request.messages.map((msg, idx) => (
              <div key={idx} className="border-l-2 pl-3">
                <div className="font-semibold">{msg.role}:</div>
                <MarkdownContent content={msg.content} />
              </div>
            ))}
          </div>
        </div>

        {request.model_preferences && (
          <div className="text-sm">
            <strong>Model Preferences:</strong> {request.model_preferences.hints?.join(', ')}
          </div>
        )}
      </div>
    </BaseModal>
  );
}
```

#### 2.2 Create Sampling Manager Hook (ui/desktop/src/hooks/useSamplingManager.ts)
```typescript
import { useState, useEffect, useCallback } from 'react';
import { api } from '../api';

interface SamplingRequest {
  request_id: string;
  extension_name: string;
  messages: Array<{ role: string; content: string }>;
  model_preferences?: {
    hints?: string[];
  };
}

export function useSamplingManager() {
  const [pendingRequests, setPendingRequests] = useState<SamplingRequest[]>([]);
  const [currentRequest, setCurrentRequest] = useState<SamplingRequest | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);

  // Subscribe to sampling requests via WebSocket/SSE
  useEffect(() => {
    const subscription = api.subscribeSamplingRequests((request: SamplingRequest) => {
      setPendingRequests(prev => [...prev, request]);
      
      // If no current request, show this one immediately
      if (!currentRequest) {
        setCurrentRequest(request);
        setIsModalOpen(true);
      }
    });

    return () => subscription.unsubscribe();
  }, [currentRequest]);

  const handleApprove = useCallback(async (response?: string) => {
    if (!currentRequest) return;

    await api.respondToSamplingRequest({
      request_id: currentRequest.request_id,
      approved: true,
      response_content: response,
    });

    processNextRequest();
  }, [currentRequest]);

  const handleDeny = useCallback(async () => {
    if (!currentRequest) return;

    await api.respondToSamplingRequest({
      request_id: currentRequest.request_id,
      approved: false,
    });

    processNextRequest();
  }, [currentRequest]);

  const processNextRequest = () => {
    setPendingRequests(prev => {
      const [next, ...remaining] = prev.filter(r => r.request_id !== currentRequest?.request_id);
      
      if (next) {
        setCurrentRequest(next);
        setIsModalOpen(true);
      } else {
        setCurrentRequest(null);
        setIsModalOpen(false);
      }
      
      return remaining;
    });
  };

  return {
    currentRequest,
    isModalOpen,
    pendingCount: pendingRequests.length,
    handleApprove,
    handleDeny,
  };
}
```

#### 2.3 Integrate Modal into Main App (ui/desktop/src/App.tsx)
```typescript
// Add to App.tsx
import { SamplingModal } from './components/ui/SamplingModal';
import { useSamplingManager } from './hooks/useSamplingManager';

function App() {
  const {
    currentRequest,
    isModalOpen,
    pendingCount,
    handleApprove,
    handleDeny,
  } = useSamplingManager();

  // ... existing code ...

  return (
    <>
      {/* Existing app content */}
      
      {/* Sampling Modal - Always rendered but controlled by isOpen */}
      <SamplingModal
        isOpen={isModalOpen}
        request={currentRequest}
        onApprove={handleApprove}
        onDeny={handleDeny}
      />
      
      {/* Optional: Show pending count badge */}
      {pendingCount > 0 && (
        <div className="fixed bottom-4 right-4 bg-orange-500 text-white rounded-full px-3 py-1">
          {pendingCount} pending sampling requests
        </div>
      )}
    </>
  );
}
```

### Phase 3: Communication Layer

#### 3.1 WebSocket/SSE Setup for Real-time Updates
```typescript
// ui/desktop/src/api/sampling.ts
export class SamplingAPI {
  private eventSource: EventSource | null = null;
  
  subscribeSamplingRequests(callback: (request: SamplingRequest) => void) {
    const baseUrl = this.client.getConfig().baseUrl;
    this.eventSource = new EventSource(`${baseUrl}/api/sampling/stream`);
    
    this.eventSource.onmessage = (event) => {
      const request = JSON.parse(event.data);
      callback(request);
    };
    
    return {
      unsubscribe: () => {
        this.eventSource?.close();
        this.eventSource = null;
      }
    };
  }
  
  async respondToSamplingRequest(response: SamplingConfirmationResponse) {
    return this.client.post('/api/sampling/respond', response);
  }
}
```

#### 3.2 Update API Client (ui/desktop/src/api/index.ts)
```typescript
// Add sampling API to main API client
import { SamplingAPI } from './sampling';

export const api = {
  // ... existing APIs ...
  sampling: new SamplingAPI(client),
};
```

### Phase 4: State Management & Edge Cases

#### 4.1 Handle Multiple Concurrent Requests
- Queue requests and show them one at a time
- Display pending count to user
- Allow bulk approve/deny actions (future enhancement)

#### 4.2 Handle Timeout Scenarios
```typescript
// Add timeout handling
const SAMPLING_TIMEOUT = 30000; // 30 seconds

useEffect(() => {
  if (!currentRequest) return;
  
  const timer = setTimeout(() => {
    // Auto-deny after timeout
    handleDeny();
  }, SAMPLING_TIMEOUT);
  
  return () => clearTimeout(timer);
}, [currentRequest]);
```

#### 4.3 Handle Connection Loss
- Store pending requests in local state
- Retry sending responses on reconnection
- Show connection status to user

### Phase 5: Testing & Polish

#### 5.1 Unit Tests
```typescript
// ui/desktop/src/components/ui/__tests__/SamplingModal.test.tsx
describe('SamplingModal', () => {
  it('displays request information correctly');
  it('calls onApprove when approve button clicked');
  it('calls onDeny when deny button clicked');
  it('handles keyboard shortcuts (Enter to approve, Escape to deny)');
});
```

#### 5.2 Integration Tests
```rust
// crates/goose/tests/sampling_integration_test.rs
#[tokio::test]
async fn test_sampling_request_flow() {
    // Test end-to-end flow
}
```

#### 5.3 UI Polish
- Add loading states during request processing
- Add animations for modal appearance/disappearance
- Add keyboard shortcuts (Enter = Approve, Escape = Deny)
- Add sound notification for new requests
- Dark mode support

## File Structure Summary

### New Files to Create:
```
crates/goose-server/src/routes/sampling.rs
ui/desktop/src/components/ui/SamplingModal.tsx
ui/desktop/src/hooks/useSamplingManager.ts
ui/desktop/src/api/sampling.ts
ui/desktop/src/components/ui/__tests__/SamplingModal.test.tsx
```

### Files to Modify:
```
crates/goose-server/src/openapi.rs (add schemas)
crates/goose-server/src/routes/mod.rs (add sampling module)
crates/goose-server/src/main.rs (add sampling routes)
crates/goose/src/agents/agent.rs (integrate sampling handler)
crates/goose/src/agents/extension_manager.rs (pass sampling handler to MCP clients)
ui/desktop/src/App.tsx (integrate modal)
ui/desktop/src/api/index.ts (add sampling API)
ui/desktop/openapi.json (regenerated after backend changes)
```

## Implementation Order

1. **Backend First**
   - Implement Rust sampling handler
   - Add API routes
   - Generate OpenAPI schema

2. **Frontend Modal**
   - Create modal component
   - Add to App.tsx
   - Test with mock data

3. **Integration**
   - Connect WebSocket/SSE
   - Test end-to-end flow
   - Handle edge cases

4. **Polish**
   - Add tests
   - Improve UX
   - Add documentation

## Security Considerations

1. **Request Validation**
   - Sanitize all content displayed in modal
   - Validate request IDs to prevent injection
   - Rate limit sampling requests per extension

2. **User Privacy**
   - Don't log sensitive message content
   - Clear requests from memory after processing
   - Allow users to disable sampling globally

3. **Extension Trust**
   - Show extension name prominently
   - Track approval history per extension
   - Allow users to auto-approve trusted extensions

## Future Enhancements

1. **Batch Operations**
   - Allow bulk approve/deny for multiple requests
   - Group requests by extension

2. **Smart Defaults**
   - Remember user preferences per extension
   - Auto-approve based on patterns
   - Machine learning for safe auto-approval

3. **Audit Trail**
   - Log all sampling requests and responses
   - Export audit logs
   - Analytics on sampling patterns

## Success Criteria

- [ ] Modal appears when sampling request received
- [ ] User can approve or deny request
- [ ] Response is sent back to MCP server
- [ ] Multiple requests are queued properly
- [ ] Connection loss is handled gracefully
- [ ] All tests pass
- [ ] No memory leaks
- [ ] Responsive UI during request processing

## Notes

- The modal should be non-dismissible (no clicking outside to close)
- Consider adding a "Don't ask again for this extension" checkbox
- The modal should work in both light and dark themes
- Consider accessibility (screen readers, keyboard navigation)
- Add telemetry to track sampling request patterns
