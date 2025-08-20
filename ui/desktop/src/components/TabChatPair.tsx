import React, { useState } from 'react';
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
  // Initialize tabs state with the provided chat
  const [tabs, setTabs] = useState<ChatType[]>([{
    ...chat,
    messages: chat.messages || []
  }]);
  
  // Track active tab index (not ID)
  const [activeTabIndex, setActiveTabIndex] = useState(0);
  
  // Get active chat by index
  const activeChat = tabs[activeTabIndex] || tabs[0];
  
  // Create a new tab
  const handleNewTab = () => {
    const newChat: ChatType = {
      id: generateSessionId(),
      title: `Chat ${tabs.length + 1}`,
      messages: [],
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
  const handleCloseTab = (index: number) => {
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
      setChat(newTabs[newIndex]);
    } else if (index < activeTabIndex) {
      // If closing a tab before the active one, decrement the active index
      setActiveTabIndex(activeTabIndex - 1);
    }
  };
  
  // Switch to a tab
  const handleTabSelect = (index: number) => {
    setActiveTabIndex(index);
    setChat(tabs[index]);
  };
  
  // Update the active chat
  const handleSetActiveChat = (updatedChat: ChatType) => {
    // Update the tab in the tabs array
    const newTabs = [...tabs];
    newTabs[activeTabIndex] = updatedChat;
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
                onClick={(e) => {
                  e.stopPropagation();
                  handleCloseTab(index);
                }}
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
            key={activeChat.id} // Force re-render when active tab changes
            chat={activeChat}
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
