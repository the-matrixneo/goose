import React, { useRef } from 'react';
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

  // Calculate padding based on sidebar state and macOS
  const headerPadding = !isSidebarOpen ? (safeIsMacOS ? 'pl-20' : 'pl-12') : 'pl-4';

  return (
    <div className="h-12 flex items-center justify-between bg-background-default rounded-xl border border-border-subtle shadow-sm mx-4 mt-4 mb-4">
      {/* Left side - Sidebar toggle and Directory */}
      <div className={`flex items-center gap-3 ${headerPadding}`}>
        <SidebarTrigger className="no-drag" />
        <div className="h-4 w-px bg-border-subtle" />
        <DirSwitcher hasMessages={hasMessages} />
      </div>

      {/* Center - Status + Model and Mode selections */}
      <div className="flex items-center gap-3" ref={dropdownRef}>
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
      </div>

      {/* Right side - Summarize (always reserve space) */}
      <div className="flex items-center gap-3 pr-4 min-w-[80px] justify-end">
        {/* Summarize Button - always reserve space */}
        {messages.length > 0 && setMessages ? (
          <div className="text-xs">
            <ManualSummarizeButton
              messages={messages}
              isLoading={isLoading}
              setMessages={setMessages}
            />
          </div>
        ) : (
          // Invisible placeholder to maintain consistent spacing
          <div className="text-xs opacity-0 pointer-events-none">
            summarize
          </div>
        )}
      </div>
    </div>
  );
};