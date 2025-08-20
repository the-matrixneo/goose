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

  // Navigation items configuration with colors
  const navItems = [
    { icon: <HomeIcon size={18} />, label: 'Home', path: '/', color: 'bg-blue-600' },
    { icon: <ChatIcon size={18} />, label: 'Chat', path: '/pair', color: 'bg-purple-600' },
    { icon: <HistoryIcon size={18} />, label: 'History', path: '/sessions', color: 'bg-amber-600' },
    { icon: <FileIcon size={18} />, label: 'Recipes', path: '/recipes', color: 'bg-green-600' },
    { icon: <SettingsIcon size={18} />, label: 'Settings', path: '/settings', color: 'bg-gray-600' },
  ];

  // Find the current active item to determine pill color
  const activeItem = navItems.find(item => item.path === currentPath) || navItems[0];
  
  // Get current mode label
  const currentModeLabel = activeItem.label;

  return (
    <div className="relative">
      {/* Collapsed Pill */}
      {!isExpanded && (
        <div 
          className={`h-10 ${activeItem.color} rounded-full 
                    flex items-center cursor-pointer shadow-lg
                    transition-all duration-300 hover:shadow-xl px-4 space-x-2`}
          onClick={() => setIsExpanded(true)}
        >
          <span className="text-white font-medium text-sm">{currentModeLabel}</span>
        </div>
      )}

      {/* Expanded Navigation */}
      {isExpanded && (
        <div className="absolute left-1/2 transform -translate-x-1/2 top-0 z-50
                      bg-gradient-to-b from-gray-800/95 to-gray-900/95 backdrop-blur-xl
                      border border-white/10 shadow-xl rounded-lg animate-in fade-in duration-200">
          <div className="flex flex-col p-2">
            {/* Header with close button */}
            <div className="flex items-center justify-between px-2 py-1 mb-1">
              <span className="text-white font-medium text-sm">Navigation</span>
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
            </div>
          </div>
        </div>
      )}
    </div>
  );
};

export default PillSideNav;
