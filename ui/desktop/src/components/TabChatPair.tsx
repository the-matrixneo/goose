import React, { useState, useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import { type View, ViewOptions } from '../App';
import { ChatType } from '../types/chat';
import { generateSessionId } from '../sessions';
import { DEFAULT_CHAT_TITLE } from '../contexts/ChatContext';
import { TabBar, ChatContent } from './TabChat/components';

// Simple placeholder component for chat content
const SimpleChatPlaceholder: React.FC<{
  chat: ChatType;
  onNewChat: () => void;
}> = ({ chat, onNewChat }) => {
  return (
    <div className="flex flex-col items-center justify-center h-full p-8 text-center">
      <h2 className="text-2xl font-bold mb-4">Tab Chat Interface</h2>
      <p className="mb-6 max-w-md">
        This is a simplified implementation of the tab chat interface. The full implementation
        with chat functionality will be added once the infinite update loop issues are resolved.
      </p>
      <p className="mb-4">
        <strong>Current chat ID:</strong> {chat.id}
      </p>
      <p className="mb-4">
        <strong>Title:</strong> {chat.title || 'Untitled Chat'}
      </p>
      <p className="mb-4">
        <strong>Messages:</strong> {chat.messages?.length || 0}
      </p>
      <button
        className="px-4 py-2 bg-blue-500 text-white rounded hover:bg-blue-600"
        onClick={onNewChat}
      >
        Create New Tab
      </button>
    </div>
  );
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
  const safeActiveChat = {
    ...activeChat,
    messages: activeChat.messages || []
  };
  
  // Update parent chat state when active tab changes
  const firstRenderRef = React.useRef(true);
  useEffect(() => {
    if (firstRenderRef.current) {
      firstRenderRef.current = false;
      return;
    }
    setChat(safeActiveChat);
  }, [safeActiveChat.id, setChat]);
  
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
    
    setTabs(prevTabs => [...prevTabs, newChat]);
    setActiveTabId(newChat.id);
  };
  
  // Close a tab
  const handleCloseTab = (tabId: string) => {
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
      
      {/* Simple placeholder instead of BaseChat */}
      <div className="flex-grow">
        <SimpleChatPlaceholder 
          chat={safeActiveChat}
          onNewChat={handleNewTab}
        />
      </div>
    </div>
  );
}
