/**
 * Hub Component
 *
 * The Hub is the main landing page and entry point for the Goose Desktop application.
 * It serves as the welcome screen and initial chat interface where users start new conversations.
 *
 * Key Responsibilities:
 * - Displays SessionInsights when no active conversation exists
 * - Shows a "Continue Session" dialog when resuming an existing session
 * - Handles the initial message submission that transitions users to the Pair view
 * - Provides access to recipe management and .goosehints configuration
 * - Manages the transition between empty state and active conversation state
 *
 * Navigation Flow:
 * Hub (landing) → Pair (active conversation) → Hub (new session)
 *
 * The Hub uses BaseChat as its foundation but customizes the header, content areas,
 * and input behavior to create a welcoming onboarding experience.
 */

import { useState, useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import FlappyGoose from './FlappyGoose';
import { type View, ViewOptions } from '../App';
import { Message } from '../types/message';
import { SessionInsights } from './sessions/SessionsInsights';
import { Button } from './ui/button';
import { Idea } from './icons';
import { Tooltip, TooltipContent, TooltipTrigger } from './ui/Tooltip';
import { Bot, Save, Send } from 'lucide-react';
import { useChatContext } from '../contexts/ChatContext';
import BaseChat from './BaseChat';
import { Recipe } from '../recipe';
import 'react-toastify/dist/ReactToastify.css';

export interface ChatType {
  id: string;
  title: string;
  messageHistoryIndex: number;
  messages: Message[];
}

export default function Hub({
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
  const { resetChat } = useChatContext();
  const [showGame, setShowGame] = useState(false);
  const [forceShowInsights, setForceShowInsights] = useState(true);
  const [isInPairMode, setIsInPairMode] = useState(false);

  // Get recipeConfig directly from appConfig
  const recipeConfig = window.appConfig.get('recipeConfig') as Recipe | null;

  // Effect to detect direct navigation to the hub page
  useEffect(() => {
    // Check if we're on the hub page (root route)
    const isHubPage = location.pathname === '/';

    // If we're on the hub page, force showing insights only if there are no messages
    // This allows resumed sessions to display immediately
    if (isHubPage) {
      setForceShowInsights(chat.messages.length === 0);
      setIsInPairMode(false);
    }
  }, [location.pathname, chat.messages.length]);

  // Handle message submission callback (called after message is submitted)
  const handleMessageSubmit = (message: string) => {
    if (message.trim() && (chat.messages.length === 0 || forceShowInsights)) {
      // Navigate to pair page after message is submitted
      setTimeout(() => {
        setView('pair', { disableAnimation: true });
      }, 100);
    }
  };

  // Custom header for Hub
  const renderHeader = () => (
    <div className="h-12 flex items-center justify-between absolute">
      <div className="flex items-center justify-end pr-4">
        {chat.messages.length > 0 && (
          <>
            {setIsGoosehintsModalOpen && (
              <Tooltip delayDuration={500}>
                <TooltipTrigger asChild className="w-full">
                  <Button
                    onClick={() => setIsGoosehintsModalOpen(true)}
                    className="px-3"
                    variant="ghost"
                    size="sm"
                    shape="round"
                  >
                    <div className="flex gap-2 items-center text-text-default">
                      <Idea className="w-4 h-4" />
                    </div>
                  </Button>
                </TooltipTrigger>
                <TooltipContent side="right">
                  <p>Customize instructions</p>
                </TooltipContent>
              </Tooltip>
            )}

            {recipeConfig ? (
              <>
                <Tooltip delayDuration={500}>
                  <TooltipTrigger asChild className="w-full">
                    <Button
                      onClick={() => {
                        window.electron.createChatWindow(
                          undefined,
                          undefined,
                          undefined,
                          undefined,
                          recipeConfig as Recipe,
                          'recipeEditor'
                        );
                      }}
                      className="px-3"
                      variant="ghost"
                    >
                      <div className="flex gap-2 items-center text-text-default">
                        <Send className="w-4 h-4" />
                        View recipe
                      </div>
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent side="right">
                    <p>View the recipe you're using</p>
                  </TooltipContent>
                </Tooltip>

                <Tooltip delayDuration={500}>
                  <TooltipTrigger asChild className="w-full">
                    <Button className="px-3" variant="ghost" size="sm" shape="round">
                      <div className="flex gap-2 items-center text-text-default">
                        <Save className="w-4 h-4" />
                        Save recipe
                      </div>
                    </Button>
                  </TooltipTrigger>
                  <TooltipContent side="right">
                    <p>Save this recipe for reuse</p>
                  </TooltipContent>
                </Tooltip>
              </>
            ) : (
              <Tooltip delayDuration={500}>
                <TooltipTrigger asChild className="w-full">
                  <Button
                    onClick={() => {
                      window.electron.logInfo('Make Agent button clicked');
                      window.dispatchEvent(new CustomEvent('make-agent-from-chat'));
                    }}
                    className="px-3"
                    variant="ghost"
                    size="sm"
                    shape="round"
                  >
                    <div className="flex gap-2 items-center text-text-default">
                      <Bot className="w-4 h-4" />
                    </div>
                  </Button>
                </TooltipTrigger>
                <TooltipContent side="right">
                  <p>Make a custom agent you can share or reuse</p>
                </TooltipContent>
              </Tooltip>
            )}
          </>
        )}
      </div>
    </div>
  );

  // Custom content before messages
  const renderBeforeMessages = () => (
    <>
      {/* Session Insights - always show on hub page regardless of message count */}
      {(chat.messages.length === 0 || forceShowInsights) && !isInPairMode && <SessionInsights />}
    </>
  );

  // Custom chat input props for Hub-specific behavior
  const customChatInputProps = {
    // Remove the handleSubmit override since we're using the callback approach
    messages: forceShowInsights ? [] : undefined,
  };

  // Show session continuation UI when there's an active session but insights are shown
  if (forceShowInsights && chat.messages.length > 0) {
    return (
      <div className="flex flex-col h-full">
        <div className="flex-1 flex items-center justify-center">
          <div className="mx-6 mb-6 p-4 rounded-xl border border-borderSubtle bg-background-default max-w-md">
            <div className="flex flex-col items-center gap-4">
              <div className="text-center">
                <h3 className="text-lg font-medium">You have an active session</h3>
                <p className="text-sm text-textSubtle mt-1">
                  Would you like to continue or start fresh?
                </p>
              </div>
              <div className="flex gap-3">
                <Button
                  onClick={() => {
                    // Navigate to pair page with the session
                    setView('pair');
                  }}
                  variant="default"
                >
                  Continue Session
                </Button>
                <Button
                  onClick={() => {
                    resetChat();
                    setForceShowInsights(false);
                  }}
                  variant="outline"
                >
                  Start New Session
                </Button>
              </div>
            </div>
          </div>
        </div>
        {showGame && <FlappyGoose onClose={() => setShowGame(false)} />}
      </div>
    );
  }

  return (
    <div>
      <BaseChat
        chat={chat}
        setChat={setChat}
        setView={setView}
        setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
        onMessageSubmit={handleMessageSubmit}
        renderHeader={renderHeader}
        renderBeforeMessages={renderBeforeMessages}
        customChatInputProps={customChatInputProps}
        disableSearch={true}
      />
      {showGame && <FlappyGoose onClose={() => setShowGame(false)} />}
    </div>
  );
}
