import React, { useRef } from 'react';
import { SidebarTrigger } from './ui/sidebar';
import { DirSwitcher } from './bottom_menu/DirSwitcher';
import ModelsBottomBar from './settings/models/bottom_bar/ModelsBottomBar';
import { BottomMenuModeSelection } from './bottom_menu/BottomMenuModeSelection';
import { useSidebar } from './ui/sidebar';
import { View, ViewOptions } from '../App';

interface HeaderToolbarProps {
  setView: (view: View, viewOptions?: ViewOptions) => void;
  hasMessages?: boolean;
}

export const HeaderToolbar: React.FC<HeaderToolbarProps> = ({ setView, hasMessages = false }) => {
  const safeIsMacOS = (window?.electron?.platform || 'darwin') === 'darwin';
  const { open: isSidebarOpen } = useSidebar();
  const dropdownRef = useRef<HTMLDivElement>(null);

  // Calculate padding based on sidebar state and macOS
  const headerPadding = !isSidebarOpen ? (safeIsMacOS ? 'pl-20' : 'pl-12') : 'pl-4';

  return (
    <div className="h-12 flex items-center justify-between border-b border-border-subtle bg-background-default rounded-t-2xl">
      {/* Left side - Sidebar toggle and Directory */}
      <div className={`flex items-center gap-3 ${headerPadding}`}>
        <SidebarTrigger className="no-drag" />
        <div className="h-4 w-px bg-border-subtle" />
        <DirSwitcher hasMessages={hasMessages} />
      </div>

      {/* Right side - Model selection and Mode */}
      <div className="flex items-center gap-3 pr-4" ref={dropdownRef}>
        <ModelsBottomBar dropdownRef={dropdownRef} setView={setView} />
        <div className="h-4 w-px bg-border-subtle" />
        <BottomMenuModeSelection setView={setView} />
      </div>
    </div>
  );
};