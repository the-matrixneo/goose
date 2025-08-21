import React, { useState, useEffect, useRef } from 'react';
import { Card } from '../ui/card';
import { Clock, Move, CheckCircle, Circle, AlertCircle } from 'lucide-react';

interface Position {
  x: number;
  y: number;
}

interface QueuedTask {
  id: string;
  title: string;
  status: 'pending' | 'running' | 'completed' | 'failed';
  scheduledTime?: Date;
  progress?: number;
}

interface QueuedTasksWidgetProps {
  initialPosition?: Position;
  onPositionChange?: (position: Position) => void;
  tasks?: QueuedTask[];
}

export const QueuedTasksWidget: React.FC<QueuedTasksWidgetProps> = ({
  initialPosition = { x: 220, y: 20 },
  onPositionChange,
  tasks = [],
}) => {
  const [position, setPosition] = useState(initialPosition);
  const [isDragging, setIsDragging] = useState(false);
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 });
  const [isExpanded, setIsExpanded] = useState(false);
  const widgetRef = useRef<HTMLDivElement>(null);

  // Mock tasks for demonstration
  const [mockTasks] = useState<QueuedTask[]>([
    {
      id: '1',
      title: 'Process large dataset',
      status: 'running',
      progress: 65,
      scheduledTime: new Date(Date.now() + 5 * 60 * 1000), // 5 minutes from now
    },
    {
      id: '2',
      title: 'Generate report',
      status: 'pending',
      scheduledTime: new Date(Date.now() + 15 * 60 * 1000), // 15 minutes from now
    },
    {
      id: '3',
      title: 'Backup database',
      status: 'completed',
    },
  ]);

  const activeTasks = tasks.length > 0 ? tasks : mockTasks;

  // Handle mouse down for dragging
  const handleMouseDown = (e: React.MouseEvent) => {
    if (!widgetRef.current) return;
    
    const rect = widgetRef.current.getBoundingClientRect();
    setDragOffset({
      x: e.clientX - rect.left,
      y: e.clientY - rect.top,
    });
    setIsDragging(true);
  };

  // Handle mouse move for dragging
  useEffect(() => {
    const handleMouseMove = (e: MouseEvent) => {
      if (!isDragging) return;

      const newPosition = {
        x: e.clientX - dragOffset.x,
        y: e.clientY - dragOffset.y,
      };

      // Keep widget within viewport bounds
      const maxX = window.innerWidth - 300; // Approximate widget width
      const maxY = window.innerHeight - (isExpanded ? 300 : 100); // Approximate widget height
      
      newPosition.x = Math.max(0, Math.min(maxX, newPosition.x));
      newPosition.y = Math.max(0, Math.min(maxY, newPosition.y));

      setPosition(newPosition);
      onPositionChange?.(newPosition);
    };

    const handleMouseUp = () => {
      setIsDragging(false);
    };

    if (isDragging) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
    }

    return () => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [isDragging, dragOffset, isExpanded, onPositionChange]);

  const getStatusIcon = (status: QueuedTask['status']) => {
    switch (status) {
      case 'completed':
        return <CheckCircle className="w-4 h-4 text-green-500" />;
      case 'running':
        return <div className="w-4 h-4 border-2 border-blue-500 border-t-transparent rounded-full animate-spin" />;
      case 'failed':
        return <AlertCircle className="w-4 h-4 text-red-500" />;
      default:
        return <Circle className="w-4 h-4 text-text-muted" />;
    }
  };

  const formatTimeRemaining = (scheduledTime?: Date) => {
    if (!scheduledTime) return null;
    
    const now = new Date();
    const diff = scheduledTime.getTime() - now.getTime();
    
    if (diff <= 0) return 'Now';
    
    const minutes = Math.floor(diff / (1000 * 60));
    const hours = Math.floor(minutes / 60);
    
    if (hours > 0) {
      return `${hours}h ${minutes % 60}m`;
    }
    return `${minutes}m`;
  };

  const pendingCount = activeTasks.filter(task => task.status === 'pending').length;
  const runningCount = activeTasks.filter(task => task.status === 'running').length;

  return (
    <Card
      ref={widgetRef}
      className={`fixed bg-background-default/80 backdrop-blur-md border border-white/10 shadow-lg transition-all duration-200 select-none ${
        isDragging ? 'cursor-grabbing scale-105 shadow-xl' : 'cursor-grab hover:shadow-xl hover:scale-105'
      }`}
      style={{
        left: position.x,
        top: position.y,
        zIndex: 50,
        minWidth: '280px',
        maxWidth: '320px',
      }}
      onMouseDown={handleMouseDown}
    >
      <div className="p-4">
        {/* Header */}
        <div 
          className="flex items-center gap-2 mb-2 cursor-pointer"
          onClick={(e) => {
            e.stopPropagation();
            setIsExpanded(!isExpanded);
          }}
        >
          <Clock className="w-4 h-4 text-text-muted" />
          <span className="text-sm font-medium text-text-standard">
            Queued Tasks ({pendingCount + runningCount})
          </span>
          <Move className="w-3 h-3 text-text-muted ml-auto opacity-50" />
        </div>

        {/* Summary */}
        <div className="flex gap-4 text-xs text-text-muted mb-3">
          {runningCount > 0 && (
            <span className="flex items-center gap-1">
              <div className="w-2 h-2 bg-blue-500 rounded-full animate-pulse" />
              {runningCount} running
            </span>
          )}
          {pendingCount > 0 && (
            <span className="flex items-center gap-1">
              <div className="w-2 h-2 bg-yellow-500 rounded-full" />
              {pendingCount} pending
            </span>
          )}
        </div>

        {/* Expanded Task List */}
        {isExpanded && (
          <div className="space-y-2 max-h-48 overflow-y-auto">
            {activeTasks.map((task) => (
              <div
                key={task.id}
                className="flex items-center gap-2 p-2 bg-background-muted/50 rounded-lg"
              >
                {getStatusIcon(task.status)}
                <div className="flex-1 min-w-0">
                  <div className="text-sm text-text-standard truncate">
                    {task.title}
                  </div>
                  {task.status === 'running' && task.progress && (
                    <div className="w-full bg-background-muted rounded-full h-1 mt-1">
                      <div
                        className="bg-blue-500 h-1 rounded-full transition-all duration-300"
                        style={{ width: `${task.progress}%` }}
                      />
                    </div>
                  )}
                  {task.scheduledTime && task.status === 'pending' && (
                    <div className="text-xs text-text-muted">
                      in {formatTimeRemaining(task.scheduledTime)}
                    </div>
                  )}
                </div>
              </div>
            ))}
          </div>
        )}

        {/* Quick Actions */}
        {!isExpanded && runningCount > 0 && (
          <div className="text-xs text-text-muted">
            Click to expand and view details
          </div>
        )}
      </div>
    </Card>
  );
};

export default QueuedTasksWidget;
