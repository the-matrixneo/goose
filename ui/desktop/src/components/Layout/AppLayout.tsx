import React from 'react';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import { SidebarProvider, SidebarInset, Sidebar, SidebarTrigger, useSidebar } from '../ui/sidebar';
import AppSidebar from '../GooseSidebar/AppSidebar';
import { View, ViewOptions } from '../../App';
import { Gear, Time } from '../icons';
import { Button } from '../ui/button';

interface AppLayoutProps {
  setIsGoosehintsModalOpen?: (isOpen: boolean) => void;
}

// Inner component that uses useSidebar within SidebarProvider context
const AppLayoutContent: React.FC<AppLayoutProps> = ({ setIsGoosehintsModalOpen }) => {
  const navigate = useNavigate();
  const location = useLocation();
  const { open: isSidebarOpen } = useSidebar();
  const safeIsMacOS = (window?.electron?.platform || 'darwin') === 'darwin';

  // Calculate padding based on sidebar state and macOS
  const headerPadding = !isSidebarOpen ? (safeIsMacOS ? 'pl-[32px]' : 'pl-0') : 'pl-0';

  const setView = (view: View, viewOptions?: ViewOptions) => {
    // Convert view-based navigation to route-based navigation
    switch (view) {
      case 'chat':
        navigate('/');
        break;
      case 'pair':
        navigate('/pair');
        break;
      case 'settings':
        navigate('/settings', { state: viewOptions });
        break;
      case 'sessions':
        navigate('/sessions');
        break;
      case 'schedules':
        navigate('/schedules');
        break;
      case 'recipes':
        navigate('/recipes');
        break;
      case 'permission':
        navigate('/permission', { state: viewOptions });
        break;
      case 'ConfigureProviders':
        navigate('/configure-providers');
        break;
      case 'sharedSession':
        navigate('/shared-session', { state: viewOptions });
        break;
      case 'recipeEditor':
        navigate('/recipe-editor', { state: viewOptions });
        break;
      case 'welcome':
        navigate('/welcome');
        break;
      default:
        navigate('/');
    }
  };

  const handleSelectSession = async (sessionId: string) => {
    // Navigate to chat with session data
    navigate('/', { state: { sessionId } });
  };

  // Helper function to check if a path is active
  const isActivePath = (path: string) => {
    return location.pathname === path;
  };

  return (
    <div className="flex flex-1 w-full relative animate-fade-in">
      <Sidebar variant="inset" collapsible="icon">
        <AppSidebar
          onSelectSession={handleSelectSession}
          setView={setView}
          setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
          currentPath={location.pathname}
        />
      </Sidebar>
      <SidebarInset>
        <div
          className={`${headerPadding} h-12 z-100 w-full flex items-center justify-between pr-2 py-1`}
        >
          <SidebarTrigger className={`no-drag hover:bg-background-medium`} />

          {/* Header Action Buttons */}
          <div className="flex items-center">
            {/* History Button */}
            <Button
              onClick={() => navigate('/sessions')}
              variant="ghost"
              size="xs"
              aria-label="View History"
              className="hover:bg-background-medium no-drag"
            >
              <Time className="w-4 h-4" />
            </Button>

            {/* Settings Button */}
            <Button
              onClick={() => navigate('/settings')}
              variant="ghost"
              size="xs"
              aria-label="Settings"
              className="hover:bg-background-medium no-drag"
            >
              <Gear className="w-4 h-4" />
            </Button>
          </div>
        </div>
        <Outlet />
      </SidebarInset>
    </div>
  );
};

export const AppLayout: React.FC<AppLayoutProps> = ({ setIsGoosehintsModalOpen }) => {
  return (
    <SidebarProvider>
      <AppLayoutContent setIsGoosehintsModalOpen={setIsGoosehintsModalOpen} />
    </SidebarProvider>
  );
};
