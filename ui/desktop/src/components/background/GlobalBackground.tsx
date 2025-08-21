import React, { useEffect, useState, useCallback, useRef } from 'react';

/**
 * GlobalBackground Component
 * 
 * A robust, non-hanging background system that:
 * 1. Handles background switching without freezing the app
 * 2. Manages proper z-index layering
 * 3. Supports gradients and custom images
 * 4. Provides smooth transitions
 * 5. Includes fail-safe mechanisms to prevent hanging
 */

interface BackgroundSettings {
  type: 'gradient' | 'image' | 'solid';
  gradient?: string;
  imageUrl?: string;
  color?: string;
  blur?: number;
  opacity?: number;
}

const DEFAULT_BACKGROUNDS = {
  'gradient-1': {
    type: 'gradient' as const,
    gradient: 'linear-gradient(135deg, #667eea 0%, #764ba2 100%)',
    blur: 20,
    opacity: 0.8,
  },
  'gradient-2': {
    type: 'gradient' as const,
    gradient: 'linear-gradient(135deg, #f093fb 0%, #f5576c 100%)',
    blur: 20,
    opacity: 0.8,
  },
  'gradient-3': {
    type: 'gradient' as const,
    gradient: 'linear-gradient(135deg, #4facfe 0%, #00f2fe 100%)',
    blur: 20,
    opacity: 0.8,
  },
  'gradient-4': {
    type: 'gradient' as const,
    gradient: 'linear-gradient(135deg, #43e97b 0%, #38f9d7 100%)',
    blur: 20,
    opacity: 0.8,
  },
  'solid-dark': {
    type: 'solid' as const,
    color: '#1a1a1a',
    opacity: 0.9,
  },
  'solid-light': {
    type: 'solid' as const,
    color: '#f5f5f5',
    opacity: 0.9,
  },
};

export const GlobalBackground: React.FC = () => {
  const [currentBackground, setCurrentBackground] = useState<BackgroundSettings>(DEFAULT_BACKGROUNDS['gradient-1']);
  const [isTransitioning, setIsTransitioning] = useState(false);
  const [isLoaded, setIsLoaded] = useState(false);
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);
  const imageRef = useRef<HTMLImageElement | null>(null);

  // Safe state update function with timeout protection
  const safeSetState = useCallback((setter: () => void, timeout: number = 5000) => {
    try {
      setter();
      
      // Clear any existing timeout
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
      
      // Set a fail-safe timeout
      timeoutRef.current = setTimeout(() => {
        setIsTransitioning(false);
        console.warn('Background transition timeout - forcing completion');
      }, timeout);
      
    } catch (error) {
      console.error('Background state update failed:', error);
      setIsTransitioning(false);
    }
  }, []);

  // Load background settings from localStorage
  useEffect(() => {
    try {
      const savedBackground = localStorage.getItem('goose-background-settings');
      if (savedBackground) {
        const parsed = JSON.parse(savedBackground);
        setCurrentBackground({ ...DEFAULT_BACKGROUNDS['gradient-1'], ...parsed });
      }
    } catch (error) {
      console.warn('Failed to load background settings:', error);
    }
    
    setIsLoaded(true);
  }, []);

  // Handle background changes from other components
  useEffect(() => {
    const handleBackgroundChange = (event: CustomEvent<BackgroundSettings>) => {
      safeSetState(() => {
        setIsTransitioning(true);
        
        // Use requestAnimationFrame for smooth transitions
        requestAnimationFrame(() => {
          try {
            const newBackground = event.detail;
            
            // If it's an image, preload it
            if (newBackground.type === 'image' && newBackground.imageUrl) {
              const img = new Image();
              imageRef.current = img;
              
              img.onload = () => {
                setCurrentBackground(newBackground);
                localStorage.setItem('goose-background-settings', JSON.stringify(newBackground));
                setIsTransitioning(false);
                if (timeoutRef.current) {
                  clearTimeout(timeoutRef.current);
                }
              };
              
              img.onerror = () => {
                console.error('Failed to load background image');
                setIsTransitioning(false);
                if (timeoutRef.current) {
                  clearTimeout(timeoutRef.current);
                }
              };
              
              img.src = newBackground.imageUrl;
            } else {
              // For gradients and solid colors, update immediately
              setCurrentBackground(newBackground);
              localStorage.setItem('goose-background-settings', JSON.stringify(newBackground));
              setIsTransitioning(false);
              if (timeoutRef.current) {
                clearTimeout(timeoutRef.current);
              }
            }
          } catch (error) {
            console.error('Background change failed:', error);
            setIsTransitioning(false);
          }
        });
      });
    };

    window.addEventListener('goose-background-change', handleBackgroundChange as EventListener);
    
    return () => {
      window.removeEventListener('goose-background-change', handleBackgroundChange as EventListener);
      if (timeoutRef.current) {
        clearTimeout(timeoutRef.current);
      }
      if (imageRef.current) {
        imageRef.current.onload = null;
        imageRef.current.onerror = null;
      }
    };
  }, [safeSetState]);

  // Generate background style
  const getBackgroundStyle = useCallback((): React.CSSProperties => {
    const { type, gradient, imageUrl, color } = currentBackground;
    
    switch (type) {
      case 'gradient':
        return {
          background: gradient || DEFAULT_BACKGROUNDS['gradient-1'].gradient,
        };
      case 'image':
        return {
          backgroundImage: imageUrl ? `url(${imageUrl})` : 'none',
          backgroundSize: 'cover',
          backgroundPosition: 'center',
          backgroundRepeat: 'no-repeat',
        };
      case 'solid':
        return {
          backgroundColor: color || DEFAULT_BACKGROUNDS['solid-dark'].color,
        };
      default:
        return {
          background: DEFAULT_BACKGROUNDS['gradient-1'].gradient,
        };
    }
  }, [currentBackground]);

  // Don't render until loaded to prevent flashing
  if (!isLoaded) {
    return null;
  }

  return (
    <>
      {/* Background Layer - Lowest z-index */}
      <div
        className="fixed inset-0 transition-all duration-500 ease-in-out"
        style={{
          zIndex: -1000,
          ...getBackgroundStyle(),
          opacity: isTransitioning ? 0.5 : 1,
        }}
      />
      
      {/* Blur Overlay Layer */}
      <div
        className="fixed inset-0 transition-all duration-500 ease-in-out pointer-events-none"
        style={{
          zIndex: -999,
          backdropFilter: `blur(${currentBackground.blur || 20}px)`,
          backgroundColor: `rgba(0, 0, 0, ${1 - (currentBackground.opacity || 0.8)})`,
          opacity: isTransitioning ? 0.5 : 1,
        }}
      />
      
      {/* Loading Overlay - Only show during transitions */}
      {isTransitioning && (
        <div
          className="fixed inset-0 bg-black/20 flex items-center justify-center transition-opacity duration-300 pointer-events-none"
          style={{ zIndex: -998 }}
        >
          <div className="bg-white/10 backdrop-blur-sm rounded-lg px-4 py-2 text-white text-sm">
            Updating background...
          </div>
        </div>
      )}
    </>
  );
};

// Helper function to change background from other components
export const changeBackground = (settings: Partial<BackgroundSettings>) => {
  const event = new CustomEvent('goose-background-change', {
    detail: settings,
  });
  window.dispatchEvent(event);
};

// Predefined background options
export const BACKGROUND_PRESETS = DEFAULT_BACKGROUNDS;

export default GlobalBackground;
