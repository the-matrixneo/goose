import React, { useRef, useState, useEffect } from 'react';
import { SidebarTrigger } from './ui/sidebar';
import { DirSwitcher } from './bottom_menu/DirSwitcher';
import ModelsBottomBar from './settings/models/bottom_bar/ModelsBottomBar';
import { BottomMenuModeSelection } from './bottom_menu/BottomMenuModeSelection';
import { useSidebar } from './ui/sidebar';
import { View, ViewOptions } from '../App';
import BottomMenuAlertPopover from './bottom_menu/BottomMenuAlertPopover';
import { ManualSummarizeButton } from './context_management/ManualSummaryButton';
import { Alert } from './alerts';
import { Message } from '../types/message';

interface HeaderToolbarProps {
  setView: (view: View, viewOptions?: ViewOptions) => void;
  hasMessages?: boolean;
  alerts?: Alert[];
  messages?: Message[];
  isLoading?: boolean;
  setMessages?: (messages: Message[]) => void;
}

export const HeaderToolbar: React.FC<HeaderToolbarProps> = ({ 
  setView, 
  hasMessages = false,
  alerts = [],
  messages = [],
  isLoading = false,
  setMessages
}) => {
  const safeIsMacOS = (window?.electron?.platform || 'darwin') === 'darwin';
  const { open: isSidebarOpen } = useSidebar();
  const dropdownRef = useRef<HTMLDivElement>(null);
  const [isInitialized, setIsInitialized] = useState(false);

  // Wait for sidebar state to be initialized to prevent layout shift
  useEffect(() => {
    const timer = setTimeout(() => {
      setIsInitialized(true);
    }, 50); // Small delay to let sidebar state settle
    return () => clearTimeout(timer);
  }, []);

  // Calculate positioning to match ChatInput margins (mx-6 = 24px)
  const leftPosition = !isSidebarOpen 
    ? 'left-6' // Match ChatInput left margin when sidebar collapsed
    : 'left-6'; // Always match ChatInput left margin

  return (
    <div className={`absolute top-8 right-6 z-10 flex items-center justify-between ${leftPosition} transition-opacity duration-200 ${
      isInitialized ? 'opacity-100' : 'opacity-0'
    }`}>
      {/* Toolbar container matching ChatInput width exactly */}
      <div className={`flex items-center justify-between w-full bg-background-default rounded-xl border border-border-subtle shadow-sm py-2 ${
        !isSidebarOpen ? 'pl-6 pr-4' : 'pl-4 pr-4' // Consistent spacing with minimal clearance for stoplight buttons
      }`} ref={dropdownRef}>
        
        {/* Left side - Sidebar toggle */}
        <div className="flex items-center">
          <SidebarTrigger className="no-drag" />
        </div>

        {/* Center - Directory, Status + Model, Mode, and Summarize */}
        <div className="flex items-center gap-3">
          <DirSwitcher hasMessages={hasMessages} />
          <div className="h-4 w-px bg-border-subtle" />
          
          {/* Model selection with Status Icon to the left */}
          <div className="flex items-center gap-1">
            {/* Status Icon to the left of model selection */}
            {alerts.length > 0 && (
              <BottomMenuAlertPopover alerts={alerts} />
            )}
            <ModelsBottomBar dropdownRef={dropdownRef} setView={setView} />
          </div>
          
          <div className="h-4 w-px bg-border-subtle" />
          <BottomMenuModeSelection setView={setView} />
          
          <div className="h-4 w-px bg-border-subtle" />
          {/* Summarize Button - always shown, to the right of spiral icon */}
          <ManualSummarizeButton
            messages={messages}
            isLoading={isLoading}
            setMessages={setMessages}
          />
        </div>

        {/* Right side - Empty space for balance */}
        <div className="flex items-center min-w-[40px] justify-end">
          {/* Empty space to balance the left sidebar toggle */}
        </div>
      </div>
    </div>
  );
};