import { useEffect, useState } from 'react';
import { View, ViewOptions } from '../utils/navigationUtils';
import BaseChat from './BaseChat';
import { useRecipeManager } from '../hooks/useRecipeManager';
import { useIsMobile } from '../hooks/use-mobile';
import { useSidebar } from './ui/sidebar';
import { AgentState } from '../hooks/useAgent';
import 'react-toastify/dist/ReactToastify.css';
import { cn } from '../utils';

import { ChatType } from '../types/chat';
import { Recipe } from '../recipe';
import { Message } from '../types/message';

export interface PairRouteState {
  resumeSessionId?: string;
  recipeConfig?: Recipe;
  initialMessage?: string;
}

export default function Pair({
  chat,
  setChatMessages,
  setView,
  setIsGoosehintsModalOpen,
  agentState,
  routeState,
}: {
  chat: ChatType;
  setChatMessages: (messages: Message[]) => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  setIsGoosehintsModalOpen: (isOpen: boolean) => void;
  agentState: AgentState;
  routeState: PairRouteState;
}) {
  const isMobile = useIsMobile();
  const { state: sidebarState } = useSidebar();
  const [hasProcessedInitialInput, setHasProcessedInitialInput] = useState(false);
  const [shouldAutoSubmit, setShouldAutoSubmit] = useState(false);
  const [messageToSubmit, setMessageToSubmit] = useState<string | null>(null);
  const [isTransitioningFromHub, setIsTransitioningFromHub] = useState(false);
  const [recipeResetOverride, _setRecipeResetOverride] = useState(false);
  const [loadingChat, _setLoadingChat] = useState(false);

  // Followed by sending the initialMessage if we have one. This will happen
  // only once, unless we reset the chat in step one.
  useEffect(() => {
    if (
      agentState !== AgentState.INITIALIZED ||
      !routeState.initialMessage ||
      hasProcessedInitialInput
    ) {
      return;
    }

    setIsTransitioningFromHub(true);
    setHasProcessedInitialInput(true);
    setMessageToSubmit(routeState.initialMessage);
    setShouldAutoSubmit(true);
  }, [agentState, routeState.initialMessage, hasProcessedInitialInput]);

  useEffect(() => {
    if (agentState === AgentState.NO_PROVIDER) {
      setView('welcome');
    }
  }, [agentState, setView]);

  const { initialPrompt: recipeInitialPrompt } = useRecipeManager(chat, chat.recipeConfig || null);

  const handleMessageSubmit = (message: string) => {
    // Clean up any auto submit state:
    setShouldAutoSubmit(false);
    setIsTransitioningFromHub(false);
    setMessageToSubmit(null);
    console.log('Message submitted:', message);
  };

  const initialValue =
    messageToSubmit ||
    (agentState === 'initialized' && !recipeResetOverride ? recipeInitialPrompt : undefined) ||
    undefined;

  const customChatInputProps = {
    // Pass initial message from Hub or recipe prompt
    initialValue,
  };

  return (
    <BaseChat
      chat={chat}
      loadingChat={loadingChat}
      autoSubmit={shouldAutoSubmit}
      setChatMessages={setChatMessages}
      setView={setView}
      setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
      onMessageSubmit={handleMessageSubmit}
      customChatInputProps={customChatInputProps}
      contentClassName={cn('pr-1 pb-10', (isMobile || sidebarState === 'collapsed') && 'pt-11')} // Use dynamic content class with mobile margin and sidebar state
      showPopularTopics={!isTransitioningFromHub} // Don't show popular topics while transitioning from Hub
      suppressEmptyState={isTransitioningFromHub} // Suppress all empty state content while transitioning from Hub
      recipeResetOverride={recipeResetOverride}
    />
  );
}
