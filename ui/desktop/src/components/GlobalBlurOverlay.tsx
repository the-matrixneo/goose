import React, { useEffect, useState } from 'react';
import { useFocusMode } from '../contexts/FocusModeContext';

/**
 * GlobalBlurOverlay Component
 * 
 * A reusable component that provides a consistent glassmorphism effect across the application.
 * This component:
 * 1. Applies an animated gradient background
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

  // Fixed blur intensity and background color based on theme
  const blurIntensity = 20; // Consistent blur for chat mode
  
  // Determine background color based on focus mode and theme
  const backgroundColor = isInFocusMode
    ? (isDarkTheme ? 'rgba(0, 0, 0, 0.9)' : 'rgba(255, 255, 255, 0.9)') // 90% opacity in focus mode
    : (isDarkTheme ? 'rgba(0, 0, 0, 0.7)' : 'rgba(255, 255, 255, 0.7)'); // 70% opacity in normal mode

  // Animated gradient background for dark and light themes
  const darkGradient = `
    linear-gradient(135deg, 
      rgba(26, 26, 46, 1) 0%, 
      rgba(22, 33, 62, 1) 25%, 
      rgba(15, 52, 96, 1) 50%,
      rgba(22, 33, 62, 1) 75%,
      rgba(26, 26, 46, 1) 100%)
  `;
  
  const lightGradient = `
    linear-gradient(135deg, 
      rgba(245, 247, 250, 1) 0%, 
      rgba(228, 232, 240, 1) 25%, 
      rgba(195, 207, 226, 1) 50%,
      rgba(228, 232, 240, 1) 75%,
      rgba(245, 247, 250, 1) 100%)
  `;

  return (
    <>
      {/* Animated gradient background */}
      <div 
        className="fixed inset-0 -z-10 animate-gradient-slow" 
        style={{
          background: isDarkTheme ? darkGradient : lightGradient,
          backgroundSize: '400% 400%',
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
