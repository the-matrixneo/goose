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
 * A simple tab component that doesn't use any complex UI libraries
 */
export const Tab: React.FC<TabProps> = ({ id, title, isActive, onSelect, onClose }) => {
  return (
    <div 
      className={`px-3 py-1.5 mr-1 rounded-t-md text-sm font-medium whitespace-nowrap cursor-pointer transition-colors ${
        isActive 
          ? 'bg-background-default text-textProminent' 
          : 'hover:bg-background-muted text-textStandard'
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
 * A simple button to create a new tab
 */
export const NewTabButton: React.FC<NewTabButtonProps> = ({ onClick }) => {
  return (
    <div 
      className="px-2 py-1.5 rounded-md text-textStandard hover:bg-background-muted cursor-pointer"
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
 * A simple tab bar component that doesn't use any complex UI libraries
 */
export const TabBar: React.FC<TabBarProps> = ({ 
  tabs, 
  activeTabId, 
  onTabSelect, 
  onTabClose, 
  onNewTab 
}) => {
  return (
    <div className="flex overflow-x-auto border-b border-borderSubtle pb-1 mb-2 hide-scrollbar">
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
