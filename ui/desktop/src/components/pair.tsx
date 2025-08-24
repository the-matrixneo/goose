import { useEffect, useState } from 'react';
import { useLocation } from 'react-router-dom';
import { View, ViewOptions } from '../utils/navigationUtils';
import BaseChat from './BaseChat';
import { useRecipeManager } from '../hooks/useRecipeManager';
import { useIsMobile } from '../hooks/use-mobile';
import { useSidebar } from './ui/sidebar';
import { AgentState, InitializationContext } from '../hooks/useAgent';
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
  agentState,
  loadCurrentChat,
}: {
  chat: ChatType;
  setChat: (chat: ChatType) => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  setIsGoosehintsModalOpen: (isOpen: boolean) => void;
  setFatalError: (value: ((prevState: string | null) => string | null) | string | null) => void;
  setAgentWaitingMessage: (msg: string | null) => void;
  agentState: AgentState;
  loadCurrentChat: (context: InitializationContext) => Promise<ChatType>;
}) {
  const location = useLocation();
  const isMobile = useIsMobile();
  const { state: sidebarState } = useSidebar();
  const [hasProcessedInitialInput, setHasProcessedInitialInput] = useState(false);
  const [shouldAutoSubmit, setShouldAutoSubmit] = useState(false);
  const [initialMessage, setInitialMessage] = useState<string | null>(null);
  const [isTransitioningFromHub, setIsTransitioningFromHub] = useState(false);

  const { initialPrompt: recipeInitialPrompt } = useRecipeManager(chat, location.state);

  useEffect(() => {
    const initializeFromState = async () => {
      const appConfig = window.appConfig?.get('recipe');
      const resumedSession = location.state?.resumedSession as SessionDetails | undefined;
      console.log('resumed', resumedSession);
      const recipeConfig = location.state?.recipeConfig as Recipe | undefined;
      const resetChat = location.state?.resetChat as boolean | undefined;
      const messageFromHub = location.state?.initialMessage;
      let shouldClearState = false;
      try {
        const chat = await loadCurrentChat({
          recipeConfig: recipeConfig || (appConfig as Recipe) || null,
          resumedSession: resumedSession,
          setAgentWaitingMessage,
          initialMessage: messageFromHub || null,
          resetChat,
        });
        setChat(chat);
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
    setChat,
    setFatalError,
    setAgentWaitingMessage,
    setView,
    loadCurrentChat,
  ]);

  useEffect(() => {
    if (agentState === AgentState.NO_PROVIDER) {
      setView('welcome');
    }
  }, [agentState, setView]);

  const handleMessageSubmit = (message: string) => {
    // This is called after a message is submitted
    setShouldAutoSubmit(false);
    setIsTransitioningFromHub(false); // Clear transitioning state once message is submitted
    console.log('Message submitted:', message);
  };

  const initialValue = initialMessage || recipeInitialPrompt || undefined;

  // Custom chat input props for Pair-specific behavior
  const customChatInputProps = {
    // Pass initial message from Hub or recipe prompt
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
      customChatInputProps={customChatInputProps}
      contentClassName={cn('pr-1 pb-10', (isMobile || sidebarState === 'collapsed') && 'pt-11')} // Use dynamic content class with mobile margin and sidebar state
      showPopularTopics={!isTransitioningFromHub} // Don't show popular topics while transitioning from Hub
      suppressEmptyState={isTransitioningFromHub} // Suppress all empty state content while transitioning from Hub
    />
  );
}
