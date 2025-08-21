import React, { useState, useEffect, useRef } from 'react';
import { Card } from '../ui/card';
import { Clock, Move } from 'lucide-react';

interface Position {
  x: number;
  y: number;
}

interface ClockWidgetProps {
  initialPosition?: Position;
  onPositionChange?: (position: Position) => void;
}

export const ClockWidget: React.FC<ClockWidgetProps> = ({
  initialPosition = { x: 20, y: 20 },
  onPositionChange,
}) => {
  const [time, setTime] = useState(new Date());
  const [position, setPosition] = useState(initialPosition);
  const [isDragging, setIsDragging] = useState(false);
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 });
  const widgetRef = useRef<HTMLDivElement>(null);

  // Sync position with initialPosition prop - CRITICAL FOR STATE PERSISTENCE
  useEffect(() => {
    setPosition(initialPosition);
  }, [initialPosition]);
  // Update time every second
  useEffect(() => {
    const timer = setInterval(() => {
      setTime(new Date());
    }, 1000);

    return () => clearInterval(timer);
  }, []);

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
      const maxX = window.innerWidth - 200; // Approximate widget width
      const maxY = window.innerHeight - 100; // Approximate widget height
      
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
  }, [isDragging, dragOffset, onPositionChange]);

  const formatTime = (date: Date) => {
    return date.toLocaleTimeString([], { 
      hour: '2-digit', 
      minute: '2-digit',
      second: '2-digit',
    });
  };

  const formatDate = (date: Date) => {
    return date.toLocaleDateString([], {
      weekday: 'short',
      month: 'short',
      day: 'numeric',
    });
  };

  return (
    <Card
      ref={widgetRef}
      className={`fixed bg-background-card/20 backdrop-blur-md border border-white/10 hover:bg-background-card/30 hover:border-white/15 shadow-lg transition-all duration-200 select-none ${
        isDragging ? 'cursor-grabbing scale-105 shadow-xl' : 'cursor-grab hover:shadow-xl hover:scale-105'
      }`}
      style={{
        left: position.x,
        top: position.y,
        zIndex: 10,
        minWidth: '180px',
      }}
      onMouseDown={handleMouseDown}
    >
      <div className="p-4">
        <div className="flex items-center gap-2 mb-2">
          <Clock className="w-4 h-4 text-text-muted" />
          <Move className="w-3 h-3 text-text-muted ml-auto opacity-50" />
        </div>
        
        <div className="space-y-1">
          <div className="text-2xl font-mono font-bold text-text-standard">
            {formatTime(time)}
          </div>
          <div className="text-sm text-text-muted">
            {formatDate(time)}
          </div>
        </div>
      </div>
    </Card>
  );
};

export default ClockWidget;
