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
    <li className="mb-1">
      <button 
        onClick={onClick}
        className={cn(
          "flex items-center gap-3 px-4 py-3 rounded-full w-full transition-all duration-200",
          isActive 
            ? "bg-white/15 text-white" 
            : "text-white/80 hover:text-white hover:bg-white/10"
        )}
      >
        <div className="text-lg">
          {icon}
        </div>
        <span className="text-sm font-medium">{label}</span>
        {isActive && (
          <ChevronRightIcon className="ml-auto text-white/70" size={16} />
        )}
      </button>
    </li>
  );
};

interface NavSectionProps {
  title?: string;
  children: React.ReactNode;
}

const NavSection: React.FC<NavSectionProps> = ({ title, children }) => {
  return (
    <div className="mb-4">
      {title && (
        <h3 className="text-xs font-medium text-white/50 uppercase tracking-wider px-4 mb-2">
          {title}
        </h3>
      )}
      <ul>{children}</ul>
    </div>
  );
};

const NavDivider: React.FC = () => {
  return <div className="h-px bg-white/10 my-3 mx-2" />;
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

  return (
    <div className="h-full flex items-center">
      {/* Collapsed Pill */}
      {!isExpanded && (
        <div 
          className="h-12 w-12 bg-gradient-to-r from-blue-600/90 to-purple-600/90 rounded-full 
                    flex items-center justify-center cursor-pointer shadow-lg hover:shadow-xl
                    transition-all duration-300 hover:scale-105 ml-3"
          onClick={() => setIsExpanded(true)}
        >
          <span className="text-white font-bold text-lg">G</span>
        </div>
      )}

      {/* Expanded Navigation */}
      {isExpanded && (
        <div className="absolute left-0 top-0 h-full min-w-64 z-50
                      bg-gradient-to-b from-gray-800/95 to-gray-900/95 backdrop-blur-xl
                      border-r border-white/10 shadow-xl animate-in slide-in-from-left duration-300">
          <div className="flex flex-col h-full">
            {/* Header with close button */}
            <div className="flex items-center justify-between px-4 py-4 border-b border-white/10">
              <div className="flex items-center">
                <div className="w-8 h-8 bg-gradient-to-r from-blue-600 to-purple-600 rounded-full flex items-center justify-center">
                  <span className="text-white font-bold text-lg">G</span>
                </div>
                <span className="ml-2 text-white font-medium text-lg">Goose</span>
              </div>
              <button 
                onClick={() => setIsExpanded(false)}
                className="text-white/70 hover:text-white p-1 rounded-full hover:bg-white/10"
              >
                <CloseIcon size={18} />
              </button>
            </div>

            {/* Navigation Content */}
            <div className="flex-1 overflow-y-auto px-2 py-4">
              <NavSection>
                <NavItem 
                  icon={<HomeIcon size={20} />} 
                  label="Home" 
                  path="/"
                  isActive={currentPath === '/'}
                  onClick={() => handleNavigation('/')}
                />
                <NavItem 
                  icon={<ChatIcon size={20} />} 
                  label="Chat" 
                  path="/pair"
                  isActive={currentPath === '/pair'}
                  onClick={() => handleNavigation('/pair')}
                />
              </NavSection>

              <NavDivider />

              <NavSection title="History">
                <NavItem 
                  icon={<HistoryIcon size={20} />} 
                  label="Sessions" 
                  path="/sessions"
                  isActive={currentPath === '/sessions'}
                  onClick={() => handleNavigation('/sessions')}
                />
                <NavItem 
                  icon={<ClockIcon size={20} />} 
                  label="Scheduler" 
                  path="/schedules"
                  isActive={currentPath === '/schedules'}
                  onClick={() => handleNavigation('/schedules')}
                />
              </NavSection>

              <NavDivider />

              <NavSection title="Tools">
                <NavItem 
                  icon={<FileIcon size={20} />} 
                  label="Recipes" 
                  path="/recipes"
                  isActive={currentPath === '/recipes'}
                  onClick={() => handleNavigation('/recipes')}
                />
                <NavItem 
                  icon={<PuzzleIcon size={20} />} 
                  label="Extensions" 
                  path="/extensions"
                  isActive={currentPath === '/extensions'}
                  onClick={() => handleNavigation('/extensions')}
                />
              </NavSection>
            </div>

            {/* Footer */}
            <div className="p-3 border-t border-white/10">
              <NavItem 
                icon={<SettingsIcon size={20} />} 
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
