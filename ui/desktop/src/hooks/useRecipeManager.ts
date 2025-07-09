import { useEffect, useMemo, useState } from 'react';
import { createRecipe, Recipe } from '../recipe';
import { Message } from '../types/message';

export const useRecipeManager = (messages: Message[]) => {
  const [isGeneratingRecipe, setIsGeneratingRecipe] = useState(false);

  // Get recipeConfig directly from appConfig
  const recipeConfig = useMemo(() => {
    return window.appConfig.get('recipeConfig') as Recipe | null;
  }, []);

  // Pre-fill input with recipe prompt instead of auto-sending it
  const initialPrompt = useMemo(() => {
    return recipeConfig?.prompt || '';
  }, [recipeConfig?.prompt]);

  // Listen for make-agent-from-chat event
  useEffect(() => {
    const handleMakeAgent = async () => {
      window.electron.logInfo('Making recipe from chat...');
      setIsGeneratingRecipe(true);

      try {
        // Create recipe directly from chat messages
        const createRecipeRequest = {
          messages: messages,
          title: '',
          description: '',
        };

        const response = await createRecipe(createRecipeRequest);

        if (response.error) {
          throw new Error(`Failed to create recipe: ${response.error}`);
        }

        window.electron.logInfo('Created recipe:');
        window.electron.logInfo(JSON.stringify(response.recipe, null, 2));

        // First, verify the recipe data
        if (!response.recipe) {
          throw new Error('No recipe data received');
        }

        // Create a new window for the recipe editor
        console.log('Opening recipe editor with config:', response.recipe);
        const recipeConfig = {
          id: response.recipe.title || 'untitled',
          title: response.recipe.title || 'Untitled Recipe',
          description: response.recipe.description || '',
          instructions: response.recipe.instructions || '',
          activities: response.recipe.activities || [],
          prompt: response.recipe.prompt || '',
        };
        window.electron.createChatWindow(
          undefined, // query
          undefined, // dir
          undefined, // version
          undefined, // resumeSessionId
          recipeConfig, // recipe config
          'recipeEditor' // view type
        );

        window.electron.logInfo('Opening recipe editor window');
      } catch (error) {
        window.electron.logInfo('Failed to create recipe:');
        const errorMessage = error instanceof Error ? error.message : String(error);
        window.electron.logInfo(errorMessage);
      } finally {
        setIsGeneratingRecipe(false);
      }
    };

    window.addEventListener('make-agent-from-chat', handleMakeAgent);

    return () => {
      window.removeEventListener('make-agent-from-chat', handleMakeAgent);
    };
  }, [messages]);

  return {
    recipeConfig,
    initialPrompt,
    isGeneratingRecipe,
  };
};
