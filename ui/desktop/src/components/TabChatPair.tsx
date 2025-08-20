import React, { useState, useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import { type View, ViewOptions } from '../App';
import BaseChat from './BaseChat';
import { ChatType } from '../types/chat';
import { generateSessionId } from '../sessions';
import { DEFAULT_CHAT_TITLE } from '../contexts/ChatContext';
import { TabBar } from './TabChat/components';

// Wrapper for BaseChat that prevents unnecessary re-renders
const StableBaseChat: React.FC<{
  chat: ChatType;
  setChat: (chat: ChatType) => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  setIsGoosehintsModalOpen: (isOpen: boolean) => void;
}> = React.memo(({ chat, setChat, setView, setIsGoosehintsModalOpen }) => {
  return (
    <div className="relative z-10 flex justify-center h-full bg-transparent flex-grow">
      <div className="w-full max-w-[1000px] h-full bg-transparent">
        <BaseChat
          chat={chat}
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
}, (prevProps, nextProps) => {
  // Only re-render if the chat ID changes or messages length changes
  return prevProps.chat.id === nextProps.chat.id && 
         prevProps.chat.messages?.length === nextProps.chat.messages?.length;
});

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
  // Initialize tabs state with the provided chat
  const [tabs, setTabs] = useState<ChatType[]>(() => {
    // Ensure we always have at least one tab with the current chat
    return [{
      ...chat,
      messages: chat.messages || []
    }];
  });
  
  // Track active tab ID
  const [activeTabId, setActiveTabId] = useState<string>(chat.id);
  
  // Log for debugging
  console.log('Current tabs:', tabs.map(tab => ({ id: tab.id, title: tab.title })));
  console.log('Active tab ID:', activeTabId);
  
  // Get active chat
  const activeChat = tabs.find(tab => tab.id === activeTabId) || tabs[0];
  
  // Ensure active chat has messages array
  const safeActiveChat = {
    ...activeChat,
    messages: activeChat.messages || []
  };
  
  // Update parent chat state when active tab changes - but only when tab ID changes
  const prevTabIdRef = React.useRef(activeTabId);
  useEffect(() => {
    if (prevTabIdRef.current !== activeTabId) {
      prevTabIdRef.current = activeTabId;
      setChat(safeActiveChat);
    }
  }, [activeTabId, safeActiveChat, setChat]);
  
  const location = useLocation();

  // Handle recipe loading from recipes view - reset chat if needed
  useEffect(() => {
    if (location.state?.resetChat && location.state?.recipeConfig) {
      // Reset the active chat to start fresh with the recipe
      const updatedChat = {
        id: activeChat.id, // Keep the same ID to maintain the session
        title: location.state.recipeConfig.title || 'Recipe Chat',
        messages: [], // Clear messages to start fresh
        messageHistoryIndex: 0,
        recipeConfig: location.state.recipeConfig, // Set the recipe config in chat state
        recipeParameters: null, // Clear parameters for new recipe
      };
      
      // Update the tab in the tabs array
      setTabs(prevTabs => 
        prevTabs.map(tab => 
          tab.id === activeTabId ? updatedChat : tab
        )
      );
      
      // Update the parent chat state
      setChat(updatedChat);

      // Clear the location state to prevent re-processing
      window.history.replaceState({}, '', '/pair');
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [location.state, activeTabId]);
  
  // Create a new tab
  const handleNewTab = () => {
    const newChat: ChatType = {
      id: generateSessionId(),
      title: DEFAULT_CHAT_TITLE,
      messages: [], // Initialize with empty array
      messageHistoryIndex: 0,
      recipeConfig: null,
    };
    
    console.log('Creating new tab:', newChat.id);
    setTabs(prevTabs => [...prevTabs, newChat]);
    setActiveTabId(newChat.id);
  };
  
  // Close a tab
  const handleCloseTab = (tabId: string) => {
    // Don't close if it's the only tab
    if (tabs.length <= 1) return;
    
    console.log('Closing tab:', tabId);
    
    // If closing the active tab, switch to another tab
    if (tabId === activeTabId) {
      const tabIndex = tabs.findIndex(tab => tab.id === tabId);
      const newActiveIndex = tabIndex === 0 ? 1 : tabIndex - 1;
      setActiveTabId(tabs[newActiveIndex].id);
    }
    
    // Remove the tab
    setTabs(prevTabs => prevTabs.filter(tab => tab.id !== tabId));
  };
  
  // Update the active chat - with optimized update logic
  const handleSetActiveChat = (updatedChat: ChatType) => {
    // Ensure the updated chat has a messages array
    const safeUpdatedChat = {
      ...updatedChat,
      messages: updatedChat.messages || [],
    };
    
    // Update the tab in the tabs array
    setTabs(prevTabs => 
      prevTabs.map(tab => 
        tab.id === activeTabId ? safeUpdatedChat : tab
      )
    );
  };

  // Format tabs for the TabBar component
  const formattedTabs = tabs.map(tab => ({
    id: tab.id,
    title: tab.title || `Chat ${tabs.indexOf(tab) + 1}`
  }));

  return (
    <div className="flex flex-col h-full relative bg-transparent">
      {/* Tab Bar */}
      <TabBar
        tabs={formattedTabs}
        activeTabId={activeTabId}
        onTabSelect={setActiveTabId}
        onTabClose={handleCloseTab}
        onNewTab={handleNewTab}
      />
      
      {/* Stable BaseChat wrapper */}
      <StableBaseChat
        chat={safeActiveChat}
        setChat={handleSetActiveChat}
        setView={setView}
        setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
      />
    </div>
  );
}
