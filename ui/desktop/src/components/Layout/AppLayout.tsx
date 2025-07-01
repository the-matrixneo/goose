import React from 'react';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import { SidebarProvider, SidebarInset, Sidebar, SidebarTrigger, useSidebar } from '../ui/sidebar';
import AppSidebar from '../GooseSidebar/AppSidebar';
import { View, ViewOptions } from '../../App';

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
  // const headerPadding = isSidebarOpen ? 'pl-6' : safeIsMacOS ? 'pl-20' : 'pl-6';
  const headerPadding = '';

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
      case 'extensions':
        navigate('/extensions', { state: viewOptions });
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

  return (
    <div className="flex flex-1 w-full relative animate-fade-in">
      <Sidebar variant="inset" collapsible="offcanvas">
        <AppSidebar
          onSelectSession={handleSelectSession}
          setView={setView}
          setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
          currentPath={location.pathname}
        />
      </Sidebar>
      <SidebarInset>
        <div
          className={`${headerPadding} absolute top-0 left-0 h-12 z-100 w-full flex items-center justify-between pr-6 py-1`}
        >
          <SidebarTrigger
            className={`no-drag text-text-muted hover:text-text-default hover:bg-background-muted`}
          />
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
