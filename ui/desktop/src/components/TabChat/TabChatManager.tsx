import React, { useState, useEffect } from 'react';
import { generateSessionId } from '../../sessions';
import { ChatType } from '../../types/chat';
import TabBar, { ChatTab } from './TabBar';
import { DEFAULT_CHAT_TITLE } from '../../contexts/ChatContext';

interface TabChatManagerProps {
  children: (props: {
    activeChat: ChatType;
    setActiveChat: (chat: ChatType) => void;
  }) => React.ReactNode;
  initialChat: ChatType;
  setChat: (chat: ChatType) => void;
}

/**
 * TabChatManager Component
 * 
 * This component manages multiple chat sessions within tabs.
 * It handles:
 * - Creating new chat tabs
 * - Switching between tabs
 * - Closing tabs
 * - Updating tab titles based on session descriptions
 */
const TabChatManager: React.FC<TabChatManagerProps> = ({
  children,
  initialChat,
  setChat,
}) => {
  // State for all chat tabs
  const [chatTabs, setChatTabs] = useState<ChatType[]>([initialChat]);
  
  // State for active tab ID
  const [activeTabId, setActiveTabId] = useState<string>(initialChat.id);
  
  // Get the active chat based on the active tab ID
  const activeChat = chatTabs.find(chat => chat.id === activeTabId) || chatTabs[0];
  
  // Update the parent component's chat state when active chat changes
  useEffect(() => {
    setChat(activeChat);
  }, [activeChat, setChat]);
  
  // Function to create a new chat tab
  const handleNewTab = () => {
    const newChat: ChatType = {
      id: generateSessionId(),
      title: DEFAULT_CHAT_TITLE,
      messages: [],
      messageHistoryIndex: 0,
      recipeConfig: null,
    };
    
    setChatTabs(prev => [...prev, newChat]);
    setActiveTabId(newChat.id);
  };
  
  // Function to close a tab
  const handleCloseTab = (tabId: string) => {
    // Don't close if it's the only tab
    if (chatTabs.length <= 1) return;
    
    // If closing the active tab, switch to another tab
    if (tabId === activeTabId) {
      const tabIndex = chatTabs.findIndex(chat => chat.id === tabId);
      const newActiveIndex = tabIndex === 0 ? 1 : tabIndex - 1;
      setActiveTabId(chatTabs[newActiveIndex].id);
    }
    
    // Remove the tab
    setChatTabs(prev => prev.filter(chat => chat.id !== tabId));
  };
  
  // Function to update the active chat
  const setActiveChat = (updatedChat: ChatType) => {
    setChatTabs(prev => 
      prev.map(chat => chat.id === activeTabId ? updatedChat : chat)
    );
  };
  
  // Convert chat tabs to the format expected by TabBar
  const tabBarTabs: ChatTab[] = chatTabs.map(chat => ({
    id: chat.id,
    title: chat.title || `Chat ${chatTabs.indexOf(chat) + 1}`,
    isNewChat: chat.messages && chat.messages.length === 0
  }));

  return (
    <div className="flex flex-col h-full">
      <div className="flex-shrink-0">
        <TabBar
          tabs={tabBarTabs}
          activeTabId={activeTabId}
          onTabSelect={setActiveTabId}
          onTabClose={handleCloseTab}
          onNewTab={handleNewTab}
        />
      </div>
      
      <div className="flex-grow overflow-hidden">
        {children({
          activeChat,
          setActiveChat,
        })}
      </div>
    </div>
  );
};

export default TabChatManager;
