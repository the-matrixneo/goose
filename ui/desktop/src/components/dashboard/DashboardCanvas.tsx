import React, { useState, useRef, useCallback, useEffect } from 'react';
import { DashboardWidget } from './DashboardWidget';
import { WidgetPosition, WidgetData } from '../../types/dashboard';

interface DashboardCanvasProps {
  widgets: WidgetData[];
  onWidgetMove: (id: string, position: WidgetPosition) => void;
  onWidgetResize: (id: string, size: { width: number; height: number }) => void;
}

interface BackgroundOption {
  id: string;
  name: string;
  type: 'gradient' | 'image' | 'solid';
  value: string;
}

const DEFAULT_BACKGROUNDS: BackgroundOption[] = [
  {
    id: 'default-gradient',
    name: 'Default Gradient',
    type: 'gradient',
    value: `
      radial-gradient(circle at 20% 80%, rgba(100, 100, 110, 0.25) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(120, 120, 125, 0.22) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(90, 95, 100, 0.18) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(0, 0, 0, 0.02) 0%, 
        rgba(0, 0, 0, 0.04) 25%, 
        rgba(0, 0, 0, 0.02) 50%, 
        rgba(0, 0, 0, 0.06) 75%, 
        rgba(0, 0, 0, 0.03) 100%
      )
    `,
  },
  {
    id: 'blue-gradient',
    name: 'Ocean Blue',
    type: 'gradient',
    value: `
      radial-gradient(circle at 20% 80%, rgba(59, 130, 246, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(99, 102, 241, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(147, 197, 253, 0.08) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(59, 130, 246, 0.02) 0%, 
        rgba(99, 102, 241, 0.04) 25%, 
        rgba(147, 197, 253, 0.02) 50%, 
        rgba(59, 130, 246, 0.06) 75%, 
        rgba(99, 102, 241, 0.03) 100%
      )
    `,
  },
  {
    id: 'purple-gradient',
    name: 'Purple Haze',
    type: 'gradient',
    value: `
      radial-gradient(circle at 20% 80%, rgba(147, 51, 234, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(168, 85, 247, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(196, 181, 253, 0.08) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(147, 51, 234, 0.02) 0%, 
        rgba(168, 85, 247, 0.04) 25%, 
        rgba(196, 181, 253, 0.02) 50%, 
        rgba(147, 51, 234, 0.06) 75%, 
        rgba(168, 85, 247, 0.03) 100%
      )
    `,
  },
  {
    id: 'green-gradient',
    name: 'Forest Green',
    type: 'gradient',
    value: `
      radial-gradient(circle at 20% 80%, rgba(34, 197, 94, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(22, 163, 74, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(134, 239, 172, 0.08) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(34, 197, 94, 0.02) 0%, 
        rgba(22, 163, 74, 0.04) 25%, 
        rgba(134, 239, 172, 0.02) 50%, 
        rgba(34, 197, 94, 0.06) 75%, 
        rgba(22, 163, 74, 0.03) 100%
      )
    `,
  },
  {
    id: 'solid-dark',
    name: 'Dark',
    type: 'solid',
    value: 'rgba(0, 0, 0, 0.1)',
  },
  {
    id: 'solid-light',
    name: 'Light',
    type: 'solid',
    value: 'rgba(255, 255, 255, 0.05)',
  },
];

