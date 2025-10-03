import { useEffect, useState, useRef } from 'react';
import { View, ViewOptions } from '../utils/navigationUtils';
import BaseChat from './BaseChat';
import { useRecipeManager } from '../hooks/useRecipeManager';
import { useIsMobile } from '../hooks/use-mobile';
import { useSidebar } from './ui/sidebar';
import { AgentState, InitializationContext } from '../hooks/useAgent';
import 'react-toastify/dist/ReactToastify.css';
import { cn } from '../utils';
import { Recipe } from '../api';

import { ChatType } from '../types/chat';
import { useSearchParams } from 'react-router-dom';

export interface PairRouteState {
  resumeSessionId?: string;
  initialMessage?: string;
  recipe?: Recipe;
}

interface PairProps {
  chat: ChatType;
  setChat: (chat: ChatType) => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  setIsGoosehintsModalOpen: (isOpen: boolean) => void;
  setFatalError: (value: ((prevState: string | null) => string | null) | string | null) => void;
  setAgentWaitingMessage: (msg: string | null) => void;
  agentState: AgentState;
  loadCurrentChat: (context: InitializationContext) => Promise<ChatType>;
}

export default function Pair({
  chat,
  setChat,
  setView,
  setIsGoosehintsModalOpen,
  setFatalError,
  setAgentWaitingMessage,
  agentState,
  loadCurrentChat,
  resumeSessionId,
  initialMessage,
  recipe,
}: PairProps & PairRouteState) {
  const isMobile = useIsMobile();
  const { state: sidebarState } = useSidebar();
  const [hasProcessedInitialInput, setHasProcessedInitialInput] = useState(false);
  const [shouldAutoSubmit, setShouldAutoSubmit] = useState(false);
  const [messageToSubmit, setMessageToSubmit] = useState<string | null>(null);
  const [isTransitioningFromHub, setIsTransitioningFromHub] = useState(false);
  const [loadingChat, setLoadingChat] = useState(false);
  const [_searchParams, setSearchParams] = useSearchParams();

  // Use refs to track current chat state to avoid dependency issues
  const chatRef = useRef(chat);
  const prevRecipeRef = useRef<Recipe | null>(null);
  chatRef.current = chat;

  useEffect(() => {
    const initializeFromState = async () => {
      setLoadingChat(true);

      const currentChat = chatRef.current;
      const prevRecipe = prevRecipeRef.current;

      // If we have a recipe from navigation, always create a new chat with the recipe
      // This ensures we start fresh when loading a recipe from the recipes view
      // Compare using JSON.stringify to detect any changes in the recipe object
      const recipeChanged =
        recipe && (!prevRecipe || JSON.stringify(prevRecipe) !== JSON.stringify(recipe));

      if (recipe && recipeChanged) {
        prevRecipeRef.current = recipe;

        try {
          // Load a fresh chat session with new recipe reset behavior
          const newChat = await loadCurrentChat({
            resumeSessionId: undefined,
            recipe: recipe,
            setAgentWaitingMessage,
            resetOptions: {
              resetSession: true,
              clearMessages: true,
              clearRecipeParameters: true,
              // Keep the new recipe from recipe
            },
          });

          setChat(newChat);
          setSearchParams((prev) => {
            prev.set('resumeSessionId', newChat.sessionId);
            return prev;
          });
        } catch (error) {
          setFatalError(
            `Agent init failure: ${error instanceof Error ? error.message : '' + error}`
          );
        } finally {
          setLoadingChat(false);
        }
        return;
      }

      // If we don't have a recipe from navigation but had one before, clear it
      if (!recipe && prevRecipe) {
        prevRecipeRef.current = null;
      }

      try {
        const loadedChat = await loadCurrentChat({
          resumeSessionId,
          setAgentWaitingMessage,
        });

        // If the loaded chat doesn't have a recipe config but we have one in the current chat, preserve it
        // BUT only if we're resuming the same session (not starting a new chat)
        let finalChat = loadedChat;
        if (
          !loadedChat.recipe &&
          currentChat?.recipe &&
          resumeSessionId === currentChat.sessionId
        ) {
          finalChat = {
            ...loadedChat,
            recipe: currentChat.recipe,
            recipeParameters: currentChat.recipeParameters || null,
          };
        }

        setChat(finalChat);
        setSearchParams((prev) => {
          prev.set('resumeSessionId', loadedChat.sessionId);
          return prev;
        });
      } catch (error) {
        setFatalError(`Agent init failure: ${error instanceof Error ? error.message : '' + error}`);
      } finally {
        setLoadingChat(false);
      }
    };
    initializeFromState();
  }, [
    agentState,
    setChat,
    setFatalError,
    setAgentWaitingMessage,
    loadCurrentChat,
    resumeSessionId,
    setSearchParams,
    recipe,
  ]);

  // Followed by sending the initialMessage if we have one. This will happen
  // only once, unless we reset the chat in step one.
  useEffect(() => {
    if (agentState !== AgentState.INITIALIZED || !initialMessage || hasProcessedInitialInput) {
      return;
    }

    setIsTransitioningFromHub(true);
    setHasProcessedInitialInput(true);
    setMessageToSubmit(initialMessage);
    setShouldAutoSubmit(true);
  }, [agentState, initialMessage, hasProcessedInitialInput]);

  useEffect(() => {
    if (agentState === AgentState.NO_PROVIDER) {
      setView('welcome');
    }
  }, [agentState, setView]);

  const { initialPrompt: recipeInitialPrompt } = useRecipeManager(
    chat,
    recipe || chat.recipe || null
  );

  const handleMessageSubmit = () => {
    // Clean up any auto submit state:
    setShouldAutoSubmit(false);
    setIsTransitioningFromHub(false);
    setMessageToSubmit(null);
  };

  const recipePrompt =
    agentState === 'initialized' && chat.messages.length === 0 && recipeInitialPrompt;

  const initialValue = messageToSubmit || recipePrompt || undefined;

  const customChatInputProps = {
    // Pass initial message from Hub or recipe prompt
    initialValue,
  };

  return (
    <BaseChat
      chat={chat}
      loadingChat={loadingChat}
      autoSubmit={shouldAutoSubmit}
      setChat={setChat}
      setView={setView}
      setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
      onMessageSubmit={handleMessageSubmit}
      customChatInputProps={customChatInputProps}
      contentClassName={cn('pr-1 pb-10', (isMobile || sidebarState === 'collapsed') && 'pt-11')} // Use dynamic content class with mobile margin and sidebar state
      showPopularTopics={!isTransitioningFromHub} // Don't show popular topics while transitioning from Hub
      suppressEmptyState={isTransitioningFromHub} // Suppress all empty state content while transitioning from Hub
    />
  );
}
