import React, { useState, useEffect, useRef } from 'react';
import { Plus } from 'lucide-react';
import TabPill from './TabPill';
import { ScrollArea, ScrollAreaHandle } from '../ui/scroll-area';
import { cn } from '../../utils';

export interface ChatTab {
  id: string;
  title: string;
  isNewChat?: boolean;
}

interface TabBarProps {
  tabs: ChatTab[];
  activeTabId: string;
  onTabSelect: (tabId: string) => void;
  onTabClose: (tabId: string) => void;
  onNewTab: () => void;
}

const TabBar: React.FC<TabBarProps> = ({
  tabs,
  activeTabId,
  onTabSelect,
  onTabClose,
  onNewTab
}) => {
  const scrollAreaRef = useRef<ScrollAreaHandle>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  // Update scroll area width on resize - no automatic scrolling for now
  useEffect(() => {
    const handleResize = () => {
      // This is just to ensure the component responds to window resizing
      // No actual state updates to prevent infinite loops
    };

    window.addEventListener('resize', handleResize);
    return () => window.removeEventListener('resize', handleResize);
  }, []);

  return (
    <div className="flex flex-col">
      <div className="relative">
        <ScrollArea 
          orientation="horizontal" 
          className="pb-1"
          scrollbarClassName="h-1.5"
          ref={scrollAreaRef}
        >
          <div 
            ref={containerRef}
            className="flex items-center gap-1 px-1 py-1.5"
          >
            {tabs.map((tab) => (
              <div 
                key={tab.id} 
                data-tab-id={tab.id}
                className="flex-shrink-0"
              >
                <TabPill
                  label={tab.title}
                  isActive={tab.id === activeTabId}
                  onClick={() => onTabSelect(tab.id)}
                  onClose={tabs.length > 1 ? () => onTabClose(tab.id) : undefined}
                  isNewChat={tab.isNewChat}
                />
              </div>
            ))}
            
            {/* New tab button */}
            <div className="flex-shrink-0">
              <button
                onClick={onNewTab}
                className={cn(
                  "flex items-center justify-center p-1.5 rounded-md",
                  "text-textStandard hover:bg-background-muted transition-colors",
                  "cursor-pointer"
                )}
              >
                <Plus className="h-4 w-4" />
              </button>
            </div>
          </div>
        </ScrollArea>
      </div>
    </div>
  );
};

export default TabBar;
