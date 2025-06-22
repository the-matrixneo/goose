import React, { useEffect, useState } from 'react';
import { Folder, FileText, Clock, Save } from 'lucide-react';
import { SidebarContent, SidebarFooter } from '../ui/sidebar';
import { Button } from '../ui/button';
import { ChatSmart, Idea, Time, Send, Refresh, Gear } from '../icons';
import { Separator } from '../ui/separator';
import { ViewOptions, View } from '../../App';
import { useConfig } from '../ConfigContext';
import { Recipe } from '../../recipe';
import { saveRecipe, generateRecipeFilename } from '../../recipe/recipeStorage';
import { Tooltip, TooltipContent, TooltipTrigger } from '../ui/Tooltip';
import ThemeSelector from './ThemeSelector';

interface SidebarProps {
  onSelectSession: (sessionId: string) => void;
  refreshTrigger?: number;
  children?: React.ReactNode;
  setIsGoosehintsModalOpen?: (isOpen: boolean) => void;
  setView?: (view: View, viewOptions?: ViewOptions) => void;
}

// Main Sidebar Component
const AppSidebar: React.FC<SidebarProps> = ({ setIsGoosehintsModalOpen, setView }) => {
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

  return (
    <>
      <SidebarContent>
        {/* Action Buttons */}
        <div className="px-1 pt-14 py-0 space-y-2 relative">
          {setView && (
            <Tooltip delayDuration={500}>
              <TooltipTrigger className="w-full">
                <Button
                  onClick={() => setView('sessions')}
                  className="w-full justify-start px-3 rounded-lg h-fit hover:shadow-default hover:bg-background-default transition-all duration-200"
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
          )}

          {process.env.ALPHA && setView && (
            <Tooltip delayDuration={500}>
              <TooltipTrigger className="w-full">
                <Button
                  onClick={() => setView('schedules')}
                  className="w-full justify-start px-3 rounded-lg h-fit hover:shadow-default hover:bg-background-default transition-all duration-200"
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

          {setIsGoosehintsModalOpen && (
            <Tooltip delayDuration={500}>
              <TooltipTrigger className="w-full">
                <Button
                  onClick={() => setIsGoosehintsModalOpen(true)}
                  className="w-full justify-start px-3 rounded-lg h-fit hover:shadow-default hover:bg-background-default transition-all duration-200"
                  variant="ghost"
                >
                  <div className="flex gap-2 items-center text-text-default">
                    <Idea className="w-4 h-4" />
                    Configure .goosehints
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
                <TooltipTrigger className="w-full">
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
                    className="w-full justify-start px-3 rounded-lg h-fit hover:shadow-default hover:bg-background-default transition-all duration-200"
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
                <TooltipTrigger className="w-full">
                  <Button
                    onClick={handleSaveRecipeClick}
                    className="w-full justify-start px-3 rounded-lg h-fit hover:shadow-default hover:bg-background-default transition-all duration-200"
                    variant="ghost"
                  >
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
              <TooltipTrigger className="w-full">
                <Button
                  onClick={() => {
                    window.electron.logInfo('Make Agent button clicked');
                    window.dispatchEvent(new CustomEvent('make-agent-from-chat'));
                  }}
                  className="w-full justify-start px-3 rounded-lg h-fit hover:shadow-default hover:bg-background-default transition-all duration-200"
                  variant="ghost"
                >
                  <div className="flex gap-2 items-center text-text-default">
                    <Send className="w-4 h-4" />
                    Make Agent from this session
                  </div>
                </Button>
              </TooltipTrigger>
              <TooltipContent side="right">
                <p>Make a custom agent you can share or reuse</p>
              </TooltipContent>
            </Tooltip>
          )}
          {setView && (
            <Tooltip delayDuration={500}>
              <TooltipTrigger className="w-full">
                <Button
                  onClick={() => setView('recipes')}
                  className="w-full justify-start px-3 rounded-lg h-fit hover:shadow-default hover:bg-background-default transition-all duration-200"
                  variant="ghost"
                >
                  <div className="flex gap-2 items-center text-text-default">
                    <FileText className="w-4 h-4" />
                    Recipe Library
                  </div>
                </Button>
              </TooltipTrigger>
              <TooltipContent side="right">
                <p>Browse your saved recipes</p>
              </TooltipContent>
            </Tooltip>
          )}

          {/* {setView && (
            <Tooltip delayDuration={500}>
              <TooltipTrigger className="w-full">
                <Button
                  onClick={() => setView('settings')}
                  className="w-full justify-start px-3 rounded-lg h-fit hover:shadow-default hover:bg-background-default transition-all duration-200"
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
          )} */}
        </div>

        {/* Theme Selector */}
        <div className="mt-4">
          <ThemeSelector />
        </div>
      </SidebarContent>

      <SidebarFooter>
        <div className="flex justify-between items-center w-full">
          <Tooltip delayDuration={500}>
            <TooltipTrigger>
              <Button
                onClick={() => {
                  window.electron.createChatWindow(
                    undefined,
                    window.appConfig.get('GOOSE_WORKING_DIR') as string | undefined
                  );
                }}
                className="px-3 hover:shadow-default hover:bg-background-default transition-all duration-200"
                variant="ghost"
                shape="round"
              >
                <ChatSmart className="w-4 h-4" />
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right">
              <p>Start a new session in the current directory</p>
            </TooltipContent>
          </Tooltip>

          <Tooltip delayDuration={500}>
            <TooltipTrigger>
              <Button
                onClick={() => {
                  window.electron.directoryChooser();
                }}
                className="px-3 hover:shadow-default hover:bg-background-default transition-all duration-200"
                variant="ghost"
                shape="round"
              >
                <Folder className="w-4 h-4" />
              </Button>
            </TooltipTrigger>
            <TooltipContent side="right">
              <p>Start a new session in a different directory</p>
            </TooltipContent>
          </Tooltip>

          {/* <Button
            onClick={async () => {
              await remove('GOOSE_PROVIDER', false);
              await remove('GOOSE_MODEL', false);
              setView?.('welcome');
            }}
            className="px-3 hover:shadow-default hover:bg-background-default transition-all duration-200 text-red-400 hover:text-red-300"
            variant="ghost"
            shape="round"
          >
            <Refresh />
          </Button> */}

          <Button
            onClick={() => setView?.('settings')}
            className="px-3 hover:shadow-default hover:bg-background-default transition-all duration-200"
            variant="ghost"
            shape="round"
          >
            <Gear />
          </Button>
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
