import React, { useEffect, useState } from 'react';
import { useLocation } from 'react-router-dom';
import { useFocusMode } from '../contexts/FocusModeContext';
import GlobalBlurOverlay from './GlobalBlurOverlay';
import { type View, ViewOptions } from '../App';
import { cn } from '../utils';

/**
 * Hub Component
 * 
 * The Hub component serves as the initial landing page for the Goose Desktop application.
 * It provides a welcoming interface for users to start new conversations and access key features.
 * 
 * This component has been updated to use the GlobalBlurOverlay for consistent glassmorphism styling.
 */
export default function Hub({
  readyForAutoUserPrompt,
  chat,
  setChat,
  setPairChat,
  setView,
  setIsGoosehintsModalOpen,
}: {
  readyForAutoUserPrompt: boolean;
  chat: any;
  setChat: (chat: any) => void;
  setPairChat: (chat: any) => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  setIsGoosehintsModalOpen: (isOpen: boolean) => void;
}) {
  const location = useLocation();
  const { isInFocusMode, setIsInFocusMode } = useFocusMode();
  
  // Reset focus mode when returning to hub
  useEffect(() => {
    setIsInFocusMode(false);
  }, [setIsInFocusMode]);

  // Custom main layout props to override background completely
  const customMainLayoutProps = {
    backgroundColor: 'transparent',
    style: { 
      backgroundColor: 'transparent',
      background: 'transparent'
    },
  };

  return (
    <div className="flex flex-col h-full relative bg-transparent">
      <GlobalBlurOverlay />
      
      {/* Hub content goes here */}
      <div className="relative z-10 flex justify-center h-full bg-transparent">
        <div className="w-full max-w-[1000px] h-full bg-transparent">
          {/* Your existing Hub content */}
        </div>
      </div>
    </div>
  );
}
