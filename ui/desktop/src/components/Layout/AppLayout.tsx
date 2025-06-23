import React from 'react';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import { SidebarProvider, SidebarInset, Sidebar } from '../ui/sidebar';
import AppSidebar from '../GooseSidebar/AppSidebar';
import { View, ViewOptions } from '../../App';

interface AppLayoutProps {
  setIsGoosehintsModalOpen?: (isOpen: boolean) => void;
}

export const AppLayout: React.FC<AppLayoutProps> = ({ setIsGoosehintsModalOpen }) => {
  const navigate = useNavigate();
  const location = useLocation();

  const setView = (view: View, viewOptions?: ViewOptions) => {
    // Convert view-based navigation to route-based navigation
    switch (view) {
      case 'chat':
        navigate('/');
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

  return (
    <SidebarProvider>
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
          <Outlet />
        </SidebarInset>
      </div>
    </SidebarProvider>
  );
};
