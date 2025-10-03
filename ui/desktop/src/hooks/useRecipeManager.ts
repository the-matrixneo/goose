import { useRef, useEffect } from 'react';
import { Recipe } from '../recipe';
import { ChatType } from '../types/chat';
import { useRecipeState } from './useRecipeState';
import { useRecipeUI } from './useRecipeUI';

export const useRecipeManager = (chat: ChatType, recipe?: Recipe | null) => {
  const messagesRef = useRef(chat.messages);

  useEffect(() => {
    messagesRef.current = chat.messages;
  }, [chat.messages]);

  const recipeState = useRecipeState(recipe || chat.recipe || null);

  const recipeUI = useRecipeUI(
    chat,
    recipeState.recipeAccepted,
    recipeState.requiresParameters,
    recipeState.hasAllRequiredParameters(chat.recipeParameters || null),
    recipeState.hasSecurityWarnings,
    recipeState.recipe
  );

  const handleRecipeAccept = async () => {
    await recipeState.acceptRecipe();
    recipeUI.setIsRecipeWarningModalOpen(false);
  };

  const handleStartRecipe = (_recipe: Recipe) => {
    recipeUI.setIsParameterModalOpen(false);
  };

  return {
    recipe: recipeState.recipe,
    recipeParameters: chat.recipeParameters,
    filteredParameters: recipeState.filteredParameters,
    initialPrompt: recipeState.getInitialPrompt(chat.recipeParameters || null),
    recipeAccepted: recipeState.recipeAccepted,
    hasSecurityWarnings: recipeState.hasSecurityWarnings,
    recipeError: recipeState.recipeError,
    setRecipeError: recipeState.setRecipeError,
    isParameterModalOpen: recipeUI.isParameterModalOpen,
    setIsParameterModalOpen: recipeUI.setIsParameterModalOpen,
    isRecipeWarningModalOpen: recipeUI.isRecipeWarningModalOpen,
    setIsRecipeWarningModalOpen: recipeUI.setIsRecipeWarningModalOpen,
    readyForAutoUserPrompt: recipeUI.readyForAutoUserPrompt,
    handleParameterSubmit: recipeUI.handleParameterSubmit,
    handleAutoExecution: recipeUI.handleAutoExecution,
    handleRecipeAccept,
    handleRecipeCancel: recipeUI.handleRecipeCancel,
    handleRecipeCreated: recipeUI.handleRecipeCreated,
    handleStartRecipe,
  };
};
