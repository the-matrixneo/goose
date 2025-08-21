/**
 * Pair Component
 *
 * The Pair component represents the active conversation mode in the Goose Desktop application.
 * This is where users engage in ongoing conversations with the AI assistant after transitioning
 * from the Hub's initial welcome screen.
 *
 * Key Responsibilities:
 * - Manages active chat sessions with full message history
 * - Handles transitions from Hub with initial input processing
 * - Provides the main conversational interface for extended interactions
 * - Enables local storage persistence for conversation continuity
 * - Supports all advanced chat features like file attachments, tool usage, etc.
 * - Initializes the agent when entering active conversation mode
 *
 * Navigation Flow:
 * Hub (initial message) → Pair (active conversation) → Hub (new session)
 *
 * The Pair component is essentially a specialized wrapper around BaseChat that:
 * - Processes initial input from the Hub transition
 * - Enables conversation persistence
 * - Provides the full-featured chat experience
 * - Initializes the agent for conversation handling
 *
 * Unlike Hub, Pair assumes an active conversation state and focuses on
 * maintaining conversation flow rather than onboarding new users.
 */

import { useEffect, useState } from 'react';
import { useLocation } from 'react-router-dom';
import { View, ViewOptions } from '../utils/navigationUtils';
import BaseChat from './BaseChat';
import { useRecipeManager } from '../hooks/useRecipeManager';
import { useIsMobile } from '../hooks/use-mobile';
import { useSidebar } from './ui/sidebar';
import { useAgent } from '../hooks/useAgent';
import 'react-toastify/dist/ReactToastify.css';
import { cn } from '../utils';

import { ChatType } from '../types/chat';
import { DEFAULT_CHAT_TITLE } from '../contexts/ChatContext';

