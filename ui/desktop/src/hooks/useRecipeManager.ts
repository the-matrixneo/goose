import { useRef, useEffect } from 'react';
import { Recipe } from '../recipe';
import { ChatType } from '../types/chat';
import { useRecipeState } from './useRecipeState';
import { useRecipeUI } from './useRecipeUI';

/**
 * Simplified recipe manager that composes focused hooks
 * This is a transitional hook - long term plan is to eliminate it entirely
 * and use useRecipeState and useRecipeUI directly in components
 */
export const useRecipeManager = (chat: ChatType, recipe?: Recipe | null) => {
  const messagesRef = useRef(chat.messages);

  useEffect(() => {
    messagesRef.current = chat.messages;
  }, [chat.messages]);

  // Core recipe state and business logic
  const recipeState = useRecipeState(recipe || chat.recipe || null);

  // UI-specific recipe interactions
  const recipeUI = useRecipeUI(
    chat,
    recipeState.recipeAccepted,
    recipeState.requiresParameters,
    recipeState.hasAllRequiredParameters(chat.recipeParameters || null),
    recipeState.hasSecurityWarnings,
    recipeState.recipe
  );

  // Compose handlers that need both state and UI logic
  const handleRecipeAccept = async () => {
    await recipeState.acceptRecipe();
    recipeUI.setIsRecipeWarningModalOpen(false);
  };

  const handleStartRecipe = (_recipe: Recipe) => {
    // Note: startRecipe functionality needs to be handled at the component level
    // since useRecipeState doesn't manage chat state
    recipeUI.setIsParameterModalOpen(false);
  };

  return {
    // State from useRecipeState
    recipe: recipeState.recipe,
    recipeParameters: chat.recipeParameters,
    filteredParameters: recipeState.filteredParameters,
    initialPrompt: recipeState.getInitialPrompt(chat.recipeParameters || null),
    recipeAccepted: recipeState.recipeAccepted,
    hasSecurityWarnings: recipeState.hasSecurityWarnings,
    recipeError: recipeState.recipeError,
    setRecipeError: recipeState.setRecipeError,

    // UI state from useRecipeUI
    isParameterModalOpen: recipeUI.isParameterModalOpen,
    setIsParameterModalOpen: recipeUI.setIsParameterModalOpen,
    isRecipeWarningModalOpen: recipeUI.isRecipeWarningModalOpen,
    setIsRecipeWarningModalOpen: recipeUI.setIsRecipeWarningModalOpen,
    readyForAutoUserPrompt: recipeUI.readyForAutoUserPrompt,

    // Handlers - composed or delegated
    handleParameterSubmit: recipeUI.handleParameterSubmit,
    handleAutoExecution: recipeUI.handleAutoExecution,
    handleRecipeAccept,
    handleRecipeCancel: recipeUI.handleRecipeCancel,
    handleRecipeCreated: recipeUI.handleRecipeCreated,
    handleStartRecipe,
  };
};
