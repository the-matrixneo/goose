import React, { useEffect, useState } from 'react';
import { useFocusMode } from '../contexts/FocusModeContext';

/**
 * GlobalBlurOverlay Component
 * 
 * A reusable component that provides a consistent glassmorphism effect across the application.
 * This component:
 * 1. Applies a simple solid color background
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
    
    return () => {
      observer.disconnect();
    };
  }, []);

  // Fixed blur intensity
  const blurIntensity = 20; // Consistent blur for chat mode
  
  // Determine background color based on focus mode and theme
  const backgroundColor = isInFocusMode
    ? (isDarkTheme ? 'rgba(0, 0, 0, 0.9)' : 'rgba(255, 255, 255, 0.9)') // 90% opacity in focus mode
    : (isDarkTheme ? 'rgba(0, 0, 0, 0.7)' : 'rgba(255, 255, 255, 0.7)'); // 70% opacity in normal mode

  // Simple solid background colors
  const bgColor = isDarkTheme ? '#1a1a2e' : '#f5f7fa';

  return (
    <>
      {/* Simple solid background */}
      <div 
        className="fixed inset-0 -z-10" 
        style={{
          backgroundColor: bgColor,
        }}
      />
      
      {/* Fixed blur overlay - always present with consistent intensity */}
      <div 
        className="fixed inset-0 -z-5 pointer-events-none transition-colors duration-500"
        style={{ 
          backdropFilter: `blur(${blurIntensity}px)`,
          backgroundColor: backgroundColor
        }}
      />
    </>
  );
};

export default GlobalBlurOverlay;
