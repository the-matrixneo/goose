import React from 'react';
import { Outlet, useNavigate, useLocation } from 'react-router-dom';
import { View, ViewOptions } from '../../App';
import { AppWindowMac, AppWindow } from 'lucide-react';
import { Button } from '../ui/button';
import { SidebarProvider, useSidebar } from '../ui/sidebar';
import GlobalBlurOverlay from '../GlobalBlurOverlay';
import PillSideNav from '../PillSideNav';

interface AppLayoutProps {
  setIsGoosehintsModalOpen?: (isOpen: boolean) => void;
}

// Inner component that uses useSidebar within SidebarProvider context
const AppLayoutContent: React.FC<AppLayoutProps> = ({ setIsGoosehintsModalOpen }) => {
  const navigate = useNavigate();
  const location = useLocation();
  const safeIsMacOS = (window?.electron?.platform || 'darwin') === 'darwin';
  const { isMobile, openMobile } = useSidebar();

  // Hide buttons when mobile sheet is showing
  const shouldHideButtons = isMobile && openMobile;

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

  const handleNewWindow = () => {
    window.electron.createChatWindow(
      undefined,
      window.appConfig.get('GOOSE_WORKING_DIR') as string | undefined
    );
  };

  return (
    <div className="flex flex-1 w-full relative animate-fade-in">
      {/* Global background */}
      <GlobalBlurOverlay />
      
      {/* Floating pill navigation in center */}
      <div className="absolute top-4 left-0 right-0 z-50 flex justify-center pointer-events-none">
        <div className="pointer-events-auto">
          <PillSideNav />
        </div>
      </div>
      
      {/* New Window button in top right */}
      {!shouldHideButtons && (
        <div className="absolute top-4 right-4 z-50">
          <Button
            onClick={handleNewWindow}
            className="no-drag hover:!bg-white/10 text-white"
            variant="ghost"
            size="xs"
            title="Start a new session in a new window"
          >
            {safeIsMacOS ? <AppWindowMac className="w-4 h-4" /> : <AppWindow className="w-4 h-4" />}
          </Button>
        </div>
      )}
      
      {/* Main Content */}
      <div className="w-full h-full">
        <Outlet />
      </div>
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
