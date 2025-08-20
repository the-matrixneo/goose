import React, { useState, useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import { type View, ViewOptions } from '../App';
import BaseChat from './BaseChat';
import { ChatType } from '../types/chat';
import { generateSessionId } from '../sessions';
import { DEFAULT_CHAT_TITLE } from '../contexts/ChatContext';

/**
 * Simple TabChat implementation that avoids complex components and state management
 */
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
  // Ensure the initial chat has a messages array
  const initialChat = {
    ...chat,
    messages: chat.messages || []
  };
  
  // Initialize tabs state with the provided chat
  const [tabs, setTabs] = useState<ChatType[]>([initialChat]);
  
  // Track active tab index (not ID)
  const [activeTabIndex, setActiveTabIndex] = useState(0);
  
  // Get active chat by index with safety check
  const activeChat = activeTabIndex < tabs.length ? tabs[activeTabIndex] : tabs[0];
  
  // Double ensure active chat has messages array
  const safeActiveChat = {
    ...activeChat,
    messages: activeChat.messages || []
  };
  
  // Update parent chat state when component mounts or active tab changes
  useEffect(() => {
    // Ensure we're passing a chat with a messages array
    const chatToSet = {
      ...safeActiveChat,
      messages: safeActiveChat.messages || []
    };
    setChat(chatToSet);
  }, [activeTabIndex, safeActiveChat, setChat]);
  
  // Create a new tab
  const handleNewTab = () => {
    const newChat: ChatType = {
      id: generateSessionId(),
      title: `Chat ${tabs.length + 1}`,
      messages: [], // Explicitly initialize with empty array
      messageHistoryIndex: 0,
      recipeConfig: null,
    };
    
    // Add the new tab
    setTabs([...tabs, newChat]);
    
    // Switch to the new tab
    setActiveTabIndex(tabs.length);
    
    // Update parent chat state
    setChat(newChat);
  };
  
  // Close a tab
  const handleCloseTab = (index: number, e: React.MouseEvent) => {
    // Stop event propagation to prevent tab selection
    e.stopPropagation();
    
    // Don't close if it's the only tab
    if (tabs.length <= 1) return;
    
    // Create new tabs array without the closed tab
    const newTabs = tabs.filter((_, i) => i !== index);
    
    // Update tabs
    setTabs(newTabs);
    
    // If closing the active tab or a tab before it, adjust the active index
    if (index === activeTabIndex) {
      // If closing the last tab, go to the new last tab
      const newIndex = Math.min(activeTabIndex, newTabs.length - 1);
      setActiveTabIndex(newIndex);
      
      // Update parent chat state with safety check
      const chatToSet = {
        ...newTabs[newIndex],
        messages: newTabs[newIndex]?.messages || []
      };
      setChat(chatToSet);
    } else if (index < activeTabIndex) {
      // If closing a tab before the active one, decrement the active index
      setActiveTabIndex(activeTabIndex - 1);
    }
  };
  
  // Switch to a tab
  const handleTabSelect = (index: number) => {
    setActiveTabIndex(index);
    
    // Update parent chat state with safety check
    const chatToSet = {
      ...tabs[index],
      messages: tabs[index]?.messages || []
    };
    setChat(chatToSet);
  };
  
  // Update the active chat
  const handleSetActiveChat = (updatedChat: ChatType) => {
    // Ensure the updated chat has a messages array
    const safeUpdatedChat = {
      ...updatedChat,
      messages: updatedChat.messages || []
    };
    
    // Update the tab in the tabs array
    const newTabs = [...tabs];
    newTabs[activeTabIndex] = safeUpdatedChat;
    setTabs(newTabs);
  };

  return (
    <div className="flex flex-col h-full relative bg-transparent">
      {/* Simple tab bar */}
      <div className="absolute left-4 top-10 z-50 flex flex-col bg-white/80 backdrop-blur-md p-2 rounded-lg shadow-lg border border-gray-200">
        {tabs.map((tab, index) => (
          <div 
            key={tab.id}
            className={`px-3 py-1.5 mb-1 rounded-full text-sm font-medium whitespace-nowrap cursor-pointer transition-colors ${
              index === activeTabIndex 
                ? 'bg-blue-500 text-white shadow-sm' 
                : 'bg-gray-100 hover:bg-gray-200 text-gray-700'
            }`}
            onClick={() => handleTabSelect(index)}
          >
            <span className="truncate max-w-[120px] inline-block">
              {tab.title || `Chat ${index + 1}`}
            </span>
            {tabs.length > 1 && (
              <button
                className="ml-2 text-xs opacity-60 hover:opacity-100"
                onClick={(e) => handleCloseTab(index, e)}
                aria-label="Close tab"
              >
                ✕
              </button>
            )}
          </div>
        ))}
        
        {/* New tab button */}
        <div 
          className="px-3 py-1.5 rounded-full bg-green-500 text-white hover:bg-green-600 cursor-pointer font-bold"
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
            key={safeActiveChat.id} // Force re-render when active tab changes
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
