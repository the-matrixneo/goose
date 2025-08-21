import React, { useEffect, useState, useRef, useCallback } from 'react';
import { useFocusMode } from '../contexts/FocusModeContext';

/**
 * GlobalBlurOverlay Component - FIXED VERSION
 * 
 * A reusable component that provides a consistent glassmorphism effect across the application.
 * This component:
 * 1. Renders a background image from app settings
 * 2. Applies a blur effect with theme-aware styling
 * 3. Adjusts opacity based on focus mode state
 * 4. Handles theme changes automatically
 * 5. PREVENTS APP HANGING with robust error handling and timeouts
 * 
 * The component is designed to be mounted once at the application level to ensure
 * consistent styling across all views.
 */
const GlobalBlurOverlay: React.FC = () => {
  const { isInFocusMode } = useFocusMode();
  const [isDarkTheme, setIsDarkTheme] = useState(false);
  const [imageLoaded, setImageLoaded] = useState(true); // Start as true to prevent initial loading state
  const [backgroundImage, setBackgroundImage] = useState<string | null>(null);
  const [backgroundId, setBackgroundId] = useState<string>('default-gradient');
  
  // Refs for cleanup and timeout management
  const imageRef = useRef<HTMLImageElement | null>(null);
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Safe state clearing function to prevent stuck states
  const clearProcessingState = useCallback(() => {
    setImageLoaded(true);
    
    // Clear timeouts
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
    
    // Clear image ref
    if (imageRef.current) {
      imageRef.current.onload = null;
      imageRef.current.onerror = null;
      imageRef.current = null;
    }
  }, []);

  // Async image loading with timeout protection
  const loadImageSafely = useCallback(async (imageUrl: string): Promise<void> => {
    return new Promise((resolve, reject) => {
      // Clear any existing processing
      clearProcessingState();
      
      // Set loading state briefly
      setImageLoaded(false);
      
      // Create timeout for image loading (3 seconds max)
      const loadTimeout = setTimeout(() => {
        console.warn('Image loading timeout - clearing processing state');
        clearProcessingState();
        reject(new Error('Image loading timeout'));
      }, 3000);
      
      timeoutRef.current = loadTimeout;
      
      try {
        const img = new Image();
        imageRef.current = img;
        
        img.onload = () => {
          console.log("Background image loaded successfully");
          clearTimeout(loadTimeout);
          setImageLoaded(true);
          resolve();
        };
        
        img.onerror = (e) => {
          console.error("Failed to load background image:", e);
          clearTimeout(loadTimeout);
          clearProcessingState();
          reject(new Error('Failed to load image'));
        };
        
        // Use requestAnimationFrame to prevent blocking
        requestAnimationFrame(() => {
          if (img) {
            img.src = imageUrl;
          }
        });
        
      } catch (error) {
        clearTimeout(loadTimeout);
        clearProcessingState();
        reject(error);
      }
    });
  }, [clearProcessingState]);

  // Update theme detection when it changes
  useEffect(() => {
    const updateTheme = () => {
      try {
        setIsDarkTheme(document.documentElement.classList.contains('dark'));
      } catch (error) {
        console.error('Error updating theme:', error);
      }
    };
    
    // Initial check
    updateTheme();
    
    // Set up observer to detect theme changes
    const observer = new MutationObserver((mutations) => {
      try {
        mutations.forEach((mutation) => {
          if (mutation.attributeName === 'class') {
            updateTheme();
          }
        });
      } catch (error) {
        console.error('Error in theme observer:', error);
      }
    });
    
    observer.observe(document.documentElement, { attributes: true });
    
    return () => {
      observer.disconnect();
    };
  }, []);

  // Load initial background settings
  useEffect(() => {
    try {
      const savedBackground = localStorage.getItem('dashboard-background');
      const savedCustomImage = localStorage.getItem('dashboard-custom-image');
      
      if (savedBackground) {
        setBackgroundId(savedBackground);
      }
      
      if (savedBackground === 'custom-image' && savedCustomImage) {
        setBackgroundImage(savedCustomImage);
        
        // Load image asynchronously without blocking
        loadImageSafely(savedCustomImage).catch((error) => {
          console.error('Failed to load initial background image:', error);
          // Fallback to default
          setBackgroundId('default-gradient');
          setBackgroundImage(null);
        });
      }
    } catch (error) {
      console.error('Error loading background settings:', error);
      // Continue with defaults
    }
  }, [loadImageSafely]);

  // Listen for background changes with robust error handling
  useEffect(() => {
    const handleBackgroundChange = async (e: Event) => {
      try {
        const customEvent = e as CustomEvent;
        console.log("Background change requested:", customEvent.detail);
        
        const { backgroundId: newBackgroundId, customImage: newCustomImage } = customEvent.detail;
        
        // Use requestAnimationFrame to ensure smooth UI updates
        requestAnimationFrame(async () => {
          try {
            setBackgroundId(newBackgroundId);
            
            if (newBackgroundId === 'custom-image' && newCustomImage) {
              setBackgroundImage(newCustomImage);
              await loadImageSafely(newCustomImage);
            } else {
              setBackgroundImage(null);
              setImageLoaded(true);
            }
            
          } catch (error) {
            console.error('Error in background change handler:', error);
            clearProcessingState();
          }
        });
        
      } catch (error) {
        console.error('Error handling background change:', error);
        clearProcessingState();
      }
    };
    
    window.addEventListener('dashboard-background-changed', handleBackgroundChange);
    
    return () => {
      window.removeEventListener('dashboard-background-changed', handleBackgroundChange);
    };
  }, [loadImageSafely, clearProcessingState]);

  // Cleanup on unmount
  useEffect(() => {
    return () => {
      clearProcessingState();
    };
  }, [clearProcessingState]);

  // Fixed blur intensity
  const blurIntensity = 20; // Consistent blur for chat mode
  
  // Determine background color based on focus mode and theme
  const backgroundColor = isInFocusMode
    ? (isDarkTheme ? 'rgba(24, 24, 27, 0.8)' : 'rgba(245, 245, 250, 0.8)') // 80% opacity in focus mode
    : (isDarkTheme ? 'rgba(24, 24, 27, 0.5)' : 'rgba(245, 245, 250, 0.5)'); // 50% opacity in normal mode

  // Determine background style based on settings
  const getBackgroundStyle = useCallback((): React.CSSProperties => {
    try {
      // If using custom image, return image style
      if (backgroundId === 'custom-image' && backgroundImage) {
        return {
          backgroundImage: `url(${backgroundImage})`,
          backgroundSize: 'cover',
          backgroundPosition: 'center',
          backgroundRepeat: 'no-repeat',
        };
      }
      
      // Fallback to transparent for non-image backgrounds
      return {
        backgroundColor: 'transparent',
      };
    } catch (error) {
      console.error('Error getting background style:', error);
      return { backgroundColor: 'transparent' };
    }
  }, [backgroundId, backgroundImage]);

  return (
    <>
      {/* Background Image Layer - Only render if we have a custom image */}
      {backgroundId === 'custom-image' && backgroundImage && imageLoaded && (
        <div
          className="fixed inset-0 transition-opacity duration-500 ease-in-out z-[-10]"
          style={{
            ...getBackgroundStyle(),
            opacity: 1,
          }}
        />
      )}
      
      {/* Blur Overlay Layer - Always present for consistency */}
      <div 
        className="fixed inset-0 pointer-events-none transition-all duration-500 z-[-5]"
        style={{ 
          backdropFilter: `blur(${blurIntensity}px)`,
          backgroundColor: backgroundColor,
        }}
      />
    </>
  );
};

export default GlobalBlurOverlay;