export default function Pair({
  chat,
  setChat,
  setView,
  setIsGoosehintsModalOpen,
  setFatalError,
}: {
  chat: ChatType;
  setChat: (chat: ChatType) => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  setIsGoosehintsModalOpen: (isOpen: boolean) => void;
  setFatalError: (value: ((prevState: string | null) => string | null) | string | null) => void;
}) {
  const location = useLocation();
  const isMobile = useIsMobile();
  const { state: sidebarState } = useSidebar();
  const [hasProcessedInitialInput, setHasProcessedInitialInput] = useState(false);
  const [shouldAutoSubmit, setShouldAutoSubmit] = useState(false);
  const [initialMessage, setInitialMessage] = useState<string | null>(null);
  const [isTransitioningFromHub, setIsTransitioningFromHub] = useState(false);

  // Use the shared agent hook
  const { isAgentInitialized, isInitializing, initializeAgentIfNeeded } = useAgent(setChat);

  // Get recipe configuration and parameter handling
  const { initialPrompt: recipeInitialPrompt } = useRecipeManager(chat, location.state);

  const recipeConfig = location.state?.recipeConfig || null;

  useEffect(() => {
    try {
      initializeAgentIfNeeded({
        recipeConfig,
        resumedChat: chat,
      });
    } catch (error) {
      setFatalError(
        `Agent init failure: ${error instanceof Error ? error.message : 'Unknown error'}`
      );
    }
  }, [initializeAgentIfNeeded, setFatalError, chat, recipeConfig]);

  // Handle initial chat setup when entering Pair mode
  // Handle initial message from hub page
  useEffect(() => {
    const messageFromHub = location.state?.initialMessage;
    const resetChat = location.state?.resetChat;

    // If we have a resetChat flag from Hub, clear any existing recipe config
    // This scenario occurs when a user navigates from Hub to start a new chat,
    // ensuring any previous recipe configuration is cleared for a fresh start
    if (resetChat) {
      const newChat: ChatType = {
        ...chat,
        recipeConfig: null,
        recipeParameters: null,
        title: DEFAULT_CHAT_TITLE,
        messages: [], // Clear messages for fresh start
        messageHistoryIndex: 0,
      };
      setChat(newChat);
    }

    // Reset processing state when we have a new message from hub
    if (messageFromHub) {
      // Set transitioning state to prevent showing popular topics
      setIsTransitioningFromHub(true);

      // If this is a different message than what we processed before, reset the flag
      if (messageFromHub !== initialMessage) {
        setHasProcessedInitialInput(false);
      }

      if (!hasProcessedInitialInput) {
        setHasProcessedInitialInput(true);
        setInitialMessage(messageFromHub);
        setShouldAutoSubmit(true);

        // Clear the location state to prevent re-processing
        window.history.replaceState({}, '', '/pair');
      }
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [location.state, hasProcessedInitialInput, initialMessage]);

  // Auto-submit the initial message after it's been set and component is ready
  useEffect(() => {
    if (shouldAutoSubmit && initialMessage && isAgentInitialized) {
      // Wait for the component to be fully rendered AND agent to be initialized
      const timer = setTimeout(() => {
        // Try to trigger form submission programmatically
        const textarea = document.querySelector(
          'textarea[data-testid="chat-input"]'
        ) as HTMLTextAreaElement;
        const form = textarea?.closest('form');

        if (textarea && form) {
          // Set the textarea value
          textarea.value = initialMessage;
          // eslint-disable-next-line no-undef
          textarea.dispatchEvent(new Event('input', { bubbles: true }));

          // Focus the textarea
          textarea.focus();

          // Simulate Enter key press to trigger submission
          const enterEvent = new KeyboardEvent('keydown', {
            key: 'Enter',
            code: 'Enter',
            keyCode: 13,
            which: 13,
            bubbles: true,
          });
          textarea.dispatchEvent(enterEvent);

          setShouldAutoSubmit(false);
        }
      }, 500); // Give more time for the component to fully mount

      return () => clearTimeout(timer);
    }

    // Return undefined when condition is not met
    return undefined;
  }, [shouldAutoSubmit, initialMessage, isAgentInitialized]);

  // Custom message submit handler
  const handleMessageSubmit = (message: string) => {
    // This is called after a message is submitted
    setShouldAutoSubmit(false);
    setIsTransitioningFromHub(false); // Clear transitioning state once message is submitted
    console.log('Message submitted:', message);
  };

  // Custom message stream finish handler to handle recipe auto-execution
  const handleMessageStreamFinish = () => {
    // This will be called with the proper append function from BaseChat
    // For now, we'll handle auto-execution in the BaseChat component
  };

  // Determine the initial value for the chat input
  // Priority: Hub message > Recipe prompt > empty
  const initialValue = initialMessage || recipeInitialPrompt || undefined;

  // Custom chat input props for Pair-specific behavior
  const customChatInputProps = {
    // Pass initial message from Hub or recipe prompt
    initialValue,
  };

  // Custom content before messages
  const renderBeforeMessages = () => {
    return <div>{/* Any Pair-specific content before messages can go here */}</div>;
  };

  // Show loading state while agent is initializing
  if (isInitializing) {
    return (
      <div className="flex justify-center items-center h-full">
        <div className="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-textStandard"></div>
      </div>
    );
  }

  return (
    <>
      <BaseChat
        chat={chat}
        setChat={setChat}
        setView={setView}
        setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
        enableLocalStorage={true} // Enable local storage for Pair mode
        onMessageSubmit={handleMessageSubmit}
        onMessageStreamFinish={handleMessageStreamFinish}
        renderBeforeMessages={renderBeforeMessages}
        customChatInputProps={customChatInputProps}
        contentClassName={cn('pr-1 pb-10', (isMobile || sidebarState === 'collapsed') && 'pt-11')} // Use dynamic content class with mobile margin and sidebar state
        showPopularTopics={!isTransitioningFromHub} // Don't show popular topics while transitioning from Hub
        suppressEmptyState={isTransitioningFromHub} // Suppress all empty state content while transitioning from Hub
      />
    </>
  );
}
