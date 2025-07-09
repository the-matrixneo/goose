import React, { createContext, useContext, ReactNode } from 'react';
import { ChatType } from '../components/BaseChat';
import { generateSessionId } from '../sessions';

interface ChatContextType {
  chat: ChatType;
  setChat: (chat: ChatType) => void;
  resetChat: () => void;
  hasActiveSession: boolean;
}

const ChatContext = createContext<ChatContextType | undefined>(undefined);

interface ChatProviderProps {
  children: ReactNode;
  chat: ChatType;
  setChat: (chat: ChatType) => void;
}

export const ChatProvider: React.FC<ChatProviderProps> = ({ children, chat, setChat }) => {
  const resetChat = () => {
    const newSessionId = generateSessionId();
    setChat({
      id: newSessionId,
      title: 'New Chat',
      messages: [],
      messageHistoryIndex: 0,
    });
  };

  const hasActiveSession = chat.messages.length > 0;

  const value: ChatContextType = {
    chat,
    setChat,
    resetChat,
    hasActiveSession,
  };

  return <ChatContext.Provider value={value}>{children}</ChatContext.Provider>;
};

export const useChatContext = (): ChatContextType => {
  const context = useContext(ChatContext);
  if (context === undefined) {
    throw new Error('useChatContext must be used within a ChatProvider');
  }
  return context;
};
