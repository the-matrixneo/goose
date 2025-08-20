import React, { useState } from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { cn } from '../utils';
import { 
  Home as HomeIcon,
  MessageSquare as ChatIcon,
  Clock as ClockIcon,
  FileText as FileIcon,
  Puzzle as PuzzleIcon,
  Settings as SettingsIcon,
  History as HistoryIcon,
  ChevronRight as ChevronRightIcon,
  X as CloseIcon
} from 'lucide-react';

interface NavItemProps {
  icon: React.ReactNode;
  label: string;
  path: string;
  isActive?: boolean;
  onClick: () => void;
}

const NavItem: React.FC<NavItemProps> = ({ 
  icon, 
  label, 
  isActive = false,
  onClick
}) => {
  return (
    <button 
      onClick={onClick}
      className={cn(
        "flex items-center gap-2 px-3 py-2 rounded-full transition-all duration-200 w-full",
        isActive 
          ? "bg-white/15 text-white" 
          : "text-white/80 hover:text-white hover:bg-white/10"
      )}
    >
      <div className="text-lg">
        {icon}
      </div>
      <span className="text-sm font-medium">{label}</span>
    </button>
  );
};

export const PillSideNav: React.FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const currentPath = location.pathname;
  const [isExpanded, setIsExpanded] = useState(false);

  const handleNavigation = (path: string) => {
    navigate(path);
    setIsExpanded(false);
  };

  // Navigation items configuration
  const navItems = [
    { icon: <HomeIcon size={18} />, label: 'Home', path: '/' },
    { icon: <ChatIcon size={18} />, label: 'Chat', path: '/pair' },
    { icon: <HistoryIcon size={18} />, label: 'History', path: '/sessions' },
    { icon: <FileIcon size={18} />, label: 'Recipes', path: '/recipes' },
  ];

  return (
    <div className="relative">
      {/* Collapsed Pill */}
      {!isExpanded && (
        <div 
          className="h-10 bg-gradient-to-r from-blue-600/90 to-purple-600/90 rounded-full 
                    flex items-center cursor-pointer shadow-lg
                    transition-all duration-300 hover:shadow-xl px-3 space-x-2"
          onClick={() => setIsExpanded(true)}
        >
          <span className="text-white font-bold text-lg">G</span>
          <span className="text-white text-sm font-medium">Menu</span>
        </div>
      )}

      {/* Expanded Navigation */}
      {isExpanded && (
        <div className="absolute left-0 top-0 z-50
                      bg-gradient-to-b from-gray-800/95 to-gray-900/95 backdrop-blur-xl
                      border border-white/10 shadow-xl rounded-lg animate-in fade-in duration-200">
          <div className="flex flex-col p-2">
            {/* Header with close button */}
            <div className="flex items-center justify-between px-2 py-1 mb-1">
              <div className="flex items-center">
                <div className="w-6 h-6 bg-gradient-to-r from-blue-600 to-purple-600 rounded-full flex items-center justify-center">
                  <span className="text-white font-bold text-sm">G</span>
                </div>
                <span className="ml-2 text-white font-medium text-sm">Goose</span>
              </div>
              <button 
                onClick={() => setIsExpanded(false)}
                className="text-white/70 hover:text-white p-1 rounded-full hover:bg-white/10"
              >
                <CloseIcon size={16} />
              </button>
            </div>

            {/* Navigation Items */}
            <div className="space-y-1 min-w-40">
              {navItems.map((item) => (
                <NavItem 
                  key={item.path}
                  icon={item.icon} 
                  label={item.label} 
                  path={item.path}
                  isActive={currentPath === item.path}
                  onClick={() => handleNavigation(item.path)}
                />
              ))}
              <div className="h-px bg-white/10 my-1" />
              <NavItem 
                icon={<SettingsIcon size={18} />} 
                label="Settings" 
                path="/settings"
                isActive={currentPath === '/settings'}
                onClick={() => handleNavigation('/settings')}
              />
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default PillSideNav;
