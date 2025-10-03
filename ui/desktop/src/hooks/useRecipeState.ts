import { useEffect, useMemo, useState } from 'react';
import { Recipe, scanRecipe } from '../recipe';
import { filterValidUsedParameters, substituteParameters } from '../utils/providerUtils';

/**
 * Recipe state hook with explicit, predictable behavior
 */
export function useRecipeState(recipe: Recipe | null) {
  const [recipeAccepted, setRecipeAccepted] = useState(false);
  const [hasSecurityWarnings, setHasSecurityWarnings] = useState(false);
  const [recipeError, setRecipeError] = useState<string | null>(null);

  // Reset acceptance state when recipe changes
  useEffect(() => {
    setRecipeAccepted(false);
    setHasSecurityWarnings(false);
    setRecipeError(null);
  }, [recipe?.title, recipe?.instructions, recipe?.prompt]);

  // Check if recipe needs acceptance
  useEffect(() => {
    const checkRecipeAcceptance = async () => {
      if (!recipe) {
        setRecipeAccepted(false);
        return;
      }

      try {
        const hasAccepted = await window.electron.hasAcceptedRecipeBefore(recipe);

        if (hasAccepted) {
          setRecipeAccepted(true);
        } else {
          // Need to show warning modal - scan for security issues
          const securityScanResult = await scanRecipe(recipe);
          setHasSecurityWarnings(securityScanResult.has_security_warnings);
          // Don't auto-accept - let UI handle the modal
        }
      } catch (error) {
        console.error('Error checking recipe acceptance:', error);
        setHasSecurityWarnings(false);
        // Don't auto-accept - let UI handle the modal
      }
    };

    checkRecipeAcceptance();
  }, [recipe]);

  // Filter parameters to only show valid ones that are actually used in the recipe
  const filteredParameters = useMemo(() => {
    if (!recipe?.parameters) {
      return [];
    }
    return filterValidUsedParameters(recipe.parameters, {
      prompt: recipe.prompt || undefined,
      instructions: recipe.instructions || undefined,
      activities: recipe.activities || undefined,
    });
  }, [recipe]);

  // Check if template variables are actually used in the recipe content
  const requiresParameters = useMemo(() => {
    return filteredParameters.length > 0;
  }, [filteredParameters]);

  // Check if all required parameters have been filled in
  const hasAllRequiredParameters = useMemo(() => {
    return (recipeParameters: Record<string, string> | null) => {
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
    };
  }, [filteredParameters, requiresParameters]);

  // Generate initial prompt based on recipe and parameters
  const getInitialPrompt = useMemo(() => {
    return (recipeParameters: Record<string, string> | null) => {
      if (!recipe?.prompt || !recipeAccepted || recipe?.isScheduledExecution) {
        return '';
      }

      if (requiresParameters && recipeParameters) {
        return substituteParameters(recipe.prompt, recipeParameters);
      }

      return recipe.prompt;
    };
  }, [recipe, recipeAccepted, requiresParameters]);

  const acceptRecipe = async () => {
    try {
      if (recipe) {
        await window.electron.recordRecipeHash(recipe);
        setRecipeAccepted(true);
      }
    } catch (error) {
      console.error('Error recording recipe hash:', error);
      setRecipeAccepted(true); // Accept anyway to not block user
    }
  };

  return {
    recipe,
    recipeAccepted,
    hasSecurityWarnings,
    recipeError,
    setRecipeError,
    filteredParameters,
    requiresParameters,
    hasAllRequiredParameters,
    getInitialPrompt,
    acceptRecipe,
  };
}
