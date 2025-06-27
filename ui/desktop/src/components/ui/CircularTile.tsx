import circularTileImage from '../../assets/circular-tile-bg.png';
import { Goose } from '../icons/Goose';
import { useState, useEffect } from 'react';

interface CircularTileProps {
  onClick?: () => void;
  className?: string;
  size?: 'small' | 'medium' | 'large';
}

export default function CircularTile({ onClick, className = '', size = 'medium' }: CircularTileProps) {
  const [isDragging, setIsDragging] = useState(false);
  const [floatingButtonId, setFloatingButtonId] = useState<string | null>(null);
  const [dragPreview, setDragPreview] = useState<{ x: number; y: number; visible: boolean }>({ x: 0, y: 0, visible: false });
  
  const sizeClasses = {
    small: 'w-16 h-16',
    medium: 'w-20 h-20',
    large: 'w-24 h-24',
  };

  // Generate unique button ID
  const buttonId = `circular-tile-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;

  // Listen for docking events
  useEffect(() => {
    const handleDocking = (_event: any, ...args: unknown[]) => {
      const data = args[0] as { buttonId: string };
      if (data.buttonId === floatingButtonId) {
        setFloatingButtonId(null);
        console.log('Button docked back to window');
      }
    };

    if (window.electron?.on) {
      window.electron.on('floating-button-dock', handleDocking);
    }

    return () => {
      if (window.electron?.off) {
        window.electron.off('floating-button-dock', handleDocking);
      }
    };
  }, [floatingButtonId]);

  // Mouse-based drag handling with visual feedback
  const handleMouseDown = (e: React.MouseEvent<HTMLButtonElement>) => {
    e.preventDefault();
    const startPos = { x: e.clientX, y: e.clientY };
    let hasDragged = false;

    console.log('Mouse down at:', startPos);

    const handleMouseMove = (moveEvent: MouseEvent) => {
      const deltaX = Math.abs(moveEvent.clientX - startPos.x);
      const deltaY = Math.abs(moveEvent.clientY - startPos.y);
      
      // Start dragging if moved more than 5px
      if (deltaX > 5 || deltaY > 5) {
        if (!hasDragged) {
          hasDragged = true;
          setIsDragging(true);
          console.log('Dragging started, current pos:', { x: moveEvent.clientX, y: moveEvent.clientY });
        }
        
        // Update drag preview position
        setDragPreview({
          x: moveEvent.clientX,
          y: moveEvent.clientY,
          visible: true
        });
      }
    };

    const handleMouseUp = async (upEvent: MouseEvent) => {
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
      
      // Hide drag preview
      setDragPreview({ x: 0, y: 0, visible: false });
      
      console.log('Mouse up at:', { x: upEvent.clientX, y: upEvent.clientY });
      console.log('Window dimensions:', { width: window.innerWidth, height: window.innerHeight });
      console.log('Had dragged:', hasDragged);
      
      if (hasDragged) {
        // For testing: always create floating button when dragging
        // Later we can add back the outside detection
        console.log('Creating floating button (test mode - always create when dragging)');
        
        if (window.electron?.createFloatingButton) {
          console.log('Creating floating button...');
          try {
            // Convert the background image to base64 data URL
            const canvas = document.createElement('canvas');
            const ctx = canvas.getContext('2d');
            const img = new Image();
            
            img.onload = async () => {
              canvas.width = img.width;
              canvas.height = img.height;
              ctx?.drawImage(img, 0, 0);
              const imageData = canvas.toDataURL();
              
              // Calculate screen position using screen coordinates
              const screenX = upEvent.screenX || (window.screenX + upEvent.clientX);
              const screenY = upEvent.screenY || (window.screenY + upEvent.clientY);
              
              console.log('Screen position:', { screenX, screenY });
              
              const result = await window.electron.createFloatingButton({
                buttonId,
                x: screenX - 40, // Center the button
                y: screenY - 40,
                size,
                imageData
              });
              
              if (result.success) {
                setFloatingButtonId(buttonId);
                console.log('Floating button created successfully:', result.windowId);
              } else {
                console.error('Failed to create floating button:', result.error);
              }
            };
            
            img.onerror = (error) => {
              console.error('Failed to load image:', error);
            };
            
            img.src = circularTileImage;
          } catch (error) {
            console.error('Error creating floating button:', error);
          }
        } else {
          console.error('window.electron.createFloatingButton not available');
        }
      } else {
        console.log('Was a click, not a drag');
        // This was a click, not a drag - trigger click after a small delay
        setTimeout(() => {
          onClick?.();
        }, 0);
      }
      
      setIsDragging(false);
    };

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
  };

  const handleClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    // Prevent default click if we handled it in mouse events
    if (isDragging) {
      e.preventDefault();
      return;
    }
  };

  return (
    <>
      <button
        onClick={handleClick}
        onMouseDown={handleMouseDown}
        className={`
          ${sizeClasses[size]}
          rounded-full
          transition-all duration-300 ease-out
          transform hover:scale-105 active:scale-95
          border-2 border-white/50
          relative overflow-hidden
          group
          cursor-grab active:cursor-grabbing
          ${isDragging ? 'opacity-50' : ''}
          ${floatingButtonId ? 'opacity-30 border-dashed border-blue-400' : ''}
          ${className}
        `}
        style={{
          backgroundImage: `url(${circularTileImage})`,
          backgroundSize: 'contain',
          backgroundPosition: 'center',
          backgroundRepeat: 'no-repeat',
          boxShadow: '0px 2px 4px 0px rgba(255, 255, 255, 0.8) inset, 0px 8px 16px 0px rgba(255, 255, 255, 0.3) inset, -2px 2px 8px 0px rgba(255, 255, 255, 0.4) inset, 0px 4px 4px 0px rgba(0, 0, 0, 0.25)',
        }}
      >
        {/* Hover overlay effect */}
        <div className="absolute inset-0 bg-white/10 opacity-0 group-hover:opacity-100 transition-opacity duration-300 rounded-full" />
        
        {/* Subtle shine effect */}
        <div className="absolute top-1 left-1 w-2 h-2 bg-white/30 rounded-full opacity-60" />
        
        {/* 16x16 Goose logo in bottom right corner inside the circle */}
        <div className="absolute bottom-2 w-4 h-4 text-white/90 drop-shadow-md z-10" style={{ right: '11px' }}>
          <Goose className="w-4 h-4" />
        </div>
        
        {/* Optional overlay for better contrast if needed */}
        <div className="absolute inset-0 bg-gradient-to-br from-transparent via-transparent to-black/5 rounded-full" />
      </button>

      {/* Drag Preview */}
      {dragPreview.visible && (
        <div
          className="fixed pointer-events-none z-50"
          style={{
            left: dragPreview.x - 40,
            top: dragPreview.y - 40,
            transform: 'scale(0.8)',
            opacity: 0.8
          }}
        >
          <div
            className={`
              ${sizeClasses[size]}
              rounded-full
              border-2 border-white/50
              relative overflow-hidden
              shadow-2xl
            `}
            style={{
              backgroundImage: `url(${circularTileImage})`,
              backgroundSize: 'contain',
              backgroundPosition: 'center',
              backgroundRepeat: 'no-repeat',
              boxShadow: '0px 2px 4px 0px rgba(255, 255, 255, 0.8) inset, 0px 8px 16px 0px rgba(255, 255, 255, 0.3) inset, -2px 2px 8px 0px rgba(255, 255, 255, 0.4) inset, 0px 4px 4px 0px rgba(0, 0, 0, 0.25)',
            }}
          >
            {/* Goose logo in drag preview */}
            <div className="absolute bottom-2 w-4 h-4 text-white/90 drop-shadow-md z-10" style={{ right: '11px' }}>
              <Goose className="w-4 h-4" />
            </div>
          </div>
        </div>
      )}
    </>
  );
}