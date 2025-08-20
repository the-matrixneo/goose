import React, { useState, useEffect } from 'react';
import { Session } from '../../sessions';
import { Calendar, MessageSquareText, Target, Folder, MoreHorizontal } from 'lucide-react';
import { formatMessageTimestamp } from '../../utils/timeUtils';
import { Button } from '../ui/button';
import { cn } from '../../utils';

interface StaggeredSessionItemProps {
  session: Session;
  onEditClick: (session: Session) => void;
  index: number;
  groupIndex: number; // Add group index to stagger across groups
}

export const StaggeredSessionItem: React.FC<StaggeredSessionItemProps> = ({ 
  session, 
  onEditClick,
  index,
  groupIndex
}) => {
  const [isVisible, setIsVisible] = useState(false);
  
  useEffect(() => {
    // Calculate a more natural staggered delay
    // Base delay + group delay + position within group delay
    const baseDelay = 30; // Base delay in ms
    const groupDelay = groupIndex * 100; // Additional delay per group
    const positionDelay = index * 80; // Delay per position within group
    
    // Apply a slight randomization to make it feel more organic
    const randomFactor = Math.random() * 50; // Random factor between 0-50ms
    
    // Total delay calculation
    const totalDelay = baseDelay + groupDelay + positionDelay + randomFactor;
    
    const timeout = setTimeout(() => {
      setIsVisible(true);
    }, totalDelay);
    
    return () => clearTimeout(timeout);
  }, [index, groupIndex]);
  
  const handleEditClick = (e: React.MouseEvent) => {
    e.stopPropagation(); // Prevent card click
    onEditClick(session);
  };
  
  return (
    <div 
      className={cn(
        "bg-background-default/40 backdrop-blur-sm hover:bg-background-default/60 border border-borderSubtle rounded-xl p-4 cursor-pointer transition-all duration-500 transform hover:shadow-md",
        isVisible ? "opacity-100 translate-y-0" : "opacity-0 translate-y-8"
      )}
      style={{
        transitionTimingFunction: "cubic-bezier(0.34, 1.56, 0.64, 1)", // Spring-like easing
        transitionDelay: isVisible ? "0ms" : `${index * 30}ms` // Additional delay on initial animation
      }}
      onClick={() => {
        // Use the API directly instead of window.electron
        if (window.electron && window.electron.resumeSession) {
          window.electron.resumeSession(session.id);
        } else {
          console.warn("resumeSession function not available on window.electron");
          // Fallback method if available
          if (window.location && typeof window.location.href === 'string') {
            window.location.href = `/?resumeSessionId=${encodeURIComponent(session.id)}`;
          }
        }
      }}
    >
      <div className="flex justify-between items-start">
        <div className="flex-1 min-w-0">
          <h3 className="text-base mb-1 pr-6 break-words">
            {session.metadata.description || session.id}
          </h3>

          <div className="flex items-center text-text-muted text-xs mb-1">
            <Calendar className="w-3 h-3 mr-1 flex-shrink-0" />
            <span>{formatMessageTimestamp(Date.parse(session.modified) / 1000)}</span>
          </div>
          <div className="flex items-center text-text-muted text-xs mb-1">
            <Folder className="w-3 h-3 mr-1 flex-shrink-0" />
            <span className="truncate">{session.metadata.working_dir}</span>
          </div>
        </div>

        <div className="flex items-center">
          <Button
            size="icon"
            variant="ghost"
            className="h-8 w-8 rounded-full"
            onClick={handleEditClick}
          >
            <MoreHorizontal className="h-4 w-4" />
          </Button>
        </div>
      </div>

      <div className="flex items-center justify-between mt-1 pt-2">
        <div className="flex items-center space-x-3 text-xs text-text-muted">
          <div className="flex items-center">
            <MessageSquareText className="w-3 h-3 mr-1" />
            <span className="font-mono">{session.metadata.message_count}</span>
          </div>
          {session.metadata.total_tokens !== null && (
            <div className="flex items-center">
              <Target className="w-3 h-3 mr-1" />
              <span className="font-mono">{session.metadata.total_tokens.toLocaleString()}</span>
            </div>
          )}
        </div>
      </div>
    </div>
  );
};
