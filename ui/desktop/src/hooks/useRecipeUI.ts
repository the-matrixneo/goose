import { useEffect, useState } from 'react';
import { Message, createUserMessage } from '../types/message';
import { Recipe } from '../recipe';
import { ChatType } from '../types/chat';
import { useChatContext } from '../contexts/ChatContext';
import { updateSystemPromptWithParameters, substituteParameters } from '../utils/providerUtils';
import { updateSessionUserRecipeValues } from '../api';

/**
 * Hook for managing recipe-related UI state and interactions
 * Handles modals, user interactions, and UI-specific recipe logic
 */
export function useRecipeUI(
  chat: ChatType,
  recipeAccepted: boolean,
  requiresParameters: boolean,
  hasAllRequiredParameters: boolean,
  hasSecurityWarnings: boolean,
  finalRecipeConfig?: Recipe | null
) {
  const [isParameterModalOpen, setIsParameterModalOpen] = useState(false);
  const [isRecipeWarningModalOpen, setIsRecipeWarningModalOpen] = useState(false);
  const [readyForAutoUserPrompt, setReadyForAutoUserPrompt] = useState(false);

  const chatContext = useChatContext();
  const messages = chat.messages;
  const hasMessages = messages.length > 0;

  // Handle parameter modal display logic
  useEffect(() => {
    // Only show parameter modal if:
    // 1. Recipe requires parameters
    // 2. Recipe has been accepted
    // 3. Not all required parameters have been filled in yet
    // 4. Parameter modal is not already open (prevent multiple opens)
    // 5. No messages in chat yet (don't show after conversation has started)
    if (
      requiresParameters &&
      recipeAccepted &&
      !hasAllRequiredParameters &&
      !isParameterModalOpen &&
      !hasMessages
    ) {
      setIsParameterModalOpen(true);
    }
  }, [
    requiresParameters,
    hasAllRequiredParameters,
    recipeAccepted,
    isParameterModalOpen,
    hasMessages,
    chat.sessionId,
    finalRecipeConfig?.title,
  ]);

  // Handle recipe warning modal display
  useEffect(() => {
    if (finalRecipeConfig && !recipeAccepted && hasSecurityWarnings) {
      setIsRecipeWarningModalOpen(true);
    }
  }, [finalRecipeConfig, recipeAccepted, hasSecurityWarnings]);

  useEffect(() => {
    setReadyForAutoUserPrompt(true);
  }, []);

  // UI interaction handlers
  const handleParameterSubmit = async (inputValues: Record<string, string>) => {
    // Update chat state with parameters
    if (chatContext) {
      chatContext.setChat({
        ...chatContext.chat,
        recipeParameters: inputValues,
      });
    }
    setIsParameterModalOpen(false);

    try {
      await updateSystemPromptWithParameters(
        chat.sessionId,
        inputValues,
        finalRecipeConfig || undefined
      );

      // Save recipe parameters to session metadata
      await updateSessionUserRecipeValues({
        path: { session_id: chat.sessionId },
        body: { userRecipeValues: inputValues },
      });
    } catch (error) {
      console.error('Failed to update system prompt with parameters:', error);
    }
  };

  const handleRecipeCancel = (onCancel?: () => void) => {
    setIsRecipeWarningModalOpen(false);
    // Delegate window management to the caller
    if (onCancel) {
      onCancel();
    } else {
      window.electron.closeWindow();
    }
  };

  const handleRecipeAccept = (onAccept: () => Promise<void>) => {
    onAccept().then(() => {
      setIsRecipeWarningModalOpen(false);
    });
  };

  const handleAutoExecution = (
    append: (message: Message) => void,
    isLoading: boolean,
    onAutoExecute?: () => void
  ) => {
    if (
      finalRecipeConfig?.isScheduledExecution &&
      finalRecipeConfig?.prompt &&
      (!requiresParameters || chat.recipeParameters) &&
      messages.length === 0 &&
      !isLoading &&
      readyForAutoUserPrompt &&
      recipeAccepted
    ) {
      const finalPrompt = chat.recipeParameters
        ? substituteParameters(finalRecipeConfig.prompt, chat.recipeParameters)
        : finalRecipeConfig.prompt;

      const userMessage = createUserMessage(finalPrompt);
      append(userMessage);
      onAutoExecute?.();
    }
  };

  const handleRecipeCreated = (recipe: Recipe, onRecipeCreated?: (recipe: Recipe) => void) => {
    // Delegate toast/notification responsibility to the caller
    onRecipeCreated?.(recipe);
  };

  return {
    // Modal states
    isParameterModalOpen,
    setIsParameterModalOpen,
    isRecipeWarningModalOpen,
    setIsRecipeWarningModalOpen,
    readyForAutoUserPrompt,

    // Handlers
    handleParameterSubmit,
    handleRecipeCancel,
    handleRecipeAccept,
    handleAutoExecution,
    handleRecipeCreated,
  };
}
