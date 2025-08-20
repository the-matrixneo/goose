import React from 'react';
import { ChatType } from '../types/chat';

interface TabProps {
  id: string;
  title: string;
  isActive: boolean;
  onSelect: () => void;
  onClose?: () => void;
}

/**
 * A simple tab component with improved visibility
 */
export const Tab: React.FC<TabProps> = ({ id, title, isActive, onSelect, onClose }) => {
  return (
    <div 
      className={`px-3 py-1.5 mr-1 rounded-t-md text-sm font-medium whitespace-nowrap cursor-pointer transition-colors ${
        isActive 
          ? 'bg-blue-500 text-white shadow-md' 
          : 'bg-gray-100 hover:bg-gray-200 text-gray-700'
      }`}
      onClick={onSelect}
    >
      {title}
      {onClose && (
        <button
          className="ml-2 text-xs opacity-60 hover:opacity-100"
          onClick={(e) => {
            e.stopPropagation();
            onClose();
          }}
          aria-label="Close tab"
        >
          ✕
        </button>
      )}
    </div>
  );
};

interface NewTabButtonProps {
  onClick: () => void;
}

/**
 * A more visible button to create a new tab
 */
export const NewTabButton: React.FC<NewTabButtonProps> = ({ onClick }) => {
  return (
    <div 
      className="px-3 py-1.5 rounded-md bg-green-500 text-white hover:bg-green-600 cursor-pointer font-bold"
      onClick={onClick}
      title="New tab"
    >
      +
    </div>
  );
};

interface TabBarProps {
  tabs: Array<{
    id: string;
    title: string;
  }>;
  activeTabId: string;
  onTabSelect: (tabId: string) => void;
  onTabClose: (tabId: string) => void;
  onNewTab: () => void;
}

/**
 * A more visible tab bar component
 */
export const TabBar: React.FC<TabBarProps> = ({ 
  tabs, 
  activeTabId, 
  onTabSelect, 
  onTabClose, 
  onNewTab 
}) => {
  return (
    <div className="flex overflow-x-auto border-b-2 border-gray-300 pb-1 mb-2 px-2 pt-2 bg-gray-50">
      <style>{`
        .hide-scrollbar {
          -ms-overflow-style: none;
          scrollbar-width: none;
        }
        .hide-scrollbar::-webkit-scrollbar {
          display: none;
        }
      `}</style>
      
      {tabs.map(tab => (
        <Tab 
          key={tab.id}
          id={tab.id}
          title={tab.title}
          isActive={tab.id === activeTabId}
          onSelect={() => onTabSelect(tab.id)}
          onClose={tabs.length > 1 ? () => onTabClose(tab.id) : undefined}
        />
      ))}
      
      <NewTabButton onClick={onNewTab} />
    </div>
  );
};

interface ChatContentProps {
  chat: ChatType;
  children: React.ReactNode;
}

/**
 * A simple wrapper for the chat content
 */
export const ChatContent: React.FC<ChatContentProps> = ({ chat, children }) => {
  return (
    <div className="relative z-10 flex justify-center h-full bg-transparent flex-grow">
      <div className="w-full max-w-[1000px] h-full bg-transparent">
        {children}
      </div>
    </div>
  );
};
