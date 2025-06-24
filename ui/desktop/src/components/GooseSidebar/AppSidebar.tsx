import React, { useEffect, useState, useCallback, useRef } from 'react';
import { Folder, FileText, Clock, MessageCircle, Home, Loader2 } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { motion, AnimatePresence } from 'framer-motion';
import {
  SidebarContent,
  SidebarFooter,
  SidebarMenu,
  SidebarMenuItem,
  SidebarMenuButton,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarGroupContent,
} from '../ui/sidebar';
import { Button } from '../ui/button';
import { ChatSmart, Time, Gear, LinkedIn, Youtube, Discord } from '../icons';
import { ViewOptions, View } from '../../App';
import { useConfig } from '../ConfigContext';
import { Recipe } from '../../recipe';
import { saveRecipe, generateRecipeFilename } from '../../recipe/recipeStorage';
import { Tooltip, TooltipContent, TooltipTrigger } from '../ui/Tooltip';
import ThemeSelector from './ThemeSelector';
import GooseLogo from '../GooseLogo';
import { useChatContext } from '../../contexts/ChatContext';
import { fetchSessions, type Session } from '../../sessions';

interface SidebarProps {
  onSelectSession: (sessionId: string) => void;
  refreshTrigger?: number;
  children?: React.ReactNode;
  setIsGoosehintsModalOpen?: (isOpen: boolean) => void;
  setView?: (view: View, viewOptions?: ViewOptions) => void;
  currentPath?: string;
}

