import { useEffect, useState, useContext, createContext } from 'react';
import { useLocation } from 'react-router-dom';
import { type View, ViewOptions } from '../App';
import { Message } from '../types/message';
import BaseChat from './BaseChat';
import 'react-toastify/dist/ReactToastify.css';

// Context for sharing current model info
const CurrentModelContext = createContext<{ model: string; mode: string } | null>(null);
export const useCurrentModelInfo = () => useContext(CurrentModelContext);

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

  // Custom content before messages for loading state
  const renderBeforeMessages = () => (
    <div className="px-6">
      {/* Loading indicator will be handled by BaseChat's isLoading state */}
    </div>
  );

  // Custom main layout props for Pair-specific styling
  const customMainLayoutProps = {
    className: 'pl-6 px-4 pb-16 pt-2',
  };

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
      customMainLayoutProps={customMainLayoutProps}
      customChatInputProps={customChatInputProps}
    />
  );
}
