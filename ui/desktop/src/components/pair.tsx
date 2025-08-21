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
import { DEFAULT_CHAT_TITLE } from '../contexts/ChatContext';
import { Recipe } from '../recipe';
import { SessionDetails } from '../sessions';

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

      let chatToInit: ChatType | null = null;
      let shouldClearState = false;

      if (appConfig && !chatRef.current.recipeConfig) {
        const recipe = appConfig as Recipe;
        chatToInit = {
          ...chatRef.current,
          recipeConfig: recipe,
          title: recipe.title || chatRef.current.title,
          messages: [],
          messageHistoryIndex: 0,
        };
        shouldClearState = false;
      } else if (resumedSession) {
        console.log('Loading resumed session in pair view:', resumedSession.sessionId);
        chatToInit = {
          sessionId: resumedSession.sessionId,
          title: resumedSession.metadata?.description || `ID: ${resumedSession.sessionId}`,
          messages: resumedSession.messages,
          messageHistoryIndex: resumedSession.messages.length,
          recipeConfig: null,
        };
        shouldClearState = true;
      } else if (recipeConfig && resetChat) {
        console.log('Loading new recipe config in pair view:', recipeConfig.title);
        chatToInit = {
          sessionId: chatRef.current.sessionId,
          title: recipeConfig.title || 'Recipe Chat',
          messages: [],
          messageHistoryIndex: 0,
          recipeConfig: recipeConfig,
          recipeParameters: null,
        };
        shouldClearState = true;
      } else if (recipeConfig && !chatRef.current.recipeConfig) {
        chatToInit = {
          ...chatRef.current,
          recipeConfig: recipeConfig,
          title: recipeConfig.title || chatRef.current.title,
        };
        shouldClearState = true;
      } else if (resetChat) {
        chatToInit = {
          ...chatRef.current,
          recipeConfig: null,
          recipeParameters: null,
          title: DEFAULT_CHAT_TITLE,
          messages: [],
          messageHistoryIndex: 0,
        };
        shouldClearState = true;
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

      if (chatToInit) {
        setChat(chatToInit);
      }

      try {
        await initializeAgentIfNeeded({
          recipeConfig: recipeConfig || (appConfig as Recipe) || null,
          resumedChat: chatToInit || chatRef.current,
          initialMessage: messageFromHub || null,
        });
      } catch (error) {
        setFatalError(`Agent init failure: ${error instanceof Error ? error.message : '' + error}`);
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
  }, [shouldAutoSubmit, initialMessage, agentState]);

  if (agentState == AgentState.NO_PROVIDER) {
    setView('welcome');
    return;
  }

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

  const renderBeforeMessages = () => {
    return <div></div>;
  };

  if (agentState === AgentState.INITIALIZING) {
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
        enableLocalStorage={true}
        onMessageSubmit={handleMessageSubmit}
        onMessageStreamFinish={handleMessageStreamFinish}
        renderBeforeMessages={renderBeforeMessages}
        customChatInputProps={customChatInputProps}
        contentClassName={cn('pr-1 pb-10', (isMobile || sidebarState === 'collapsed') && 'pt-11')}
        showPopularTopics={!isTransitioningFromHub}
        suppressEmptyState={isTransitioningFromHub}
      />
    </>
  );
}
