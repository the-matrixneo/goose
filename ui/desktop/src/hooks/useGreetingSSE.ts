import { useEffect, useState, useRef } from 'react';
import { getApiUrl } from '../config';

// Ensure TextDecoder is available in the global scope
const TextDecoder = globalThis.TextDecoder;

export function useGreetingSSE() {
  const [greeting, setGreeting] = useState<string | null>(null);
  const abortControllerRef = useRef<AbortController | null>(null);

  useEffect(() => {
    const connectToGreetingStream = async () => {
      try {
        const abortController = new AbortController();
        abortControllerRef.current = abortController;

        // Use fetch with X-Secret-Key header, just like useMessageStream
        const response = await fetch(getApiUrl('/greeting'), {
          method: 'GET',
          headers: {
            'X-Secret-Key': await window.electron.getSecretKey(),
          },
          signal: abortController.signal,
        });

        if (!response.ok) {
          console.error('Failed to connect to greeting stream:', response.statusText);
          return;
        }

        if (!response.body) {
          console.error('Response body is empty');
          return;
        }

        // Process the SSE stream
        const reader = response.body.getReader();
        const decoder = new TextDecoder();
        let buffer = '';

        while (true) {
          const { done, value } = await reader.read();
          if (done) break;

          buffer += decoder.decode(value, { stream: true });

          // Process complete SSE events
          const events = buffer.split('\n\n');
          buffer = events.pop() || '';

          for (const event of events) {
            if (event.startsWith('data: ')) {
              const data = event.slice(6); // Remove 'data: ' prefix
              setGreeting(data);
            }
          }
        }
      } catch (error) {
        if (error instanceof Error && error.name !== 'AbortError') {
          console.error('SSE connection error:', error);
        }
      }
    };

    connectToGreetingStream();

    // Cleanup on unmount
    return () => {
      if (abortControllerRef.current) {
        abortControllerRef.current.abort();
      }
    };
  }, []);

  const dismissGreeting = () => {
    setGreeting(null);
  };

  return { greeting, dismissGreeting };
}
