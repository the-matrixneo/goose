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
  
  const sizeClasses = {
    small: 'w-16 h-16',
    medium: 'w-20 h-20',
    large: 'w-24 h-24',
  };

  // Generate unique button ID
  const buttonId = `circular-tile-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;

  // Listen for docking events
  useEffect(() => {
    const handleDocking = (_event: any, data: { buttonId: string }) => {
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

  const handleDragStart = (e: React.DragEvent<HTMLButtonElement>) => {
    setIsDragging(true);
    
    // Create a drag image using the circular tile itself
    const dragImage = e.currentTarget.cloneNode(true) as HTMLElement;
    dragImage.style.transform = 'none';
    dragImage.style.opacity = '0.8';
    document.body.appendChild(dragImage);
    
    // Set the drag image
    e.dataTransfer.setDragImage(dragImage, 40, 40); // Center the drag image
    
    // Set drag data for different drop targets
    e.dataTransfer.setData('text/plain', 'Goose Assistant');
    e.dataTransfer.setData('text/uri-list', 'goose://assistant');
    
    // Custom data for our desktop shortcut creation
    e.dataTransfer.setData('application/x-goose-tile', JSON.stringify({
      type: 'goose-assistant',
      title: 'Goose Assistant',
      description: 'AI Assistant Tile',
      iconPath: circularTileImage
    }));
    
    // Clean up drag image after a short delay
    setTimeout(() => {
      if (document.body.contains(dragImage)) {
        document.body.removeChild(dragImage);
      }
    }, 100);
  };

  const handleDragEnd = async (e: React.DragEvent<HTMLButtonElement>) => {
    setIsDragging(false);
    
    // Check if the drag ended outside the window (potential floating button creation)
    const isOutsideWindow = 
      e.clientX < 0 || 
      e.clientY < 0 || 
      e.clientX > window.innerWidth || 
      e.clientY > window.innerHeight;
    
    if (isOutsideWindow && window.electron?.createFloatingButton) {
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
          
          // Calculate position where the drag ended (in screen coordinates)
          const screenX = e.screenX;
          const screenY = e.screenY;
          
          const result = await window.electron.createFloatingButton({
            buttonId,
            x: screenX - 40, // Center the button on cursor
            y: screenY - 40,
            size,
            imageData
          });
          
          if (result.success) {
            setFloatingButtonId(buttonId);
            console.log('Floating button created:', result.windowId);
          } else {
            console.error('Failed to create floating button:', result.error);
          }
        };
        
        img.src = circularTileImage;
      } catch (error) {
        console.error('Error creating floating button:', error);
      }
    }
  };

  const handleClick = (e: React.MouseEvent<HTMLButtonElement>) => {
    // Prevent click if we were dragging
    if (isDragging) {
      e.preventDefault();
      return;
    }
    onClick?.();
  };

  return (
    <button
      onClick={handleClick}
      draggable={true}
      onDragStart={handleDragStart}
      onDragEnd={handleDragEnd}
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
  );
}