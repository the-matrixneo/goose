import { useState, useCallback, useRef } from 'react';
import { ChatState } from '../types/chatState';
import { Message } from '../api';

const TextDecoder = globalThis.TextDecoder;

interface UseChatStreamProps {
  sessionId: string;
  messages: Message[];
  setMessages: (messages: Message[]) => void;
  onStreamFinish?: () => void;
}

export function useChatStream({
  sessionId,
  messages,
  setMessages,
  onStreamFinish,
}: UseChatStreamProps) {
  const [chatState, setChatState] = useState<ChatState>(ChatState.Idle);
  const abortControllerRef = useRef<AbortController | null>(null);

  const handleSubmit = useCallback(
    async (userMessage: string) => {
      const newMessage: Message = {
        role: 'user',
        content: [{ type: 'text', text: userMessage }],
        created: Date.now(),
      };

      const updatedMessages = [...messages, newMessage];
      setMessages(updatedMessages);
      setChatState(ChatState.Streaming);

      abortControllerRef.current = new AbortController();

      try {
        const response = await fetch('/reply', {
          method: 'POST',
          headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({
            session_id: sessionId,
            messages: updatedMessages.map((m) => ({
              role: m.role,
              content: m.content,
            })),
          }),
          signal: abortControllerRef.current.signal,
        });

        if (!response.ok) throw new Error(`HTTP ${response.status}`);
        if (!response.body) throw new Error('No response body');

        const reader = response.body.getReader();
        const decoder = new TextDecoder();

        while (true) {
          const { done, value } = await reader.read();
          if (done) break;

          const chunk = decoder.decode(value);
          const lines = chunk.split('\n');

          for (const line of lines) {
            if (!line.startsWith('data: ')) continue;

            const data = line.slice(6);
            if (data === '[DONE]') continue;

            try {
              const event = JSON.parse(data);

              if (event.message) {
                const msg = event.message as Message;
                setMessages([...updatedMessages, msg]);
              }

              if (event.error) {
                console.error('Stream error:', event.error);
                setChatState(ChatState.Idle);
                return;
              }

              if (event.finish) {
                setChatState(ChatState.Idle);
                onStreamFinish?.();
                return;
              }
            } catch (e) {
              console.error('Failed to parse SSE:', e);
            }
          }
        }
      } catch (error: any) {
        if (error.name !== 'AbortError') {
          console.error('Stream error:', error);
        }
        setChatState(ChatState.Idle);
      }
    },
    [sessionId, messages, setMessages, onStreamFinish]
  );

  const stopStreaming = useCallback(() => {
    abortControllerRef.current?.abort();
    setChatState(ChatState.Idle);
  }, []);

  return {
    chatState,
    handleSubmit,
    stopStreaming,
  };
}
