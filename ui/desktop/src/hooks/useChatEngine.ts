import { useCallback, useEffect, useMemo, useState } from 'react';
import { getApiUrl } from '../config';
import { useMessageStream } from './useMessageStream';
import { LocalMessageStorage } from '../utils/localMessageStorage';
import {
  Message,
  createUserMessage,
  ToolCall,
  ToolCallResult,
  ToolRequestMessageContent,
  ToolResponseMessageContent,
  ToolConfirmationRequestMessageContent,
  getTextContent,
  TextContent,
} from '../types/message';
import { ChatType } from '../types/chat';
import { ChatState } from '../types/chatState';
import { getSession } from '../api';

// Helper function to determine if a message is a user message
const isUserMessage = (message: Message): boolean => {
  if (message.role === 'assistant') {
    return false;
  }
  return !message.content.every((c) => c.type === 'toolConfirmationRequest');
};

interface UseChatEngineProps {
  chat: ChatType;
  setChat: (chat: ChatType) => void;
  onMessageStreamFinish?: () => void;
  onMessageSent?: () => void; // Add callback for when message is sent
}

export const useChatEngine = ({
  chat,
  setChat,
  onMessageStreamFinish,
  onMessageSent,
}: UseChatEngineProps) => {
  const [lastInteractionTime, setLastInteractionTime] = useState<number>(Date.now());
  const [sessionTokenCount, setSessionTokenCount] = useState<number>(0);
  const [sessionInputTokens, setSessionInputTokens] = useState<number>(0);
  const [sessionOutputTokens, setSessionOutputTokens] = useState<number>(0);
  const [localInputTokens, setLocalInputTokens] = useState<number>(0);
  const [localOutputTokens, setLocalOutputTokens] = useState<number>(0);
  const [powerSaveTimeoutId, setPowerSaveTimeoutId] = useState<number | null>(null);

  // Track pending edited message
  const [pendingEdit, setPendingEdit] = useState<{ id: string; content: string } | null>(null);

  // Store message in global history when it's added
  const storeMessageInHistory = useCallback((message: Message) => {
    if (isUserMessage(message)) {
      const text = getTextContent(message);
      if (text) {
        LocalMessageStorage.addMessage(text);
      }
    }
  }, []);

  const stopPowerSaveBlocker = useCallback(() => {
    try {
      window.electron.stopPowerSaveBlocker();
    } catch (error) {
      console.error('Failed to stop power save blocker:', error);
    }

    // Clear timeout if it exists
    if (powerSaveTimeoutId) {
      window.clearTimeout(powerSaveTimeoutId);
      setPowerSaveTimeoutId(null);
    }
  }, [powerSaveTimeoutId]);

  const {
    messages,
    append: originalAppend,
    stop,
    chatState,
    error,
    setMessages,
    input: _input,
    setInput: _setInput,
    handleInputChange: _handleInputChange,
    updateMessageStreamBody,
    notifications,
    session,
    setError,
  } = useMessageStream({
    api: getApiUrl('/reply'),
    id: chat.sessionId,
    initialMessages: chat.messages,
    body: {
      session_id: chat.sessionId,
      session_working_dir: window.appConfig.get('GOOSE_WORKING_DIR'),
      ...(chat.recipe?.title
        ? {
            recipe_name: chat.recipe.title,
            recipe_version: chat.recipe?.version ?? 'unknown',
          }
        : {}),
    },
    onFinish: async (_message, _reason) => {
      stopPowerSaveBlocker();

      const timeSinceLastInteraction = Date.now() - lastInteractionTime;
      window.electron.logInfo('last interaction:' + lastInteractionTime);
      if (timeSinceLastInteraction > 60000) {
        // 60000ms = 1 minute
        window.electron.showNotification({
          title: 'Goose finished the task.',
          body: 'Click here to expand.',
        });
      }

      // Always emit refresh event when message stream finishes for new sessions
      // Check if this is a new session by looking at the current session ID format
      const isNewSession = chat.sessionId && chat.sessionId.match(/^\d{8}_\d{6}$/);
      if (isNewSession) {
        console.log(
          'ChatEngine: Message stream finished for new session, emitting message-stream-finished event'
        );
        // Emit event to trigger session refresh
        window.dispatchEvent(new CustomEvent('message-stream-finished'));
      }

      onMessageStreamFinish?.();
    },
    onError: (error) => {
      stopPowerSaveBlocker();

      console.log(
        'CHAT ENGINE RECEIVED ERROR FROM MESSAGE STREAM:',
        JSON.stringify(
          {
            errorMessage: error.message,
            errorName: error.name,
            isTokenLimitError: (error as Error & { isTokenLimitError?: boolean }).isTokenLimitError,
            errorStack: error.stack,
            timestamp: new Date().toISOString(),
            sessionId: chat.sessionId,
          },
          null,
          2
        )
      );
    },
  });

  // Wrap append to store messages in global history
  const append = useCallback(
    (messageOrString: Message | string) => {
      const message =
        typeof messageOrString === 'string' ? createUserMessage(messageOrString) : messageOrString;
      storeMessageInHistory(message);

      // If this is the first message in a new session, trigger a refresh immediately
      // Only trigger if we're starting a completely new session (no existing messages)
      if (messages.length === 0 && chat.messages.length === 0) {
        // Emit event to indicate a new session is being created
        window.dispatchEvent(new CustomEvent('session-created'));
      }

      return originalAppend(message);
    },
    [originalAppend, storeMessageInHistory, messages.length, chat.messages.length]
  );

  // Simple token estimation function (roughly 4 characters per token)
  const estimateTokens = (text: string): number => {
    return Math.ceil(text.length / 4);
  };

  // Calculate token counts from messages
  useEffect(() => {
    let inputTokens = 0;
    let outputTokens = 0;

    messages.forEach((message) => {
      const textContent = getTextContent(message);
      if (textContent) {
        const tokens = estimateTokens(textContent);
        if (message.role === 'user') {
          inputTokens += tokens;
        } else if (message.role === 'assistant') {
          outputTokens += tokens;
        }
      }
    });

    setLocalInputTokens(inputTokens);
    setLocalOutputTokens(outputTokens);
  }, [messages]);

  // Update chat messages when they change
  useEffect(() => {
    // @ts-expect-error - TypeScript being overly strict about the return type
    setChat((prevChat: ChatType) => ({ ...prevChat, messages }));
  }, [messages, setChat]);

  useEffect(() => {
    const fetchSessionTokens = async () => {
      try {
        const response = await getSession<true>({
          path: { session_id: chat.sessionId },
          throwOnError: true,
        });
        const sessionDetails = response.data;
        setSessionTokenCount(sessionDetails.total_tokens || 0);
        setSessionInputTokens(sessionDetails.accumulated_input_tokens || 0);
        setSessionOutputTokens(sessionDetails.accumulated_output_tokens || 0);
      } catch (err) {
        console.error('Error fetching session token count:', err);
      }
    };
    // Only fetch session tokens when chat state is idle to avoid resetting during streaming
    if (chat.sessionId && chatState === ChatState.Idle) {
      fetchSessionTokens();
    }
  }, [chat.sessionId, messages, chatState]);

  // Update token counts when session changes from the message stream
  useEffect(() => {
    console.log('Session received:', session);
    if (session) {
      setSessionTokenCount(session.total_tokens || 0);
      setSessionInputTokens(session.accumulated_input_tokens || 0);
      setSessionOutputTokens(session.accumulated_output_tokens || 0);
    }
  }, [session]);

  useEffect(() => {
    return () => {
      if (powerSaveTimeoutId) {
        window.clearTimeout(powerSaveTimeoutId);
      }
      try {
        window.electron.stopPowerSaveBlocker();
      } catch (error) {
        console.error('Failed to stop power save blocker during cleanup:', error);
      }
    };
  }, [powerSaveTimeoutId]);

  // Handle submit
  const handleSubmit = useCallback(
    (combinedTextFromInput: string, onSummaryReset?: () => void) => {
      if (combinedTextFromInput.trim()) {
        try {
          window.electron.startPowerSaveBlocker();
        } catch (error) {
          console.error('Failed to start power save blocker:', error);
        }

        setLastInteractionTime(Date.now());

        // Set a timeout to automatically stop the power save blocker after 15 minutes
        const timeoutId = window.setTimeout(
          () => {
            console.warn('Power save blocker timeout - stopping automatically after 15 minutes');
            stopPowerSaveBlocker();
          },
          15 * 60 * 1000
        );

        setPowerSaveTimeoutId(timeoutId);

        const userMessage = createUserMessage(combinedTextFromInput.trim());

        if (onSummaryReset) {
          onSummaryReset();
          window.setTimeout(() => {
            append(userMessage);
            onMessageSent?.();
          }, 150);
        } else {
          append(userMessage);
          onMessageSent?.();
        }
      } else {
        // If nothing was actually submitted (e.g. empty input and no images pasted)
        stopPowerSaveBlocker();
      }
    },
    [append, onMessageSent, stopPowerSaveBlocker]
  );

  // Handle stopping the message stream
  const onStopGoose = useCallback(() => {
    stop();
    setLastInteractionTime(Date.now());
    stopPowerSaveBlocker();

    // Handle stopping the message stream
    const lastMessage = messages[messages.length - 1];

    // Check if there are any messages before proceeding
    if (!lastMessage) {
      return;
    }

    // check if the last user message has any tool response(s)
    const isToolResponse = lastMessage.content.some(
      (content): content is ToolResponseMessageContent => content.type == 'toolResponse'
    );

    // isUserMessage also checks if the message is a toolConfirmationRequest
    // check if the last message is a real user's message
    if (lastMessage && isUserMessage(lastMessage) && !isToolResponse) {
      // Get the text content from the last message before removing it
      const textContent = lastMessage.content.find((c): c is TextContent => c.type === 'text');
      const textValue = textContent?.text || '';

      // Set the text back to the input field
      _setInput(textValue);

      // Also add to local storage history as a backup so cmd+up can retrieve it
      if (textValue.trim()) {
        LocalMessageStorage.addMessage(textValue.trim());
      }

      // Remove the last user message if it's the most recent one
      if (messages.length > 1) {
        setMessages(messages.slice(0, -1));
      } else {
        setMessages([]);
      }
    } else if (!isUserMessage(lastMessage)) {
      // the last message was an assistant message
      // check if we have any tool requests or tool confirmation requests
      const toolRequests: [string, ToolCallResult<ToolCall>][] = lastMessage.content
        .filter(
          (content): content is ToolRequestMessageContent | ToolConfirmationRequestMessageContent =>
            content.type === 'toolRequest' || content.type === 'toolConfirmationRequest'
        )
        .map((content) => {
          if (content.type === 'toolRequest') {
            return [content.id, content.toolCall];
          } else {
            // extract tool call from confirmation
            const toolCall: ToolCallResult<ToolCall> = {
              status: 'success',
              value: {
                name: content.toolName,
                arguments: content.arguments,
              },
            };
            return [content.id, toolCall];
          }
        });

      if (toolRequests.length !== 0) {
        // This means we were interrupted during a tool request
        // Create tool responses for all interrupted tool requests

        let responseMessage: Message = {
          role: 'user',
          created: Date.now(),
          content: [],
        };

        const notification = 'Interrupted by the user to make a correction';

        // generate a response saying it was interrupted for each tool request
        for (const [reqId, _] of toolRequests) {
          const toolResponse: ToolResponseMessageContent = {
            type: 'toolResponse',
            id: reqId,
            toolResult: {
              status: 'error',
              error: notification,
            },
          };

          responseMessage.content.push(toolResponse);
        }
        // Use an immutable update to add the response message to the messages array
        setMessages([...messages, responseMessage]);
      }
    }
  }, [stop, messages, _setInput, setMessages, stopPowerSaveBlocker]);

  // Since server now handles all filtering, we just use messages directly
  const filteredMessages = useMemo(() => {
    return messages;
  }, [messages]);

  // Generate command history from messages
  const commandHistory = useMemo(() => {
    return filteredMessages
      .reduce<string[]>((history, message) => {
        if (isUserMessage(message)) {
          const textContent = message.content.find((c): c is TextContent => c.type === 'text');
          const text = textContent?.text?.trim();
          if (text) {
            history.push(text);
          }
        }
        return history;
      }, [])
      .reverse();
  }, [filteredMessages]);

  // Process tool call notifications
  const toolCallNotifications = useMemo(() => {
    return notifications.reduce((map, item) => {
      const key = item.request_id;
      if (!map.has(key)) {
        map.set(key, []);
      }
      map.get(key).push(item);
      return map;
    }, new Map());
  }, [notifications]);

  // Handle message updates from the UI
  const onMessageUpdate = useCallback(
    (messageId: string, newContent: string) => {
      const messageIndex = messages.findIndex((msg) => msg.id === messageId);

      if (messageIndex !== -1) {
        // Truncate the history to the point *before* the edited message.
        const history = messages.slice(0, messageIndex);

        // Set the truncated history.
        setMessages(history);

        // Instead of setTimeout, set pendingEdit which will be handled in useEffect
        setPendingEdit({ id: messageId, content: newContent });
      }
    },
    [messages, setMessages, setPendingEdit]
  );

  // Listen for pending edit and append message after messages updated
  useEffect(() => {
    if (pendingEdit) {
      const updatedMessage = createUserMessage(pendingEdit.content);
      append(updatedMessage);
      setPendingEdit(null); // Reset after processing
    }
  }, [pendingEdit, append]);

  return {
    // Core message data
    messages,
    filteredMessages,

    // Message stream controls
    append,
    stop,
    chatState,
    error,
    setMessages,

    // Input controls
    input: _input,
    setInput: _setInput,
    handleInputChange: _handleInputChange,

    // Event handlers
    handleSubmit,
    onStopGoose,

    // Token and session data
    sessionTokenCount,
    sessionInputTokens,
    sessionOutputTokens,
    localInputTokens,
    localOutputTokens,

    // UI helpers
    commandHistory,
    toolCallNotifications,

    // Stream utilities
    updateMessageStreamBody,
    sessionMetadata: session,

    // Utilities
    isUserMessage,

    // Error management
    clearError: () => setError(undefined),

    // New functions for message editing
    onMessageUpdate,
  };
};
