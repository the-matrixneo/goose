import React, { useEffect, useState, useCallback, useRef } from 'react';
import {
  Folder,
  FileText,
  Clock,
  MessageCircle,
  Home,
  Loader2,
  AppWindowMac,
  AppWindow,
} from 'lucide-react';
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
  SidebarTrigger,
  useSidebar,
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
import { Goose } from '../icons/Goose';

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
  const { remove } = useConfig();
  const { hasActiveSession, resetChat } = useChatContext();
  const { state } = useSidebar();
  const safeIsMacOS = (window?.electron?.platform || 'darwin') === 'darwin';

  // Calculate padding based on sidebar state and macOS
  const headerPadding = safeIsMacOS ? 'pl-8' : 'pl-4';

  useEffect(() => {
    // Trigger animation after a small delay
    const timer = setTimeout(() => {
      setIsVisible(true);
    }, 100);
    return () => clearTimeout(timer);
  }, []);

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
      <SidebarContent>
        {/* <SidebarHeader>
          <div className="flex items-center gap-2 pt-12 pb-4">
            <GooseLogo size="small" />
            <span className="text-base">codename goose</span>
          </div>
        </SidebarHeader> */}

        {/* Menu */}
        <div className="pt-12">
          <SidebarMenu>
            <motion.div
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ type: 'spring', stiffness: 300, damping: 25, delay: 0.1 }}
            >
              <SidebarMenuItem>
                <SidebarMenuButton
                  onClick={() => {
                    // Always reset chat and navigate to home when clicking Home button
                    // This ensures insights are displayed regardless of active session
                    resetChat();
                    navigate('/');
                  }}
                  isActive={isActivePath('/')}
                  tooltip="Go back to the main chat screen"
                  className="w-full justify-start px-3 rounded-xl h-fit hover:bg-background-medium transition-all duration-200 data-[active=true]:bg-background-medium"
                >
                  <Home className="w-4 h-4" />
                  <span>Home</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ type: 'spring', stiffness: 300, damping: 25, delay: 0.125 }}
            >
              <SidebarMenuItem>
                <SidebarMenuButton
                  onClick={() => navigate('/pair')}
                  isActive={isActivePath('/pair')}
                  tooltip="Start pairing with Goose"
                  className="w-full justify-start px-3 rounded-xl h-fit hover:bg-background-medium transition-all duration-200 data-[active=true]:bg-background-medium"
                >
                  <ChatSmart className="w-4 h-4" />
                  <span>Chat</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </motion.div>

            <motion.div
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ type: 'spring', stiffness: 300, damping: 25, delay: 0.15 }}
            >
              <SidebarMenuItem>
                <SidebarMenuButton
                  onClick={() => {
                    window.electron.createChatWindow(
                      undefined,
                      window.appConfig.get('GOOSE_WORKING_DIR') as string | undefined
                    );
                  }}
                  tooltip="Start a new session in a new window"
                  className="w-full justify-start px-3 rounded-xl h-fit hover:bg-background-medium transition-all duration-200"
                >
                  {safeIsMacOS ? (
                    <AppWindowMac className="w-4 h-4" />
                  ) : (
                    <AppWindow className="w-4 h-4" />
                  )}
                  <span>New window</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </motion.div>

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

            {/* History button moved to AppLayout header */}

            {process.env.ALPHA && (
              <motion.div
                initial={{ opacity: 0, x: 20 }}
                animate={{ opacity: 1, x: 0 }}
                transition={{ type: 'spring', stiffness: 300, damping: 25, delay: 0.25 }}
              >
                <SidebarMenuItem>
                  <SidebarMenuButton
                    onClick={() => navigate('/schedules')}
                    isActive={isActivePath('/schedules')}
                    tooltip="Manage scheduled runs"
                    className="w-full justify-start px-3 rounded-xl h-fit hover:bg-background-medium transition-all duration-200 data-[active=true]:bg-background-medium"
                  >
                    <Clock className="w-4 h-4" />
                    <span>Scheduler</span>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              </motion.div>
            )}

            <motion.div
              initial={{ opacity: 0, x: 20 }}
              animate={{ opacity: 1, x: 0 }}
              transition={{ type: 'spring', stiffness: 300, damping: 25, delay: 0.3 }}
            >
              <SidebarMenuItem>
                <SidebarMenuButton
                  onClick={() => navigate('/recipes')}
                  isActive={isActivePath('/recipes')}
                  tooltip="Browse your saved recipes"
                  className="w-full justify-start px-3 rounded-xl h-fit hover:bg-background-medium transition-all duration-200 data-[active=true]:bg-background-medium"
                >
                  <FileText className="w-4 h-4" />
                  <span>Recipes</span>
                </SidebarMenuButton>
              </SidebarMenuItem>
            </motion.div>

            {/* Settings button moved to AppLayout header */}
          </SidebarMenu>
        </div>

        {/* Theme Selector */}
        <div className="mt-4 opacity-0">
          <ThemeSelector />
        </div>
      </SidebarContent>

      <SidebarFooter>
        <motion.div
          initial={{ opacity: 0, x: 20 }}
          animate={{ opacity: 1, x: 0 }}
          transition={{ type: 'spring', stiffness: 300, damping: 25, delay: 0.4 }}
          className="flex flex-col gap-2 w-full"
        >
          <div className="flex items-center gap-2">
            <Goose className="w-6 h-6" />
            <AnimatePresence mode="wait">
              {state === 'expanded' && (
                <motion.span
                  key="logo-text"
                  initial={{ opacity: 0, width: 0 }}
                  animate={{ opacity: 1, width: 'auto' }}
                  exit={{ opacity: 0, width: 0 }}
                  transition={{ duration: 0.2, ease: 'easeInOut' }}
                  className="text-base overflow-hidden whitespace-nowrap leading-tight"
                >
                  codename
                  <br /> goose
                </motion.span>
              )}
            </AnimatePresence>
          </div>

          {/* <AnimatePresence mode="wait">
            {state === 'expanded' && (
              <motion.div
                key="social-icons"
                initial={{ opacity: 0, height: 0 }}
                animate={{ opacity: 1, height: 'auto' }}
                exit={{ opacity: 0, height: 0 }}
                transition={{ duration: 0.2, ease: 'easeInOut' }}
                className="flex gap-1 overflow-hidden"
              >
                <Tooltip delayDuration={500}>
                  <TooltipTrigger asChild>
                    <a
                      href="https://discord.gg/pvQ8S2e5"
                      target="_blank"
                      rel="noopener noreferrer"
                      className="inline-flex items-center justify-center h-8 w-8 p-0 rounded-lg hover:bg-background-medium hover:text-text-default transition-all duration-200 text-text-muted"
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
                      className="inline-flex items-center justify-center h-8 w-8 p-0 rounded-lg hover:bg-background-medium hover:text-text-default transition-all duration-200 text-text-muted"
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
                      className="inline-flex items-center justify-center h-8 w-8 p-0 rounded-lg hover:bg-background-medium hover:text-text-default transition-all duration-200 text-text-muted"
                    >
                      <Youtube className="w-4 h-4" />
                    </a>
                  </TooltipTrigger>
                  <TooltipContent side="right">
                    <p>Watch on YouTube</p>
                  </TooltipContent>
                </Tooltip>
              </motion.div>
            )}
          </AnimatePresence> */}
        </motion.div>
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
