import { useEffect, useMemo, useState } from 'react';
import { Recipe, scanRecipe } from '../recipe';
import { ChatType } from '../types/chat';
import { useChatContext } from '../contexts/ChatContext';
import { filterValidUsedParameters, substituteParameters } from '../utils/providerUtils';

/**
 * Hook for managing core recipe state and business logic
 * Extracted from useRecipeManager to focus only on recipe state management
 */
export function useRecipeState(chat: ChatType, recipeConfig?: Recipe | null) {
  const [recipeAccepted, setRecipeAccepted] = useState(false);
  const [hasSecurityWarnings, setHasSecurityWarnings] = useState(false);
  const [recipeError, setRecipeError] = useState<string | null>(null);

  const chatContext = useChatContext();
  const finalRecipeConfig = chat.recipeConfig;
  const recipeParameters = chat.recipeParameters;

  // Handle recipe loading and state management
  useEffect(() => {
    if (!chatContext) return;

    // If we have a recipe from navigation state, always set it and reset acceptance state
    // This ensures that when loading a new recipe, we start fresh
    if (recipeConfig) {
      // Check if this is actually a different recipe (by comparing title and content)
      const currentRecipe = chatContext.chat.recipeConfig;
      const isNewRecipe =
        !currentRecipe ||
        currentRecipe.title !== recipeConfig.title ||
        currentRecipe.instructions !== recipeConfig.instructions ||
        currentRecipe.prompt !== recipeConfig.prompt ||
        JSON.stringify(currentRecipe.activities) !== JSON.stringify(recipeConfig.activities);

      if (isNewRecipe) {
        console.log('Setting new recipe config:', recipeConfig.title);
        // Reset recipe acceptance state when loading a new recipe
        setRecipeAccepted(false);

        chatContext.setChat({
          ...chatContext.chat,
          recipeConfig: recipeConfig,
          recipeParameters: null,
          messages: [],
        });
      }
      return;
    }

    // If we have a recipe from app config (deeplink), persist it
    // But only if the chat context doesn't explicitly have null (which indicates it was cleared)
    const appRecipeConfig = window.appConfig.get('recipe') as Recipe | null;
    if (appRecipeConfig && chatContext.chat.recipeConfig === undefined) {
      chatContext.setRecipeConfig(appRecipeConfig);
    }
  }, [chatContext, recipeConfig]);

  // Handle recipe acceptance logic
  useEffect(() => {
    const checkRecipeAcceptance = async () => {
      if (finalRecipeConfig) {
        // If the recipe comes from session metadata (not from navigation state),
        // it means it was already accepted in a previous session, so auto-accept it
        const isFromSessionMetadata = !recipeConfig && finalRecipeConfig;

        if (isFromSessionMetadata) {
          // Recipe loaded from session metadata should be automatically accepted
          setRecipeAccepted(true);
          return;
        }

        try {
          const hasAccepted = await window.electron.hasAcceptedRecipeBefore(finalRecipeConfig);

          if (!hasAccepted) {
            const securityScanResult = await scanRecipe(finalRecipeConfig);
            setHasSecurityWarnings(securityScanResult.has_security_warnings);
            // Note: Setting warning modal state should be handled by UI hook
          } else {
            setRecipeAccepted(true);
          }
        } catch {
          setHasSecurityWarnings(false);
          // Note: Setting warning modal state should be handled by UI hook
        }
      } else {
        setRecipeAccepted(false);
      }
    };

    checkRecipeAcceptance();
  }, [finalRecipeConfig, recipeConfig]);

  // Filter parameters to only show valid ones that are actually used in the recipe
  const filteredParameters = useMemo(() => {
    if (!finalRecipeConfig?.parameters) {
      return [];
    }
    return filterValidUsedParameters(finalRecipeConfig.parameters, {
      prompt: finalRecipeConfig.prompt || undefined,
      instructions: finalRecipeConfig.instructions || undefined,
      activities: finalRecipeConfig.activities || undefined,
    });
  }, [finalRecipeConfig]);

  // Check if template variables are actually used in the recipe content
  const requiresParameters = useMemo(() => {
    return filteredParameters.length > 0;
  }, [filteredParameters]);

  // Check if all required parameters have been filled in
  const hasAllRequiredParameters = useMemo(() => {
    if (!requiresParameters) {
      return true; // No parameters required, so all are "filled"
    }

    if (!recipeParameters) {
      return false; // Parameters required but none provided
    }

    // Check if all filtered parameters have values
    return filteredParameters.every((param) => {
      const value = recipeParameters[param.key];
      return value !== undefined && value !== null && value.trim() !== '';
    });
  }, [filteredParameters, recipeParameters, requiresParameters]);

  // Generate initial prompt based on recipe and parameters
  const initialPrompt = useMemo(() => {
    if (!finalRecipeConfig?.prompt || !recipeAccepted || finalRecipeConfig?.isScheduledExecution) {
      return '';
    }

    if (requiresParameters && recipeParameters) {
      return substituteParameters(finalRecipeConfig.prompt, recipeParameters);
    }

    return finalRecipeConfig.prompt;
  }, [finalRecipeConfig, recipeParameters, recipeAccepted, requiresParameters]);

  // Business logic functions
  const acceptRecipe = async () => {
    try {
      if (finalRecipeConfig) {
        await window.electron.recordRecipeHash(finalRecipeConfig);
        setRecipeAccepted(true);
      }
    } catch (error) {
      console.error('Error recording recipe hash:', error);
      setRecipeAccepted(true);
    }
  };

  const startRecipe = (recipe: Recipe) => {
    if (chatContext) {
      setRecipeAccepted(false);

      chatContext.setChat({
        ...chatContext.chat,
        messages: [],
        recipeConfig: recipe,
        recipeParameters: null,
      });
    }
  };

  return {
    // State
    recipeConfig: finalRecipeConfig,
    recipeParameters,
    recipeAccepted,
    hasSecurityWarnings,
    recipeError,
    setRecipeError,

    // Computed values
    filteredParameters,
    requiresParameters,
    hasAllRequiredParameters,
    initialPrompt,

    // Actions
    acceptRecipe,
    startRecipe,
  };
}
