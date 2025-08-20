import React from 'react';
import { X } from 'lucide-react';
import { cn } from '../../utils';

interface TabPillProps {
  label: string;
  isActive: boolean;
  onClick: () => void;
  onClose?: () => void;
  isNewChat?: boolean;
}

const TabPill: React.FC<TabPillProps> = ({ 
  label, 
  isActive, 
  onClick, 
  onClose,
  isNewChat = false 
}) => {
  return (
    <div 
      className={cn(
        "flex items-center gap-2 px-3 py-1.5 rounded-md cursor-pointer transition-all",
        "text-sm font-medium whitespace-nowrap max-w-[180px] overflow-hidden text-ellipsis",
        isActive 
          ? "bg-background-default text-textProminent shadow-sm" 
          : "hover:bg-background-muted text-textStandard",
        isNewChat && "bg-blue-500/10 hover:bg-blue-500/20"
      )}
      onClick={onClick}
    >
      <span className="truncate">{label}</span>
      
      {!isNewChat && onClose && (
        <button 
          className="opacity-50 hover:opacity-100 rounded-full p-0.5 hover:bg-background-muted"
          onClick={(e) => {
            e.stopPropagation();
            onClose();
          }}
        >
          <X className="h-3 w-3" />
        </button>
      )}
    </div>
  );
};

export default TabPill;