// Main Sidebar Component
const AppSidebar: React.FC<SidebarProps> = ({ setIsGoosehintsModalOpen, setView, currentPath }) => {
  const navigate = useNavigate();
  const [isVisible, setIsVisible] = useState(false);
  const [showSaveDialog, setShowSaveDialog] = useState(false);
  const [saveRecipeName, setSaveRecipeName] = useState('');
  const [saveGlobal, setSaveGlobal] = useState(true);
  const [saving, setSaving] = useState(false);
  const [recentSessions, setRecentSessions] = useState<Session[]>([]);
  const { remove } = useConfig();
  const { hasActiveSession, resetChat } = useChatContext();

  const refreshTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    // Trigger animation after a small delay
    const timer = setTimeout(() => {
      setIsVisible(true);
    }, 100);
    return () => clearTimeout(timer);
  }, []);

  // Debounced refresh function
  const debouncedRefresh = useCallback(() => {
    console.log('AppSidebar: Debounced refresh triggered');
    // Clear any existing timeout
    if (refreshTimeoutRef.current) {
      clearTimeout(refreshTimeoutRef.current);
    }

    // Set new timeout - reduced to 200ms for faster response
    refreshTimeoutRef.current = setTimeout(() => {
      console.log('AppSidebar: Executing debounced refresh');
      loadRecentSessions();
      refreshTimeoutRef.current = null;
    }, 200);
  }, []);

  // Cleanup timeout on unmount
  useEffect(() => {
    return () => {
      if (refreshTimeoutRef.current) {
        clearTimeout(refreshTimeoutRef.current);
      }
    };
  }, []);

  // Load recent sessions
  const loadRecentSessions = async () => {
    try {
      const sessions = await fetchSessions();
      // Take only the last 5 sessions
      setRecentSessions(sessions.slice(0, 5));
    } catch (error) {
      console.error('Failed to load recent sessions:', error);
      setRecentSessions([]);
    }
  };

  useEffect(() => {
    console.log('AppSidebar: Initial load');
    loadRecentSessions();
  }, []);

  // Add effect to listen for session creation events
  useEffect(() => {
    const handleSessionCreated = () => {
      console.log('AppSidebar: Session created event received');
      // Immediately refresh to show the new session
      loadRecentSessions();
    };

    const handleMessageStreamFinish = () => {
      console.log('AppSidebar: Message stream finished event received');
      // Always refresh when message stream finishes
      debouncedRefresh();
    };

    // Listen for custom events that indicate a session was created
    window.addEventListener('session-created', handleSessionCreated);

    // Also listen for message stream finish events
    window.addEventListener('message-stream-finished', handleMessageStreamFinish);

    return () => {
      window.removeEventListener('session-created', handleSessionCreated);
      window.removeEventListener('message-stream-finished', handleMessageStreamFinish);
    };
  }, [debouncedRefresh]);

  const handleSaveRecipe = async () => {
    if (!saveRecipeName.trim()) {
      return;
    }

    setSaving(true);
    try {
      const currentRecipeConfig = window.appConfig.get('recipeConfig');

      if (!currentRecipeConfig || typeof currentRecipeConfig !== 'object') {
        throw new Error('No recipe configuration found');
      }

      const recipe = currentRecipeConfig as Recipe;
      if (!recipe.title || !recipe.description || !recipe.instructions) {
        throw new Error('Invalid recipe configuration: missing required fields');
      }

      const filePath = await saveRecipe(recipe, {
        name: saveRecipeName.trim(),
        global: saveGlobal,
      });

      console.log(`Recipe saved to: ${filePath}`);

      setShowSaveDialog(false);
      setSaveRecipeName('');

      window.electron.showNotification({
        title: 'Recipe Saved',
        body: `Recipe "${saveRecipeName}" has been saved successfully.`,
      });
    } catch (error) {
      console.error('Failed to save recipe:', error);

      window.electron.showNotification({
        title: 'Save Failed',
        body: `Failed to save recipe: ${error instanceof Error ? error.message : 'Unknown error'}`,
      });
    } finally {
      setSaving(false);
    }
  };

  const handleSaveRecipeClick = () => {
    const currentRecipeConfig = window.appConfig.get('recipeConfig');

    if (currentRecipeConfig && typeof currentRecipeConfig === 'object') {
      const recipe = currentRecipeConfig as Recipe;
      const suggestedName = generateRecipeFilename(recipe);
      setSaveRecipeName(suggestedName);
      setShowSaveDialog(true);
    }
  };

  const recipeConfig = window.appConfig.get('recipeConfig');

  // Helper function to check if a path is active
  const isActivePath = (path: string) => {
    return currentPath === path;
  };

  return (
    <>
      <SidebarContent className="pr-3">
        {/* <SidebarHeader>
          <div className="flex items-center gap-2 pt-12 pb-4">
            <GooseLogo size="small" />
            <span className="text-base">codename goose</span>
          </div>
        </SidebarHeader> */}

        {/* Menu */}
        <div className="px-1 py-0 pt-14 space-y-2 relative">
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton
                onClick={() => {
                  // If we're not on the chat page and have an active session, just navigate back to chat
                  if (currentPath !== '/' && hasActiveSession) {
                    navigate('/');
                  } else if (hasActiveSession) {
                    // If we're already on the chat page and have an active session, create a new session
                    resetChat();
                    navigate('/');
                  } else {
                    // Navigate to home if no active session
                    navigate('/');
                  }
                }}
                isActive={isActivePath('/')}
                tooltip={
                  currentPath !== '/' && hasActiveSession
                    ? 'Return to chat'
                    : hasActiveSession
                      ? 'Create a new session'
                      : 'Go back to the main chat screen'
                }
                className="w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200"
              >
                <Home className="w-4 h-4" />
                <span>Home</span>
              </SidebarMenuButton>
            </SidebarMenuItem>

            <SidebarMenuItem>
              <SidebarMenuButton
                onClick={() => {
                  window.electron.createChatWindow(
                    undefined,
                    window.appConfig.get('GOOSE_WORKING_DIR') as string | undefined
                  );
                }}
                tooltip="Start a new session in a new window"
                className="w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200"
              >
                <ChatSmart className="w-4 h-4" />
                <span>New window</span>
              </SidebarMenuButton>
            </SidebarMenuItem>

            {/* <SidebarMenuItem>
              <SidebarMenuButton
                onClick={() => {
                  window.electron.directoryChooser();
                }}
                tooltip="Start a new session in a different directory"
              >
                <Folder className="w-4 h-4" />
                <span>Open directory</span>
              </SidebarMenuButton>
            </SidebarMenuItem> */}

            <SidebarMenuItem>
              <SidebarMenuButton
                onClick={() => navigate('/sessions')}
                isActive={isActivePath('/sessions')}
                tooltip="View and share previous sessions"
                className="w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200"
              >
                <Time className="w-4 h-4" />
                <span>History</span>
              </SidebarMenuButton>
            </SidebarMenuItem>

            {process.env.ALPHA && (
              <SidebarMenuItem>
                <SidebarMenuButton
                  onClick={() => navigate('/schedules')}
                  isActive={isActivePath('/schedules')}
                  tooltip="Manage scheduled runs"
                  className="w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200"
                >
                  <Clock className="w-4 h-4" />
                  <span>Scheduler</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            )}

            <SidebarMenuItem>
              <SidebarMenuButton
                onClick={() => navigate('/recipes')}
                isActive={isActivePath('/recipes')}
                tooltip="Browse your saved recipes"
                className="w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200"
              >
                <FileText className="w-4 h-4" />
                <span>Recipe library</span>
              </SidebarMenuButton>
            </SidebarMenuItem>

            <SidebarMenuItem>
              <SidebarMenuButton
                onClick={() => navigate('/settings')}
                isActive={isActivePath('/settings')}
                tooltip="View all settings and options"
                className="w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200"
              >
                <Gear className="w-4 h-4" />
                <span>Settings</span>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </div>

        {/* Recent Sessions */}
        {recentSessions.length > 0 && (
          <div className="mt-4">
            <SidebarGroup>
              <SidebarGroupLabel className="text-xs uppercase text-text-muted px-3">
                Recent
              </SidebarGroupLabel>
              <SidebarGroupContent>
                <SidebarMenu>
                  <AnimatePresence mode="popLayout">
                    {recentSessions.map((session, index) => {
                      const hasDescription =
                        session.metadata.description && session.metadata.description.trim() !== '';
                      const isNewSession = session.id.match(/^\d{8}_\d{6}$/);
                      const messageCount = session.metadata.message_count || 0;
                      // Show loading for new sessions with few messages and no description
                      // Only show loading for sessions created in the last 5 minutes
                      // Backend generates descriptions for sessions with < 4 user messages
                      const sessionDate = new Date(session.modified);
                      const fiveMinutesAgo = new Date(Date.now() - 5 * 60 * 1000);
                      const isRecentSession = sessionDate > fiveMinutesAgo;
                      const shouldShowLoading =
                        !hasDescription && isNewSession && messageCount <= 3 && isRecentSession;

                      return (
                        <motion.div
                          key={session.id}
                          initial={{
                            opacity: 0,
                            y: -20,
                            scale: 0.95,
                          }}
                          animate={{
                            opacity: 1,
                            y: 0,
                            scale: 1,
                          }}
                          exit={{
                            opacity: 0,
                            y: 20,
                            scale: 0.95,
                          }}
                          transition={{
                            type: 'spring',
                            stiffness: 300,
                            damping: 25,
                            delay: index * 0.05,
                          }}
                          layout
                        >
                          <SidebarMenuItem>
                            <SidebarMenuButton
                              onClick={() => {
                                const workingDir = session.metadata.working_dir;
                                if (workingDir) {
                                  window.electron.createChatWindow(
                                    undefined,
                                    workingDir,
                                    undefined,
                                    session.id
                                  );
                                }
                              }}
                              tooltip={`Resume session: ${session.metadata.description || session.id}`}
                              className="w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200"
                            >
                              <MessageCircle className="w-4 h-4" />
                              <span className="truncate">
                                {shouldShowLoading ? (
                                  <div className="flex items-center gap-2">
                                    <Loader2 className="w-3 h-3 animate-spin" />
                                    <span className="animate-pulse">Generating description...</span>
                                  </div>
                                ) : (
                                  session.metadata.description || session.id
                                )}
                              </span>
                            </SidebarMenuButton>
                          </SidebarMenuItem>
                        </motion.div>
                      );
                    })}
                  </AnimatePresence>
                </SidebarMenu>
              </SidebarGroupContent>
            </SidebarGroup>
          </div>
        )}

        {/* Theme Selector */}
        <div className="mt-4 opacity-0">
          <ThemeSelector />
        </div>
      </SidebarContent>

      <SidebarFooter>
        <div className="flex items-center gap-2">
          <GooseLogo size="small" />
          <span className="text-base">codename goose</span>
        </div>

        <div className="pb-4">
          <div className="flex gap-2">
            <Tooltip delayDuration={500}>
              <TooltipTrigger asChild>
                <a
                  href="https://discord.gg/pvQ8S2e5"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="inline-flex items-center justify-center h-8 w-8 p-0 rounded-full hover:bg-neutral-200 transition-all duration-200"
                >
                  <MessageCircle className="w-4 h-4" />
                </a>
              </TooltipTrigger>
              <TooltipContent side="right">
                <p>Join our Discord</p>
              </TooltipContent>
            </Tooltip>

            <Tooltip delayDuration={500}>
              <TooltipTrigger asChild>
                <a
                  href="https://www.linkedin.com/company/block-opensource"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="inline-flex items-center justify-center h-8 w-8 p-0 rounded-full hover:bg-neutral-200 transition-all duration-200"
                >
                  <LinkedIn className="w-4 h-4" />
                </a>
              </TooltipTrigger>
              <TooltipContent side="right">
                <p>Follow us on LinkedIn</p>
              </TooltipContent>
            </Tooltip>

            <Tooltip delayDuration={500}>
              <TooltipTrigger asChild>
                <a
                  href="https://www.youtube.com/@blockopensource"
                  target="_blank"
                  rel="noopener noreferrer"
                  className="inline-flex items-center justify-center h-8 w-8 p-0 rounded-full hover:bg-neutral-200 transition-all duration-200"
                >
                  <Youtube className="w-4 h-4" />
                </a>
              </TooltipTrigger>
              <TooltipContent side="right">
                <p>Watch on YouTube</p>
              </TooltipContent>
            </Tooltip>
          </div>
        </div>
      </SidebarFooter>

      {/* Save Recipe Dialog */}
      {showSaveDialog && (
        <div className="fixed inset-0 z-[300] flex items-center justify-center bg-black bg-opacity-50">
          <div className="bg-background-default border border-borderSubtle rounded-lg p-6 w-96 max-w-[90vw]">
            <h3 className="text-lg font-medium text-textProminent mb-4">Save Recipe</h3>

            <div className="space-y-4">
              <div>
                <label
                  htmlFor="recipe-name"
                  className="block text-sm font-medium text-textStandard mb-2"
                >
                  Recipe Name
                </label>
                <input
                  id="recipe-name"
                  type="text"
                  value={saveRecipeName}
                  onChange={(e) => setSaveRecipeName(e.target.value)}
                  className="w-full p-3 border border-borderSubtle rounded-lg bg-background-default text-textStandard focus:outline-none focus:ring-2 focus:ring-borderProminent"
                  placeholder="Enter recipe name"
                  autoFocus
                />
              </div>

              <div>
                <label className="block text-sm font-medium text-textStandard mb-2">
                  Save Location
                </label>
                <div className="space-y-2">
                  <label className="flex items-center">
                    <input
                      type="radio"
                      name="save-location"
                      checked={saveGlobal}
                      onChange={() => setSaveGlobal(true)}
                      className="mr-2"
                    />
                    <span className="text-sm text-textStandard">
                      Global - Available across all Goose sessions
                    </span>
                  </label>
                  <label className="flex items-center">
                    <input
                      type="radio"
                      name="save-location"
                      checked={!saveGlobal}
                      onChange={() => setSaveGlobal(false)}
                      className="mr-2"
                    />
                    <span className="text-sm text-textStandard">
                      Directory - Available in the working directory
                    </span>
                  </label>
                </div>
              </div>
            </div>

            <div className="flex justify-end space-x-3 mt-6">
              <button
                onClick={() => {
                  setShowSaveDialog(false);
                  setSaveRecipeName('');
                }}
                className="px-4 py-2 text-textSubtle hover:text-textStandard transition-colors"
                disabled={saving}
              >
                Cancel
              </button>
              <button
                onClick={handleSaveRecipe}
                disabled={!saveRecipeName.trim() || saving}
                className="px-4 py-2 bg-borderProminent text-text-on-accent rounded-lg hover:bg-opacity-90 transition-colors disabled:opacity-50 disabled:cursor-not-allowed"
              >
                {saving ? 'Saving...' : 'Save Recipe'}
              </button>
            </div>
          </div>
        </div>
      )}
    </>
  );
};

export default AppSidebar;
