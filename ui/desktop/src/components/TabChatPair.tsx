import React from 'react';
import { useLocation } from 'react-router-dom';
import { type View, ViewOptions } from '../App';
import BaseChat from './BaseChat';
import { ChatType } from '../types/chat';

// Simple implementation without TabChatManager
export default function TabChatPair({
  chat,
  setChat,
  setView,
  setIsGoosehintsModalOpen,
}: {
  chat: ChatType;
  setChat: (chat: ChatType) => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  setIsGoosehintsModalOpen: (isOpen: boolean) => void;
}) {
  // Ensure chat has messages array
  const safeChat = React.useMemo(() => ({
    ...chat,
    messages: chat.messages || []
  }), [chat]);
  
  const location = useLocation();

  // Handle recipe loading from recipes view - reset chat if needed
  React.useEffect(() => {
    if (location.state?.resetChat && location.state?.recipeConfig) {
      // Reset the chat to start fresh with the recipe
      const newChat = {
        id: chat.id, // Keep the same ID to maintain the session
        title: location.state.recipeConfig.title || 'Recipe Chat',
        messages: [], // Clear messages to start fresh
        messageHistoryIndex: 0,
        recipeConfig: location.state.recipeConfig, // Set the recipe config in chat state
        recipeParameters: null, // Clear parameters for new recipe
      };
      setChat(newChat);

      // Clear the location state to prevent re-processing
      window.history.replaceState({}, '', '/pair');
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [location.state, chat.id]);

  return (
    <div className="relative z-10 flex justify-center h-full bg-transparent">
      <div className="w-full max-w-[1000px] h-full bg-transparent">
        <BaseChat
          chat={safeChat}
          setChat={setChat}
          setView={setView}
          setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
          enableLocalStorage={true}
          customMainLayoutProps={{
            backgroundColor: 'transparent',
            style: { 
              backgroundColor: 'transparent',
              background: 'transparent'
            }
          }}
        />
      </div>
    </div>
  );
}
