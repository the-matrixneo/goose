/**
 * Pair Component
 *
 * The Pair component represents the active conversation mode in the Goose Desktop application.
 * This is where users engage in ongoing conversations with the AI assistant after transitioning
 * from the Hub's initial welcome screen.
 *
 * Key Responsibilities:
 * - Manages active chat sessions with full message history
 * - Handles transitions from Hub with initial input processing
 * - Provides the main conversational interface for extended interactions
 * - Enables local storage persistence for conversation continuity
 * - Supports all advanced chat features like file attachments, tool usage, etc.
 *
 * Navigation Flow:
 * Hub (initial message) → Pair (active conversation) → Hub (new session)
 *
 * The Pair component is essentially a specialized wrapper around BaseChat that:
 * - Processes initial input from the Hub transition
 * - Enables conversation persistence
 * - Provides the full-featured chat experience
 *
 * Unlike Hub, Pair assumes an active conversation state and focuses on
 * maintaining conversation flow rather than onboarding new users.
 */

import { useEffect, useState } from 'react';
import { useLocation } from 'react-router-dom';
import { type View, ViewOptions } from '../App';
import { Message } from '../types/message';
import BaseChat from './BaseChat';
import 'react-toastify/dist/ReactToastify.css';

export interface ChatType {
  id: string;
  title: string;
  messageHistoryIndex: number;
  messages: Message[];
}

export default function Pair({
  chat,
  setChat,
  setView,
  setIsGoosehintsModalOpen,
}: {
  readyForAutoUserPrompt: boolean;
  chat: ChatType;
  setChat: (chat: ChatType) => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  setIsGoosehintsModalOpen: (isOpen: boolean) => void;
}) {
  const location = useLocation();
  const [hasProcessedInitialInput, setHasProcessedInitialInput] = useState(false);

  // Handle initial input from hub page
  useEffect(() => {
    const initialInput = location.state?.initialInput;
    const fromHub = location.state?.fromHub;

    if (fromHub && initialInput && !hasProcessedInitialInput && chat.messages.length === 0) {
      // Auto-submit the initial input - this would need to be handled by BaseChat
      setHasProcessedInitialInput(true);

      // Clear the location state to prevent re-processing
      window.history.replaceState({}, '', '/pair');
    }
  }, [location.state, hasProcessedInitialInput, chat.messages.length]);

  // Custom content before messages
  const renderBeforeMessages = () => (
    <div>{/* Any Pair-specific content before messages can go here */}</div>
  );

  // Custom chat input props for Pair-specific behavior
  const customChatInputProps = {
    // Any Pair-specific chat input customizations can go here
  };

  return (
    <BaseChat
      chat={chat}
      setChat={setChat}
      setView={setView}
      setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
      enableLocalStorage={true} // Enable local storage for Pair mode
      renderBeforeMessages={renderBeforeMessages}
      customChatInputProps={customChatInputProps}
      contentClassName="pl-6 px-4 pb-16 pt-2" // Add Pair-specific padding
    />
  );
}
