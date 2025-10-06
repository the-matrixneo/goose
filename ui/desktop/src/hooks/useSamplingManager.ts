import { useState, useEffect, useCallback, useRef } from 'react';
import { SamplingConfirmationRequest, SamplingConfirmationResponse } from '../api/types.gen';

export function useSamplingManager() {
  const [pendingRequests, setPendingRequests] = useState<SamplingConfirmationRequest[]>([]);
  const [currentRequest, setCurrentRequest] = useState<SamplingConfirmationRequest | null>(null);
  const [isModalOpen, setIsModalOpen] = useState(false);
  const [_eventSource, setEventSource] = useState<EventSource | null>(null);
  const currentRequestRef = useRef<SamplingConfirmationRequest | null>(null);

  // Update ref when currentRequest changes
  useEffect(() => {
    currentRequestRef.current = currentRequest;
  }, [currentRequest]);

  // Subscribe to sampling requests via SSE
  useEffect(() => {
    // Get the base URL from the current location
    const protocol = window.location.protocol;
    const host = window.location.hostname;
    const port = window.location.port || (protocol === 'https:' ? '443' : '80');
    const baseUrl = `${protocol}//${host}:${port}`;
    
     
    const sse = new EventSource(`${baseUrl}/api/sampling/stream`);
    
    sse.addEventListener('sampling_request', (event: MessageEvent) => {
      try {
        const request: SamplingConfirmationRequest = JSON.parse(event.data);
        
        setPendingRequests(prev => {
          const newRequests = [...prev, request];
          
          // If no current request, show this one immediately
          if (!currentRequestRef.current) {
            setCurrentRequest(request);
            setIsModalOpen(true);
          }
          
          return newRequests;
        });
      } catch (error) {
        console.error('Failed to parse sampling request:', error);
      }
    });

    sse.addEventListener('error', (error: Event) => {
      console.error('SSE connection error:', error);
      // Optionally implement reconnection logic here
    });

    setEventSource(sse);

    return () => {
      sse.close();
    };
  }, []); // Empty dependency array - only run once on mount

  const sendResponse = async (response: SamplingConfirmationResponse) => {
    try {
      const protocol = window.location.protocol;
      const host = window.location.hostname;
      const port = window.location.port || (protocol === 'https:' ? '443' : '80');
      const baseUrl = `${protocol}//${host}:${port}`;
      
      const res = await fetch(`${baseUrl}/api/sampling/respond`, {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
        },
        body: JSON.stringify(response),
      });

      if (!res.ok) {
        throw new Error(`Failed to send response: ${res.statusText}`);
      }
    } catch (error) {
      console.error('Failed to send sampling response:', error);
      throw error;
    }
  };

  const processNextRequest = useCallback(() => {
    setPendingRequests(prev => {
      // Remove the current request from pending
      const remaining = prev.filter(r => r.request_id !== currentRequest?.request_id);
      
      // Get the next request if available
      const next = remaining[0];
      
      if (next) {
        setCurrentRequest(next);
        setIsModalOpen(true);
      } else {
        setCurrentRequest(null);
        setIsModalOpen(false);
      }
      
      return remaining;
    });
  }, [currentRequest]);

  const handleApprove = useCallback(async (responseContent?: string) => {
    if (!currentRequest) return;

    try {
      await sendResponse({
        request_id: currentRequest.request_id,
        approved: true,
        response_content: responseContent,
      });

      processNextRequest();
    } catch (error) {
      console.error('Failed to approve sampling request:', error);
      // Optionally show error to user
    }
  }, [currentRequest, processNextRequest]);

  const handleDeny = useCallback(async () => {
    if (!currentRequest) return;

    try {
      await sendResponse({
        request_id: currentRequest.request_id,
        approved: false,
        response_content: undefined,
      });

      processNextRequest();
    } catch (error) {
      console.error('Failed to deny sampling request:', error);
      // Optionally show error to user
    }
  }, [currentRequest, processNextRequest]);

  // Handle keyboard shortcuts
  useEffect(() => {
    const handleKeyDown = (event: KeyboardEvent) => {
      if (!isModalOpen) return;

      if (event.key === 'Enter' && !event.shiftKey) {
        event.preventDefault();
        handleApprove();
      } else if (event.key === 'Escape') {
        event.preventDefault();
        handleDeny();
      }
    };

    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [isModalOpen, handleApprove, handleDeny]);

  return {
    currentRequest,
    isModalOpen,
    pendingCount: pendingRequests.length,
    handleApprove,
    handleDeny,
  };
}
