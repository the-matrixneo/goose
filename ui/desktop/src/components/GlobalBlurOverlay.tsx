import React, { useEffect, useState } from 'react';
import { useFocusMode } from '../contexts/FocusModeContext';

/**
 * GlobalBlurOverlay Component
 * 
 * A reusable component that provides a consistent glassmorphism effect across the application.
 * This component:
 * 1. Renders a background image from app settings
 * 2. Applies a blur effect with theme-aware styling
 * 3. Adjusts opacity based on focus mode state
 * 4. Handles theme changes automatically
 * 
 * The component is designed to be mounted once at the application level to ensure
 * consistent styling across all views.
 */
const GlobalBlurOverlay: React.FC = () => {
  const { isInFocusMode } = useFocusMode();
  const [isDarkTheme, setIsDarkTheme] = useState(false);
  const [imageLoaded, setImageLoaded] = useState(false);
  const [backgroundImage, setBackgroundImage] = useState<string | null>(null);
  const [backgroundId, setBackgroundId] = useState<string>('default-gradient');
  const [isProcessingChange, setIsProcessingChange] = useState(false);
  
  // Update theme detection when it changes
  useEffect(() => {
    const updateTheme = () => {
      setIsDarkTheme(document.documentElement.classList.contains('dark'));
    };
    
    // Initial check
    updateTheme();
    
    // Set up observer to detect theme changes
    const observer = new MutationObserver((mutations) => {
      mutations.forEach((mutation) => {
        if (mutation.attributeName === 'class') {
          updateTheme();
        }
      });
    });
    
    observer.observe(document.documentElement, { attributes: true });
    
    // Load background settings
    const savedBackground = localStorage.getItem('dashboard-background');
    const savedCustomImage = localStorage.getItem('dashboard-custom-image');
    
    if (savedBackground) {
      setBackgroundId(savedBackground);
    }
    
    if (savedBackground === 'custom-image' && savedCustomImage) {
      setBackgroundImage(savedCustomImage);
      
      // Preload the custom image
      const img = new Image();
      img.onload = () => {
        console.log("Custom background image loaded successfully");
        setImageLoaded(true);
      };
      img.onerror = (e) => {
        console.error("Failed to load custom background image:", e);
      };
      img.src = savedCustomImage;
    } else {
      // If not using custom image, mark as loaded
      setImageLoaded(true);
    }
    
    // Listen for background changes
    const handleBackgroundChange = (e: CustomEvent) => {
      console.log("Background changed:", e.detail);
      
      // Set processing flag to prevent UI freezes
      setIsProcessingChange(true);
      
      // Use setTimeout to defer processing to next tick
      setTimeout(() => {
        try {
          setBackgroundId(e.detail.backgroundId);
          
          if (e.detail.backgroundId === 'custom-image' && e.detail.customImage) {
            setBackgroundImage(e.detail.customImage);
            
            // For custom images, we need to wait for them to load
            if (e.detail.customImage !== backgroundImage) {
              setImageLoaded(false);
              const img = new Image();
              img.onload = () => {
                console.log("New custom background image loaded successfully");
                setImageLoaded(true);
                setIsProcessingChange(false);
              };
              img.onerror = (e) => {
                console.error("Failed to load new custom background image:", e);
                setImageLoaded(true); // Still mark as loaded to prevent UI freeze
                setIsProcessingChange(false);
              };
              img.src = e.detail.customImage;
            } else {
              // If it's the same image, no need to reload
              setImageLoaded(true);
              setIsProcessingChange(false);
            }
          } else {
            setBackgroundImage(null);
            setImageLoaded(true);
            setIsProcessingChange(false);
          }
        } catch (error) {
          console.error("Error handling background change:", error);
          setImageLoaded(true); // Ensure we don't get stuck in loading state
          setIsProcessingChange(false);
        }
      }, 0);
    };
    
    window.addEventListener('dashboard-background-changed', handleBackgroundChange as EventListener);
    
    return () => {
      observer.disconnect();
      window.removeEventListener('dashboard-background-changed', handleBackgroundChange as EventListener);
    };
  }, [backgroundImage]);

  // Fixed blur intensity
  const blurIntensity = 20; // Consistent blur for chat mode
  
  // Determine background color based on focus mode and theme
  // Using more grey-tinted overlays to match the home page
  const backgroundColor = isInFocusMode
    ? (isDarkTheme ? 'rgba(24, 24, 27, 0.8)' : 'rgba(245, 245, 250, 0.8)') // 80% opacity in focus mode
    : (isDarkTheme ? 'rgba(24, 24, 27, 0.5)' : 'rgba(245, 245, 250, 0.5)'); // 50% opacity in normal mode

  // Determine background style based on settings
  const getBackgroundStyle = () => {
    // If using custom image, return image style
    if (backgroundId === 'custom-image' && backgroundImage) {
      return {
        backgroundImage: `url(${backgroundImage})`,
        backgroundSize: 'cover',
        backgroundPosition: 'center',
        backgroundRepeat: 'no-repeat',
      };
    }
    
    // Fallback to default image
    return {
      backgroundImage: `url('/background.jpg')`,
      backgroundSize: 'cover',
      backgroundPosition: 'center',
      backgroundRepeat: 'no-repeat',
    };
  };

  // Create a div element to append to the body
  useEffect(() => {
    // Create background and blur overlay elements
    const backgroundDiv = document.createElement('div');
    const blurDiv = document.createElement('div');
    
    // Set styles for background image
    Object.assign(backgroundDiv.style, {
      position: 'fixed',
      top: '0',
      right: '0',
      bottom: '0',
      left: '0',
      zIndex: '-8',
      ...getBackgroundStyle(),
      opacity: imageLoaded ? '1' : '0',
      transition: 'opacity 0.5s ease-in-out',
    });
    
    // Set styles for blur overlay
    Object.assign(blurDiv.style, {
      position: 'fixed',
      top: '0',
      right: '0',
      bottom: '0',
      left: '0',
      zIndex: '-5',
      backdropFilter: `blur(${blurIntensity}px)`,
      backgroundColor: backgroundColor,
      transition: 'background-color 0.5s ease',
      pointerEvents: 'none',
    });
    
    // Append elements to body
    document.body.appendChild(backgroundDiv);
    document.body.appendChild(blurDiv);
    
    // Debug info
    if (process.env.NODE_ENV === 'development') {
      const debugDiv = document.createElement('div');
      Object.assign(debugDiv.style, {
        position: 'fixed',
        bottom: '16px',
        right: '16px',
        backgroundColor: 'rgba(0, 0, 0, 0.5)',
        color: 'white',
        padding: '8px',
        borderRadius: '4px',
        fontSize: '12px',
        zIndex: '50',
      });
      
      debugDiv.innerHTML = `
        Image Loaded: ${imageLoaded ? 'Yes' : 'No'}<br />
        Background ID: ${backgroundId}<br />
        Custom Image: ${backgroundImage ? 'Yes' : 'No'}<br />
        Dark Theme: ${isDarkTheme ? 'Yes' : 'No'}<br />
        Focus Mode: ${isInFocusMode ? 'Yes' : 'No'}<br />
        Overlay Color: ${backgroundColor}<br />
        Processing Change: ${isProcessingChange ? 'Yes' : 'No'}
      `;
      
      document.body.appendChild(debugDiv);
    }
    
    // Cleanup function
    return () => {
      document.body.removeChild(backgroundDiv);
      document.body.removeChild(blurDiv);
      
      if (process.env.NODE_ENV === 'development') {
        const debugElement = document.body.querySelector('[style*="position: fixed"][style*="bottom: 16px"][style*="right: 16px"]');
        if (debugElement) {
          document.body.removeChild(debugElement);
        }
      }
    };
  }, [backgroundId, backgroundImage, imageLoaded, isDarkTheme, isInFocusMode, backgroundColor, isProcessingChange]);

  // Loading overlay to prevent interaction during background changes
  if (isProcessingChange) {
    return (
      <div className="fixed inset-0 bg-background-default/50 backdrop-blur-sm flex items-center justify-center z-50">
        <div className="bg-background-default p-4 rounded-lg shadow-lg flex flex-col items-center">
          <div className="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-textStandard mb-2"></div>
          <p className="text-sm text-textStandard">Updating background...</p>
        </div>
      </div>
    );
  }

  // Return null since we're appending directly to the body
  return null;
};

export default GlobalBlurOverlay;
