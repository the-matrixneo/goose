import React, { useEffect, useState } from 'react';
import { Folder, FileText, Clock, MessageCircle, Home } from 'lucide-react';
import { useNavigate } from 'react-router-dom';
import { SidebarContent, SidebarFooter } from '../ui/sidebar';
import { Button } from '../ui/button';
import { ChatSmart, Time, Gear, LinkedIn, Youtube, Discord } from '../icons';
import { ViewOptions, View } from '../../App';
import { useConfig } from '../ConfigContext';
import { Recipe } from '../../recipe';
import { saveRecipe, generateRecipeFilename } from '../../recipe/recipeStorage';
import { Tooltip, TooltipContent, TooltipTrigger } from '../ui/Tooltip';
import ThemeSelector from './ThemeSelector';
import GooseLogo from '../GooseLogo';

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
        <div className="px-1 py-0 pt-14 space-y-2 relative">
          <Tooltip delayDuration={500}>
            <TooltipTrigger className="w-full">
              <Button
                onClick={() => navigate('/')}
                className={`w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200`}
                variant="ghost"
              >
                <div className="flex gap-2 items-center text-text-default">
                  <Home className="w-4 h-4" />
                  <span className="text-sm">Home</span>
                </div>
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right">
              <p>Go back to the main chat screen</p>
            </TooltipContent>
          </Tooltip>

          <Tooltip delayDuration={500}>
            <TooltipTrigger className="w-full">
              <Button
                onClick={() => {
                  window.electron.createChatWindow(
                    undefined,
                    window.appConfig.get('GOOSE_WORKING_DIR') as string | undefined
                  );
                }}
                className="w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200"
                variant="ghost"
              >
                <div className="flex gap-2 items-center text-text-default">
                  <ChatSmart className="w-4 h-4" />
                  <span className="text-sm">New session</span>
                </div>
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right">
              <p>Start a new session in the current directory</p>
            </TooltipContent>
          </Tooltip>

          <Tooltip delayDuration={500}>
            <TooltipTrigger className="w-full">
              <Button
                onClick={() => {
                  window.electron.directoryChooser();
                }}
                className="w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200"
                variant="ghost"
              >
                <div className="flex gap-2 items-center text-text-default">
                  <Folder className="w-4 h-4" />
                  <span className="text-sm">Open directory</span>
                </div>
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right">
              <p>Start a new session in a different directory</p>
            </TooltipContent>
          </Tooltip>

          <Tooltip delayDuration={500}>
            <TooltipTrigger className="w-full">
              <Button
                onClick={() => navigate('/sessions')}
                className={`w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200 ${
                  isActivePath('/sessions') ? 'bg-neutral-200' : ''
                }`}
                variant="ghost"
              >
                <div className="flex gap-2 items-center text-text-default">
                  <Time className="w-4 h-4" />
                  Past sessions
                </div>
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right">
              <p>View and share previous sessions</p>
            </TooltipContent>
          </Tooltip>

          {process.env.ALPHA && (
            <Tooltip delayDuration={500}>
              <TooltipTrigger className="w-full">
                <Button
                  onClick={() => navigate('/schedules')}
                  className={`w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200 ${
                    isActivePath('/schedules') ? 'bg-neutral-200' : ''
                  }`}
                  variant="ghost"
                >
                  <div className="flex gap-2 items-center text-text-default">
                    <Clock className="w-4 h-4" />
                    Scheduler
                  </div>
                </Button>
              </TooltipTrigger>
              <TooltipContent side="right">
                <p>Manage scheduled runs</p>
              </TooltipContent>
            </Tooltip>
          )}

          <Tooltip delayDuration={500}>
            <TooltipTrigger className="w-full">
              <Button
                onClick={() => navigate('/recipes')}
                className={`w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200 ${
                  isActivePath('/recipes') ? 'bg-neutral-200' : ''
                }`}
                variant="ghost"
              >
                <div className="flex gap-2 items-center text-text-default">
                  <FileText className="w-4 h-4" />
                  Recipe library
                </div>
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right">
              <p>Browse your saved recipes</p>
            </TooltipContent>
          </Tooltip>

          <Tooltip delayDuration={500}>
            <TooltipTrigger className="w-full">
              <Button
                onClick={() => navigate('/settings')}
                className={`w-full justify-start px-3 rounded-lg h-fit hover:bg-neutral-200 transition-all duration-200 ${
                  isActivePath('/settings') ? 'bg-neutral-200' : ''
                }`}
                variant="ghost"
              >
                <div className="flex gap-2 items-center text-text-default">
                  <Gear className="w-4 h-4" />
                  Settings
                </div>
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right">
              <p>View all settings and options</p>
            </TooltipContent>
          </Tooltip>
        </div>

        {/* Theme Selector */}
        <div className="mt-4">{/* <ThemeSelector /> */}</div>
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
          <div className="bg-bgApp border border-borderSubtle rounded-lg p-6 w-96 max-w-[90vw]">
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
                  className="w-full p-3 border border-borderSubtle rounded-lg bg-bgApp text-textStandard focus:outline-none focus:ring-2 focus:ring-borderProminent"
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
