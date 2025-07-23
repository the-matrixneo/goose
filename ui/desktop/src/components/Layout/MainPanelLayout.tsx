import React, { useState, useRef, useCallback } from 'react';
import { Sidecar, useSidecar } from '../SidecarLayout';
import { GripVertical } from 'lucide-react';

export const MainPanelLayout: React.FC<{
  children: React.ReactNode;
  removeTopPadding?: boolean;
  backgroundColor?: string;
}> = ({ children, removeTopPadding = false, backgroundColor = 'bg-background-default' }) => {
  const sidecar = useSidecar();
  const isVisible = sidecar?.activeView && sidecar?.views.find((v) => v.id === sidecar.activeView);
  
  // State for resizing
  const [sidecarWidth, setSidecarWidth] = useState(50); // Percentage
  const [isResizing, setIsResizing] = useState(false);
  const containerRef = useRef<HTMLDivElement>(null);

  const handleMouseDown = useCallback((e: React.MouseEvent) => {
    e.preventDefault();
    setIsResizing(true);
    
    const handleMouseMove = (e: MouseEvent) => {
      if (!containerRef.current) return;
      
      const containerRect = containerRef.current.getBoundingClientRect();
      const containerWidth = containerRect.width;
      const mouseX = e.clientX - containerRect.left;
      
      // Calculate percentage, with constraints
      const newPercentage = Math.max(20, Math.min(80, (mouseX / containerWidth) * 100));
      setSidecarWidth(100 - newPercentage); // Invert because we want sidecar width
    };

    const handleMouseUp = () => {
      setIsResizing(false);
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };

    document.addEventListener('mousemove', handleMouseMove);
    document.addEventListener('mouseup', handleMouseUp);
  }, []);

  return (
    <div className={`h-dvh`}>
      {/* Padding top matches the app toolbar drag area height - can be removed for full bleed */}
      <div
        ref={containerRef}
        className={`flex ${backgroundColor} flex-1 min-w-0 h-full min-h-0 ${removeTopPadding ? '' : 'pt-[32px]'}`}
      >
        {/* Main Content Area */}
        <div 
          className="flex flex-col min-w-0 transition-all duration-300 ease-out"
          style={{ 
            width: isVisible ? `${100 - sidecarWidth}%` : '100%',
            transition: isResizing ? 'none' : 'width 300ms ease-out'
          }}
        >
          {children}
        </div>

        {/* Resize Handle */}
        {isVisible && (
          <div
            className={`flex items-center justify-center w-2 cursor-col-resize hover:bg-borderSubtle transition-colors group ${
              isResizing ? 'bg-borderProminent' : ''
            }`}
            onMouseDown={handleMouseDown}
          >
            <GripVertical 
              size={12} 
              className={`text-textSubtle group-hover:text-textStandard transition-colors ${
                isResizing ? 'text-textStandard' : ''
              }`} 
            />
          </div>
        )}

        {/* Sidecar Panel */}
        {isVisible && (
          <div 
            className="transition-all duration-300 ease-out h-full"
            style={{ 
              width: `${sidecarWidth}%`,
              transition: isResizing ? 'none' : 'width 300ms ease-out'
            }}
          >
            <Sidecar />
          </div>
        )}
      </div>
    </div>
  );
};
