import { useEffect, useState, useRef } from 'react';
import { useLocation } from 'react-router-dom';
import { View, ViewOptions } from '../utils/navigationUtils';
import BaseChat from './BaseChat';
import { useRecipeManager } from '../hooks/useRecipeManager';
import { useIsMobile } from '../hooks/use-mobile';
import { useSidebar } from './ui/sidebar';
import { AgentState, useAgent } from '../hooks/useAgent';
import 'react-toastify/dist/ReactToastify.css';
import { cn } from '../utils';

import { ChatType } from '../types/chat';
import { Recipe } from '../recipe';
import { SessionDetails } from '../sessions';

export default function Pair({
  chat,
  setChat,
  setView,
  setIsGoosehintsModalOpen,
  setFatalError,
  setAgentWaitingMessage,
}: {
  chat: ChatType;
  setChat: (chat: ChatType) => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  setIsGoosehintsModalOpen: (isOpen: boolean) => void;
  setFatalError: (value: ((prevState: string | null) => string | null) | string | null) => void;
  setAgentWaitingMessage: (msg: string) => void;
}) {
  const location = useLocation();
  const isMobile = useIsMobile();
  const { state: sidebarState } = useSidebar();
  const [hasProcessedInitialInput, setHasProcessedInitialInput] = useState(false);
  const [shouldAutoSubmit, setShouldAutoSubmit] = useState(false);
  const [initialMessage, setInitialMessage] = useState<string | null>(null);
  const [isTransitioningFromHub, setIsTransitioningFromHub] = useState(false);
  const chatRef = useRef(chat);

  useEffect(() => {
    chatRef.current = chat;
  }, [chat]);

  const { agentState, initializeAgentIfNeeded } = useAgent(setChat);

  const { initialPrompt: recipeInitialPrompt } = useRecipeManager(chat, location.state);

  useEffect(() => {
    const initializeFromState = async () => {
      const appConfig = window.appConfig?.get('recipe');
      const resumedSession = location.state?.resumedSession as SessionDetails | undefined;
      const recipeConfig = location.state?.recipeConfig as Recipe | undefined;
      const resetChat = location.state?.resetChat as boolean | undefined;
      const messageFromHub = location.state?.initialMessage;
      let shouldClearState = false;
      try {
        await initializeAgentIfNeeded({
          recipeConfig: recipeConfig || (appConfig as Recipe) || null,
          resumedSession: resumedSession,
          setAgentWaitingMessage,
          initialMessage: messageFromHub || null,
          resetChat,
        });
      } catch (error) {
        setFatalError(`Agent init failure: ${error instanceof Error ? error.message : '' + error}`);
      }

      if (messageFromHub) {
        setIsTransitioningFromHub(true);
        if (messageFromHub !== initialMessage) {
          setHasProcessedInitialInput(false);
        }
        if (!hasProcessedInitialInput) {
          setHasProcessedInitialInput(true);
          setInitialMessage(messageFromHub);
          setShouldAutoSubmit(true);
        }
        shouldClearState = true;
      }

      if (shouldClearState && location.state) {
        window.history.replaceState({}, document.title);
      }
    };

    initializeFromState();
  }, [
    location.state,
    hasProcessedInitialInput,
    initialMessage,
    initializeAgentIfNeeded,
    setChat,
    setFatalError,
    setAgentWaitingMessage,
    setView,
  ]);

  useEffect(() => {
    if (agentState === AgentState.NO_PROVIDER) {
      setView('welcome');
      return;
    }
    if (shouldAutoSubmit && initialMessage && agentState === AgentState.INITIALIZED) {
      const timer = setTimeout(() => {
        const textarea = document.querySelector(
          'textarea[data-testid="chat-input"]'
        ) as HTMLTextAreaElement;
        const form = textarea?.closest('form');

        if (textarea && form) {
          textarea.value = initialMessage;
          // eslint-disable-next-line no-undef
          textarea.dispatchEvent(new Event('input', { bubbles: true }));
          textarea.focus();

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
      }, 500);

      return () => clearTimeout(timer);
    }

    return undefined;
  }, [shouldAutoSubmit, initialMessage, agentState, setView]);

  if (agentState == AgentState.NO_PROVIDER) {
    setView('welcome');
    return;
  }

  // Custom message submit handler
  const handleMessageSubmit = (message: string) => {
    setShouldAutoSubmit(false);
    setIsTransitioningFromHub(false);
    console.log('Message submitted:', message);
  };

  const handleMessageStreamFinish = () => {};

  const initialValue = initialMessage || recipeInitialPrompt || undefined;

  const customChatInputProps = {
    initialValue,
  };

  return (
    <BaseChat
      chat={chat}
      autoSubmit={shouldAutoSubmit}
      setChat={setChat}
      setView={setView}
      setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
      enableLocalStorage={true} // Enable local storage for Pair mode
      onMessageSubmit={handleMessageSubmit}
      onMessageStreamFinish={handleMessageStreamFinish}
      customChatInputProps={customChatInputProps}
      contentClassName={cn('pr-1 pb-10', (isMobile || sidebarState === 'collapsed') && 'pt-11')} // Use dynamic content class with mobile margin and sidebar state
      showPopularTopics={!isTransitioningFromHub} // Don't show popular topics while transitioning from Hub
      suppressEmptyState={isTransitioningFromHub} // Suppress all empty state content while transitioning from Hub
    />
  );
}
