import React, { useState, useEffect } from 'react';
import { Plus } from 'lucide-react';
import TabPill from './TabPill';
import { ScrollArea } from '../ui/scroll-area';
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
  const [scrollAreaWidth, setScrollAreaWidth] = useState<number>(0);
  const [scrollRef, setScrollRef] = useState<HTMLDivElement | null>(null);

  // Scroll to active tab when it changes
  useEffect(() => {
    if (scrollRef) {
      const activeTabElement = scrollRef.querySelector(`[data-tab-id="${activeTabId}"]`);
      if (activeTabElement) {
        // Calculate position to center the active tab
        const tabRect = activeTabElement.getBoundingClientRect();
        const scrollAreaRect = scrollRef.getBoundingClientRect();
        const centerPosition = tabRect.left + tabRect.width / 2 - scrollAreaRect.width / 2;
        
        scrollRef.scrollTo({
          left: centerPosition,
          behavior: 'smooth'
        });
      }
    }
  }, [activeTabId, scrollRef]);

  // Update scroll area width on resize
  useEffect(() => {
    const updateWidth = () => {
      if (scrollRef) {
        setScrollAreaWidth(scrollRef.offsetWidth);
      }
    };

    updateWidth();
    window.addEventListener('resize', updateWidth);
    return () => window.removeEventListener('resize', updateWidth);
  }, [scrollRef]);

  return (
    <div className="flex flex-col">
      <div className="relative">
        <ScrollArea 
          orientation="horizontal" 
          className="pb-1"
          scrollbarClassName="h-1.5"
          ref={setScrollRef as any}
        >
          <div className="flex items-center gap-1 px-1 py-1.5">
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
