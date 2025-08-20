import React, { useEffect } from 'react';

/**
 * BackgroundImageFix Component
 * 
 * A clean implementation to ensure background image and overlay display correctly.
 * This component injects CSS directly into the document head to ensure proper z-index
 * and positioning of background elements.
 */
const BackgroundImageFix: React.FC = () => {
  useEffect(() => {
    // Create a style element
    const styleElement = document.createElement('style');
    styleElement.id = 'background-image-fix-styles';
    
    // Define CSS that ensures the background image and overlay are displayed correctly
    const css = `
      /* Reset any existing background styles that might interfere */
      .fixed.inset-0.-z-10,
      .fixed.inset-0.-z-9,
      .fixed.inset-0.-z-8,
      .fixed.inset-0.-z-5,
      .fixed.inset-0.-z-1,
      [style*="z-index: -10"],
      [style*="z-index: -9"],
      [style*="z-index: -8"],
      [style*="z-index: -5"],
      [style*="z-index: -1"] {
        z-index: auto !important;
      }
      
      /* Remove any background gradients from the app container */
      #root > div,
      .bg-background-muted,
      .animate-gradient-slow,
      [class*="bg-gradient"] {
        background: none !important;
        background-image: none !important;
      }
      
      /* Make headers transparent in the sessions view */
      .sticky.top-0.z-10.bg-background-default\/80,
      .sticky.top-0.z-10.bg-background-default,
      .sticky.top-0.z-10,
      .bg-background-default\/80.backdrop-blur-md {
        background: transparent !important;
        backdrop-filter: none !important;
        -webkit-backdrop-filter: none !important;
      }
      
      /* Make session history headers transparent */
      .text-text-muted {
        background: transparent !important;
      }
      
      /* Root background container - lowest layer */
      #root-background-container {
        position: fixed;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        z-index: -1000;
        pointer-events: none;
      }
      
      /* Background image layer */
      #app-background-image {
        position: absolute;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        background-image: url('/background.jpg');
        background-size: cover;
        background-position: center;
        background-repeat: no-repeat;
        z-index: -900;
      }
      
      /* Blur overlay layer */
      #app-background-overlay {
        position: absolute;
        top: 0;
        right: 0;
        bottom: 0;
        left: 0;
        backdrop-filter: blur(20px);
        -webkit-backdrop-filter: blur(20px);
        background-color: rgba(24, 24, 27, 0.5);
        transition: background-color 0.5s ease;
        z-index: -800;
      }
      
      /* Ensure app content is above background */
      #root > div {
        position: relative;
        z-index: 1;
      }
    `;
    
    // Add the CSS to the style element
    styleElement.textContent = css;
    
    // Append the style element to the head
    document.head.appendChild(styleElement);
    
    // Create the background container and elements
    const backgroundContainer = document.createElement('div');
    backgroundContainer.id = 'root-background-container';
    
    const backgroundImage = document.createElement('div');
    backgroundImage.id = 'app-background-image';
    
    const backgroundOverlay = document.createElement('div');
    backgroundOverlay.id = 'app-background-overlay';
    
    // Assemble the elements
    backgroundContainer.appendChild(backgroundImage);
    backgroundContainer.appendChild(backgroundOverlay);
    
    // Insert the background container as the first child of the body
    document.body.insertBefore(backgroundContainer, document.body.firstChild);
    
    // Find and remove any gradient backgrounds in the application
    const removeGradientBackgrounds = () => {
      // Target elements with gradient backgrounds
      const gradientElements = document.querySelectorAll('[class*="bg-gradient"], .animate-gradient-slow');
      gradientElements.forEach(element => {
        if (element instanceof HTMLElement) {
          element.style.background = 'none';
          element.style.backgroundImage = 'none';
        }
      });
      
      // Specifically target GlobalBackground components
      const globalBackgrounds = document.querySelectorAll('.fixed.inset-0.-z-10');
      globalBackgrounds.forEach(element => {
        if (element instanceof HTMLElement) {
          element.style.display = 'none';
        }
      });
      
      // Make session headers transparent
      const sessionHeaders = document.querySelectorAll('.sticky.top-0.z-10, .bg-background-default\\/80.backdrop-blur-md');
      sessionHeaders.forEach(element => {
        if (element instanceof HTMLElement) {
          element.style.background = 'transparent';
          element.style.backdropFilter = 'none';
          element.style.webkitBackdropFilter = 'none';
        }
      });
    };
    
    // Run immediately and set up an observer to catch any dynamically added elements
    removeGradientBackgrounds();
    
    const observer = new MutationObserver((mutations) => {
      removeGradientBackgrounds();
    });
    
    observer.observe(document.body, { 
      childList: true,
      subtree: true,
      attributes: true,
      attributeFilter: ['class', 'style']
    });
    
    // Cleanup function
    return () => {
      // Disconnect the observer
      observer.disconnect();
      
      // Remove the style element
      const styleToRemove = document.getElementById('background-image-fix-styles');
      if (styleToRemove) {
        document.head.removeChild(styleToRemove);
      }
      
      // Remove the background container
      const containerToRemove = document.getElementById('root-background-container');
      if (containerToRemove) {
        document.body.removeChild(containerToRemove);
      }
    };
  }, []);
  
  // This component doesn't render anything visible
  return null;
};

export default BackgroundImageFix;
