import { useState, useEffect } from 'react';
import { listSavedRecipes, archiveRecipe, SavedRecipe } from '../recipe/recipeStorage';
import { FileText, Trash2, Bot, Calendar, Globe, Folder, AlertCircle } from 'lucide-react';
import { ScrollArea } from './ui/scroll-area';
import { Card } from './ui/card';
import { Button } from './ui/button';
import { Skeleton } from './ui/skeleton';
import { SidebarTrigger, useSidebar } from './ui/sidebar';
import BackButton from './ui/BackButton';
import MoreMenuLayout from './more_menu/MoreMenuLayout';
import { MainPanelLayout } from './Layout/MainPanelLayout';

interface RecipesViewProps {
  onBack: () => void;
}

export default function RecipesView({ onBack }: RecipesViewProps) {
  const [savedRecipes, setSavedRecipes] = useState<SavedRecipe[]>([]);
  const [loading, setLoading] = useState(true);
  const [showSkeleton, setShowSkeleton] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [selectedRecipe, setSelectedRecipe] = useState<SavedRecipe | null>(null);
  const [showPreview, setShowPreview] = useState(false);
  const [showContent, setShowContent] = useState(false);
  const { open: isSidebarOpen } = useSidebar();

  const safeIsMacOS = (window?.electron?.platform || 'darwin') === 'darwin';

  // Calculate padding based on sidebar state and macOS
  const headerPadding = !isSidebarOpen ? (safeIsMacOS ? 'pl-20' : 'pl-12') : 'pl-4';

  useEffect(() => {
    loadSavedRecipes();
  }, []);

  // Minimum loading time to prevent skeleton flash
  useEffect(() => {
    if (!loading && showSkeleton) {
      const timer = setTimeout(() => {
        setShowSkeleton(false);
        // Add a small delay before showing content for fade-in effect
        setTimeout(() => {
          setShowContent(true);
        }, 50);
      }, 300); // Show skeleton for at least 300ms

      return () => clearTimeout(timer);
    }
  }, [loading, showSkeleton]);

  const loadSavedRecipes = async () => {
    try {
      setLoading(true);
      setShowSkeleton(true);
      setShowContent(false);
      setError(null);
      const recipes = await listSavedRecipes();
      setSavedRecipes(recipes);
    } catch (err) {
      setError(err instanceof Error ? err.message : 'Failed to load recipes');
      console.error('Failed to load saved recipes:', err);
    } finally {
      setLoading(false);
    }
  };

  const handleLoadRecipe = async (savedRecipe: SavedRecipe) => {
    try {
      // Use the recipe directly - no need for manual mapping
      window.electron.createChatWindow(
        undefined, // query
        undefined, // dir
        undefined, // version
        undefined, // resumeSessionId
        savedRecipe.recipe, // recipe config
        undefined // view type
      );
    } catch (err) {
      console.error('Failed to load recipe:', err);
      setError(err instanceof Error ? err.message : 'Failed to load recipe');
    }
  };

  const handleDeleteRecipe = async (savedRecipe: SavedRecipe) => {
    // TODO: Use Electron's dialog API for confirmation
    const result = await window.electron.showMessageBox({
      type: 'warning',
      buttons: ['Cancel', 'Delete'],
      defaultId: 0,
      title: 'Delete Recipe',
      message: `Are you sure you want to delete "${savedRecipe.name}"?`,
      detail: 'Deleted recipes can be restored later.',
    });

    if (result.response !== 1) {
      return;
    }

    try {
      await archiveRecipe(savedRecipe.name, savedRecipe.isGlobal);
      // Reload the recipes list
      await loadSavedRecipes();
    } catch (err) {
      console.error('Failed to archive recipe:', err);
      setError(err instanceof Error ? err.message : 'Failed to archive recipe');
    }
  };

  const handlePreviewRecipe = (savedRecipe: SavedRecipe) => {
    setSelectedRecipe(savedRecipe);
    setShowPreview(true);
  };

  // Render a recipe item
  const RecipeItem = ({ savedRecipe }: { savedRecipe: SavedRecipe }) => (
    <Card className="py-2 px-4 mb-2 bg-background-default border-none hover:bg-background-muted cursor-pointer transition-all duration-150">
      <div className="flex justify-between items-start gap-4">
        <div className="min-w-0 flex-1">
          <div className="flex items-center gap-2 mb-1">
            <h3 className="text-base truncate max-w-[50vw]">{savedRecipe.recipe.title}</h3>
            {savedRecipe.isGlobal ? (
              <Globe className="w-4 h-4 text-text-muted flex-shrink-0" />
            ) : (
              <Folder className="w-4 h-4 text-text-muted flex-shrink-0" />
            )}
          </div>
          <p className="text-text-muted text-sm mb-2 line-clamp-2">
            {savedRecipe.recipe.description}
          </p>
          <div className="flex items-center text-xs text-text-muted">
            <Calendar className="w-3 h-3 mr-1" />
            {savedRecipe.lastModified.toLocaleDateString()}
          </div>
        </div>

        <div className="flex items-center gap-2 shrink-0">
          <Button
            onClick={(e) => {
              e.stopPropagation();
              handleLoadRecipe(savedRecipe);
            }}
            size="sm"
            className="h-8"
          >
            <Bot className="w-4 h-4 mr-1" />
            Use
          </Button>
          <Button
            onClick={(e) => {
              e.stopPropagation();
              handlePreviewRecipe(savedRecipe);
            }}
            variant="outline"
            size="sm"
            className="h-8"
          >
            <FileText className="w-4 h-4 mr-1" />
            Preview
          </Button>
          <Button
            onClick={(e) => {
              e.stopPropagation();
              handleDeleteRecipe(savedRecipe);
            }}
            variant="ghost"
            size="sm"
            className="h-8 text-red-500 hover:text-red-600 hover:bg-red-50 dark:hover:bg-red-900/20"
          >
            <Trash2 className="w-4 h-4" />
          </Button>
        </div>
      </div>
    </Card>
  );

  // Render skeleton loader for recipe items
  const RecipeSkeleton = () => (
    <Card className="p-2 mb-2 bg-background-default">
      <div className="flex justify-between items-start gap-4">
        <div className="min-w-0 flex-1">
          <Skeleton className="h-5 w-3/4 mb-2" />
          <Skeleton className="h-4 w-full mb-2" />
          <Skeleton className="h-4 w-24" />
        </div>
        <div className="flex items-center gap-2 shrink-0">
          <Skeleton className="h-8 w-16" />
          <Skeleton className="h-8 w-20" />
          <Skeleton className="h-8 w-8" />
        </div>
      </div>
    </Card>
  );

  const renderContent = () => {
    if (loading || showSkeleton) {
      return (
        <div className="space-y-6">
          <div className="space-y-3">
            <Skeleton className="h-6 w-24" />
            <div className="space-y-2">
              <RecipeSkeleton />
              <RecipeSkeleton />
              <RecipeSkeleton />
            </div>
          </div>
        </div>
      );
    }

    if (error) {
      return (
        <div className="flex flex-col items-center justify-center h-full text-text-muted">
          <AlertCircle className="h-12 w-12 text-red-500 mb-4" />
          <p className="text-lg mb-2">Error Loading Recipes</p>
          <p className="text-sm text-center mb-4">{error}</p>
          <Button onClick={loadSavedRecipes} variant="default">
            Try Again
          </Button>
        </div>
      );
    }

    if (savedRecipes.length === 0) {
      return (
        <div className="flex flex-col justify-center pt-6 h-full text-text-muted">
          <FileText className="h-8 w-8 mb-4" />
          <p className="text-lg mb-2">No saved recipes</p>
          <p className="text-sm">Save a recipe from an active session to see it here.</p>
        </div>
      );
    }

    return (
      <div className="space-y-2">
        {savedRecipes.map((savedRecipe) => (
          <RecipeItem
            key={`${savedRecipe.isGlobal ? 'global' : 'local'}-${savedRecipe.name}`}
            savedRecipe={savedRecipe}
          />
        ))}
      </div>
    );
  };

  return (
    <>
      <MainPanelLayout>
        <div className="h-12 flex items-center justify-between">
          <div className={`flex items-center ${headerPadding}`}>
            <SidebarTrigger className="no-drag" />
          </div>
        </div>

        <div className="flex-1 flex flex-col min-h-0">
          {/* Content Area */}
          <div className="flex flex-col mt-4 mb-6 px-6">
            <h1 className="text-4xl font-light">Saved Recipes</h1>
            <h3 className="text-sm text-text-muted mt-2">
              View and manage your saved recipes to quickly start new sessions with predefined
              configurations.
            </h3>
          </div>

          <div className="flex-1 min-h-0 relative px-6">
            <ScrollArea className="h-full">
              <div
                className={`h-full relative transition-opacity duration-300 ${
                  showContent ? 'opacity-100' : 'opacity-0'
                }`}
              >
                {renderContent()}
              </div>
            </ScrollArea>
          </div>
        </div>
      </MainPanelLayout>

      {/* Preview Modal */}
      {showPreview && selectedRecipe && (
        <div className="fixed inset-0 z-[300] flex items-center justify-center bg-black bg-opacity-50">
          <div className="bg-background-default border border-border-subtle rounded-lg p-6 w-[600px] max-w-[90vw] max-h-[80vh] overflow-y-auto">
            <div className="flex items-start justify-between mb-4">
              <div>
                <h3 className="text-xl font-medium text-text-standard">
                  {selectedRecipe.recipe.title}
                </h3>
                <p className="text-sm text-text-muted">
                  {selectedRecipe.isGlobal ? 'Global recipe' : 'Project recipe'}
                </p>
              </div>
              <button
                onClick={() => setShowPreview(false)}
                className="text-text-muted hover:text-text-standard text-2xl leading-none"
              >
                ×
              </button>
            </div>

            <div className="space-y-6">
              <div>
                <h4 className="text-sm font-medium text-text-standard mb-2">Description</h4>
                <p className="text-text-muted">{selectedRecipe.recipe.description}</p>
              </div>

              {selectedRecipe.recipe.instructions && (
                <div>
                  <h4 className="text-sm font-medium text-text-standard mb-2">Instructions</h4>
                  <div className="bg-background-muted border border-border-subtle p-3 rounded-lg">
                    <pre className="text-sm text-text-muted whitespace-pre-wrap font-mono">
                      {selectedRecipe.recipe.instructions}
                    </pre>
                  </div>
                </div>
              )}

              {selectedRecipe.recipe.prompt && (
                <div>
                  <h4 className="text-sm font-medium text-text-standard mb-2">Initial Prompt</h4>
                  <div className="bg-background-muted border border-border-subtle p-3 rounded-lg">
                    <pre className="text-sm text-text-muted whitespace-pre-wrap font-mono">
                      {selectedRecipe.recipe.prompt}
                    </pre>
                  </div>
                </div>
              )}

              {selectedRecipe.recipe.activities && selectedRecipe.recipe.activities.length > 0 && (
                <div>
                  <h4 className="text-sm font-medium text-text-standard mb-2">Activities</h4>
                  <div className="flex flex-wrap gap-2">
                    {selectedRecipe.recipe.activities.map((activity, index) => (
                      <span
                        key={index}
                        className="px-2 py-1 bg-background-muted border border-border-subtle text-text-muted rounded text-sm"
                      >
                        {activity}
                      </span>
                    ))}
                  </div>
                </div>
              )}
            </div>

            <div className="flex justify-end gap-3 mt-6 pt-4 border-t border-border-subtle">
              <Button onClick={() => setShowPreview(false)} variant="ghost">
                Close
              </Button>
              <Button
                onClick={() => {
                  setShowPreview(false);
                  handleLoadRecipe(selectedRecipe);
                }}
                variant="default"
              >
                Load Recipe
              </Button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
