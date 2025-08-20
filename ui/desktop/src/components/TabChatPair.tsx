import React, { useState } from 'react';
import { useLocation } from 'react-router-dom';
import { type View, ViewOptions } from '../App';
import BaseChat from './BaseChat';
import { ChatType } from '../types/chat';
import { generateSessionId } from '../sessions';
import { DEFAULT_CHAT_TITLE } from '../contexts/ChatContext';

// Simple tab interface styles
const styles = {
  tabContainer: 'flex overflow-x-auto border-b border-borderSubtle pb-1 mb-2 hide-scrollbar',
  tab: 'px-3 py-1.5 mr-1 rounded-t-md text-sm font-medium whitespace-nowrap cursor-pointer transition-colors',
  activeTab: 'bg-background-default text-textProminent',
  inactiveTab: 'hover:bg-background-muted text-textStandard',
  newTab: 'px-2 py-1.5 rounded-md text-textStandard hover:bg-background-muted cursor-pointer',
  closeButton: 'ml-2 text-xs opacity-60 hover:opacity-100',
  hideScrollbar: `
    .hide-scrollbar {
      -ms-overflow-style: none;
      scrollbar-width: none;
    }
    .hide-scrollbar::-webkit-scrollbar {
      display: none;
    }
  `
};

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
  const [tabs, setTabs] = useState<ChatType[]>([{
    ...chat,
    messages: chat.messages || []
  }]);
  
  // Track active tab ID
  const [activeTabId, setActiveTabId] = useState<string>(chat.id);
  
  // Get active chat
  const activeChat = tabs.find(tab => tab.id === activeTabId) || tabs[0];
  
  // Ensure active chat has messages array
  const safeActiveChat = React.useMemo(() => ({
    ...activeChat,
    messages: activeChat.messages || []
  }), [activeChat]);
  
  // Update parent chat state when active chat changes
  React.useEffect(() => {
    setChat(safeActiveChat);
  }, [safeActiveChat, setChat]);
  
  const location = useLocation();

  // Handle recipe loading from recipes view - reset chat if needed
  React.useEffect(() => {
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
    
    setTabs(prevTabs => [...prevTabs, newChat]);
    setActiveTabId(newChat.id);
  };
  
  // Close a tab
  const handleCloseTab = (tabId: string, event: React.MouseEvent) => {
    event.stopPropagation(); // Prevent tab selection when clicking close button
    
    // Don't close if it's the only tab
    if (tabs.length <= 1) return;
    
    // If closing the active tab, switch to another tab
    if (tabId === activeTabId) {
      const tabIndex = tabs.findIndex(tab => tab.id === tabId);
      const newActiveIndex = tabIndex === 0 ? 1 : tabIndex - 1;
      setActiveTabId(tabs[newActiveIndex].id);
    }
    
    // Remove the tab
    setTabs(prevTabs => prevTabs.filter(tab => tab.id !== tabId));
  };
  
  // Update the active chat
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

  return (
    <div className="flex flex-col h-full relative bg-transparent">
      {/* Inject the scrollbar hiding styles */}
      <style>{styles.hideScrollbar}</style>
      
      {/* Simple tab bar */}
      <div className={styles.tabContainer}>
        {tabs.map(tab => (
          <div 
            key={tab.id}
            className={`${styles.tab} ${tab.id === activeTabId ? styles.activeTab : styles.inactiveTab}`}
            onClick={() => setActiveTabId(tab.id)}
          >
            {tab.title || `Chat ${tabs.indexOf(tab) + 1}`}
            {tabs.length > 1 && (
              <button
                className={styles.closeButton}
                onClick={(e) => handleCloseTab(tab.id, e)}
                aria-label="Close tab"
              >
                ✕
              </button>
            )}
          </div>
        ))}
        <div 
          className={styles.newTab}
          onClick={handleNewTab}
          title="New tab"
        >
          +
        </div>
      </div>
      
      {/* Chat content */}
      <div className="relative z-10 flex justify-center h-full bg-transparent flex-grow">
        <div className="w-full max-w-[1000px] h-full bg-transparent">
          <BaseChat
            chat={safeActiveChat}
            setChat={handleSetActiveChat}
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
    </div>
  );
}
