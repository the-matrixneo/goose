#!/usr/bin/env python3
"""
Test script to verify session isolation in the new Goose app.
This script tests that agent state is properly reset between sessions.
"""

import requests
import json
import time
import sys

# Configuration
GOOSE_SERVER_URL = "http://localhost:8000"
SECRET_KEY = "your-secret-key"  # Replace with actual secret key

def send_chat_request(session_id, message, working_dir="/tmp"):
    """Send a chat request to the Goose server"""
    url = f"{GOOSE_SERVER_URL}/reply"
    headers = {
        "Content-Type": "application/json",
        "X-Secret-Key": SECRET_KEY
    }
    
    payload = {
        "messages": [
            {
                "role": "user",
                "created": int(time.time()),
                "content": [{"type": "text", "text": message}]
            }
        ],
        "session_id": session_id,
        "session_working_dir": working_dir
    }
    
    print(f"Sending request to session {session_id}: {message}")
    
    try:
        response = requests.post(url, headers=headers, json=payload, stream=True)
        response.raise_for_status()
        
        # Process SSE stream
        messages = []
        for line in response.iter_lines():
            if line:
                line = line.decode('utf-8')
                if line.startswith('data: '):
                    try:
                        event_data = json.loads(line[6:])  # Remove 'data: ' prefix
                        if event_data.get('type') == 'Message':
                            message = event_data.get('message', {})
                            if message.get('role') == 'assistant':
                                content = message.get('content', [])
                                for c in content:
                                    if c.get('type') == 'text':
                                        messages.append(c.get('text', ''))
                        elif event_data.get('type') == 'Finish':
                            break
                        elif event_data.get('type') == 'Error':
                            print(f"Error: {event_data.get('error')}")
                            return None
                    except json.JSONDecodeError:
                        continue
        
        return ' '.join(messages).strip()
        
    except requests.exceptions.RequestException as e:
        print(f"Request failed: {e}")
        return None

def test_session_isolation():
    """Test that sessions are properly isolated"""
    print("Testing session isolation...")
    
    # Test 1: Different sessions should not share state
    session1_id = "test_session_1"
    session2_id = "test_session_2"
    
    # Send a message to session 1 that might create some agent state
    response1 = send_chat_request(
        session1_id, 
        "Remember that my favorite color is blue. What's my favorite color?"
    )
    
    if response1:
        print(f"Session 1 response: {response1}")
    else:
        print("Failed to get response from session 1")
        return False
    
    time.sleep(1)  # Brief pause
    
    # Send a message to session 2 asking about the same thing
    response2 = send_chat_request(
        session2_id,
        "What's my favorite color?"
    )
    
    if response2:
        print(f"Session 2 response: {response2}")
    else:
        print("Failed to get response from session 2")
        return False
    
    # Session 2 should not know about the favorite color from session 1
    if "blue" in response2.lower():
        print("❌ FAILED: Session 2 remembered information from session 1")
        return False
    else:
        print("✅ PASSED: Session 2 did not remember information from session 1")
    
    # Test 2: Switch back to session 1 - it should remember its own context
    response1_again = send_chat_request(
        session1_id,
        "What's my favorite color again?"
    )
    
    if response1_again:
        print(f"Session 1 (second time) response: {response1_again}")
        
        if "blue" in response1_again.lower():
            print("✅ PASSED: Session 1 remembered its own context")
            return True
        else:
            print("❌ FAILED: Session 1 forgot its own context")
            return False
    else:
        print("Failed to get second response from session 1")
        return False

def main():
    """Main test function"""
    print("Goose Session Isolation Test")
    print("=" * 40)
    
    # Check if server is running
    try:
        response = requests.get(f"{GOOSE_SERVER_URL}/health", timeout=5)
        print("✅ Goose server is running")
    except requests.exceptions.RequestException:
        print("❌ Goose server is not running or not accessible")
        print(f"Please start the Goose server at {GOOSE_SERVER_URL}")
        sys.exit(1)
    
    # Run the isolation test
    if test_session_isolation():
        print("\n🎉 All session isolation tests passed!")
        sys.exit(0)
    else:
        print("\n💥 Session isolation tests failed!")
        sys.exit(1)

if __name__ == "__main__":
    main()
