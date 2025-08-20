import React from 'react';
import { Plus } from 'lucide-react';
import { cn } from '../../utils';

// Add a CSS class to hide scrollbars but maintain scroll functionality
const SCROLLBAR_HIDE_STYLES = `
.hide-scrollbar {
  -ms-overflow-style: none;  /* IE and Edge */
  scrollbar-width: none;  /* Firefox */
}
.hide-scrollbar::-webkit-scrollbar {
  display: none;  /* Chrome, Safari and Opera */
}
`;

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

const TabPill: React.FC<{
  label: string;
  isActive: boolean;
  onClick: () => void;
  onClose?: () => void;
  isNewChat?: boolean;
}> = ({ 
  label, 
  isActive, 
  onClick, 
  onClose,
  isNewChat = false 
}) => {
  return (
    <div 
      className={cn(
        "flex items-center gap-2 px-3 py-1.5 rounded-md cursor-pointer transition-all",
        "text-sm font-medium whitespace-nowrap max-w-[180px] overflow-hidden text-ellipsis",
        isActive 
          ? "bg-background-default text-textProminent shadow-sm" 
          : "hover:bg-background-muted text-textStandard",
        isNewChat && "bg-blue-500/10 hover:bg-blue-500/20"
      )}
      onClick={onClick}
    >
      <span className="truncate">{label}</span>
      
      {!isNewChat && onClose && (
        <button 
          className="opacity-50 hover:opacity-100 rounded-full p-0.5 hover:bg-background-muted"
          onClick={(e) => {
            e.stopPropagation();
            onClose();
          }}
          aria-label="Close tab"
        >
          <svg width="12" height="12" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <line x1="18" y1="6" x2="6" y2="18"></line>
            <line x1="6" y1="6" x2="18" y2="18"></line>
          </svg>
        </button>
      )}
    </div>
  );
};

const TabBar: React.FC<TabBarProps> = ({
  tabs,
  activeTabId,
  onTabSelect,
  onTabClose,
  onNewTab
}) => {
  // Create a ref for the scrollable container
  const scrollContainerRef = React.useRef<HTMLDivElement>(null);
  
  // Scroll to active tab when it changes - using imperative code outside of React's rendering cycle
  React.useEffect(() => {
    // Use requestAnimationFrame to ensure DOM is ready
    const scrollToActiveTab = () => {
      if (!scrollContainerRef.current) return;
      
      // Find the active tab element
      const activeTabElement = Array.from(scrollContainerRef.current.children)
        .find(child => {
          const tabId = child.getAttribute('data-tab-id');
          return tabId === activeTabId;
        }) as HTMLElement | undefined;
      
      if (activeTabElement) {
        // Calculate the scroll position to center the active tab
        const container = scrollContainerRef.current;
        const tabRect = activeTabElement.getBoundingClientRect();
        const containerRect = container.getBoundingClientRect();
        
        // Calculate center position
        const tabCenter = tabRect.left + tabRect.width / 2;
        const containerCenter = containerRect.left + containerRect.width / 2;
        const scrollOffset = tabCenter - containerCenter;
        
        // Smooth scroll to position
        container.scrollBy({
          left: scrollOffset,
          behavior: 'smooth'
        });
      }
    };
    
    // Delay scrolling slightly to ensure DOM is ready
    const timeoutId = setTimeout(scrollToActiveTab, 100);
    return () => clearTimeout(timeoutId);
  }, [activeTabId, tabs.length]);
  
  return (
    <div className="flex flex-col">
      {/* Inject the scrollbar hiding styles */}
      <style>{SCROLLBAR_HIDE_STYLES}</style>
      
      <div className="relative border-b border-border-subtle">
        <div 
          className="overflow-x-auto hide-scrollbar py-1 px-1"
          ref={scrollContainerRef}
        >
          <div className="flex items-center gap-1 min-w-max">
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
            <div className="flex-shrink-0 pl-1">
              <button
                onClick={onNewTab}
                className={cn(
                  "flex items-center justify-center p-1.5 rounded-md",
                  "text-textStandard hover:bg-background-muted transition-colors",
                  "cursor-pointer"
                )}
                aria-label="New tab"
              >
                <Plus className="h-4 w-4" />
              </button>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
};

export default TabBar;
export { TabPill };