export function DashboardCanvas({ widgets, onWidgetMove }: DashboardCanvasProps) {
  const canvasRef = useRef<HTMLDivElement>(null);
  const [draggedWidget, setDraggedWidget] = useState<string | null>(null);
  const [dragOffset, setDragOffset] = useState({ x: 0, y: 0 });
  const [dragPosition, setDragPosition] = useState<{ x: number; y: number } | null>(null);
  const animationFrameRef = useRef<number | null>(null);
  
  // Background state
  const [currentBackground, setCurrentBackground] = useState<string>('default-gradient');
  const [customImage, setCustomImage] = useState<string | null>(null);
  const [showDotOverlay, setShowDotOverlay] = useState(true);

  // Load background settings on mount
  useEffect(() => {
    const savedBackground = localStorage.getItem('dashboard-background') || 'default-gradient';
    const savedCustomImage = localStorage.getItem('dashboard-custom-image');
    const savedDotOverlay = localStorage.getItem('dashboard-dot-overlay');

    setCurrentBackground(savedBackground);
    setCustomImage(savedCustomImage);
    setShowDotOverlay(savedDotOverlay !== 'false');
  }, []);

  // Listen for background changes
  useEffect(() => {
    const handleBackgroundChange = (event: CustomEvent) => {
      const { backgroundId, customImage: newCustomImage } = event.detail;
      setCurrentBackground(backgroundId);
      setCustomImage(newCustomImage);
    };

    const handleDotOverlayChange = (event: CustomEvent) => {
      const { enabled } = event.detail;
      setShowDotOverlay(enabled);
    };

    window.addEventListener('dashboard-background-changed', handleBackgroundChange as EventListener);
    window.addEventListener('dashboard-dot-overlay-changed', handleDotOverlayChange as EventListener);

    return () => {
      window.removeEventListener('dashboard-background-changed', handleBackgroundChange as EventListener);
      window.removeEventListener('dashboard-dot-overlay-changed', handleDotOverlayChange as EventListener);
    };
  }, []);

  // Get the current background style
  const getBackgroundStyle = () => {
    if (currentBackground === 'custom-image' && customImage) {
      return {
        backgroundImage: `url(${customImage})`,
        backgroundSize: 'cover',
        backgroundPosition: 'center',
        backgroundRepeat: 'no-repeat',
        opacity: 0.3,
      };
    }

    const backgroundOption = DEFAULT_BACKGROUNDS.find(bg => bg.id === currentBackground);
    if (backgroundOption) {
      if (backgroundOption.type === 'gradient') {
        return {
          background: backgroundOption.value,
          backgroundSize: '400% 400%',
        };
      } else {
        return {
          background: backgroundOption.value,
        };
      }
    }

    // Fallback to default
    return {
      background: DEFAULT_BACKGROUNDS[0].value,
      backgroundSize: '400% 400%',
    };
  };

  const handleMouseDown = useCallback((e: React.MouseEvent, widgetId: string) => {
    e.preventDefault();
    if (!canvasRef.current) return;
    
    const rect = canvasRef.current.getBoundingClientRect();
    const widget = widgets.find(w => w.id === widgetId);
    if (!widget) return;

    const offsetX = e.clientX - rect.left - widget.position.x;
    const offsetY = e.clientY - rect.top - widget.position.y;

    setDraggedWidget(widgetId);
    setDragOffset({ x: offsetX, y: offsetY });
    setDragPosition({ x: widget.position.x, y: widget.position.y });

    // Add cursor style to body to show dragging state
    document.body.style.cursor = 'grabbing';
    document.body.style.userSelect = 'none';
  }, [widgets]);

  const handleMouseMove = useCallback((e: MouseEvent) => {
    if (!draggedWidget || !canvasRef.current) return;
    
    e.preventDefault();
    
    const rect = canvasRef.current.getBoundingClientRect();
    const widget = widgets.find(w => w.id === draggedWidget);
    if (!widget) return;

    const newX = Math.max(0, Math.min(rect.width - widget.size.width, e.clientX - rect.left - dragOffset.x));
    const newY = Math.max(0, Math.min(rect.height - widget.size.height, e.clientY - rect.top - dragOffset.y));
    
    // Use requestAnimationFrame for smooth updates
    if (animationFrameRef.current) {
      cancelAnimationFrame(animationFrameRef.current);
    }
    
    animationFrameRef.current = requestAnimationFrame(() => {
      setDragPosition({ x: newX, y: newY });
    });
  }, [draggedWidget, dragOffset, widgets]);

  const handleMouseUp = useCallback(() => {
    if (draggedWidget && dragPosition) {
      // Commit the final position
      onWidgetMove(draggedWidget, dragPosition);
    }
    
    setDraggedWidget(null);
    setDragOffset({ x: 0, y: 0 });
    setDragPosition(null);
    
    // Reset cursor styles
    document.body.style.cursor = '';
    document.body.style.userSelect = '';
    
    if (animationFrameRef.current) {
      cancelAnimationFrame(animationFrameRef.current);
      animationFrameRef.current = null;
    }
  }, [draggedWidget, dragPosition, onWidgetMove]);

  // Use document-level event listeners for smoother dragging
  useEffect(() => {
    if (draggedWidget) {
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
      
      return () => {
        document.removeEventListener('mousemove', handleMouseMove);
        document.removeEventListener('mouseup', handleMouseUp);
      };
    }
    return () => {}; // Return empty cleanup function when no draggedWidget
  }, [draggedWidget, handleMouseMove, handleMouseUp]);

  // Clean up on unmount
  useEffect(() => {
    return () => {
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
      document.body.style.cursor = '';
      document.body.style.userSelect = '';
    };
  }, []);

  return (
    <div 
      ref={canvasRef}
      className="relative w-full h-full overflow-hidden"
    >
      {/* Dynamic background */}
      <div 
        className={`absolute inset-0 ${currentBackground !== 'custom-image' ? 'animate-gradient-slow' : ''}`}
        style={getBackgroundStyle()}
      />
      
      {/* Dot pattern overlay */}
      {showDotOverlay && (
        <div 
          className="absolute inset-0 opacity-10 dark:opacity-20"
          style={{
            backgroundImage: `radial-gradient(circle, rgba(0, 0, 0, 0.4) 1px, transparent 1px)`,
            backgroundSize: '24px 24px',
            backgroundPosition: '12px 12px',
          }}
        />
      )}
      
      {/* Widgets */}
      {widgets.map((widget) => {
        const isDragging = draggedWidget === widget.id;
        const position = isDragging && dragPosition ? dragPosition : widget.position;
        
        return (
          <DashboardWidget
            key={widget.id}
            widget={{ ...widget, position }}
            onMouseDown={(e) => handleMouseDown(e, widget.id)}
            isDragging={isDragging}
          />
        );
      })}
    </div>
  );
}
