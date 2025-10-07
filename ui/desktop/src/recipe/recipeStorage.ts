import { listRecipes, RecipeManifestResponse } from '../api';
import { Recipe } from './index';

// TODO: Lifei Remove this
export interface SavedRecipe {
  name: string;
  recipe: Recipe;
  isGlobal: boolean;
  lastModified: Date;
  isArchived?: boolean;
  filename: string; // The actual filename used
}

/**
 * Parse a lastModified value that could be a string or Date
 */
function parseLastModified(val: string | Date): Date {
  return val instanceof Date ? val : new Date(val);
}

/**
 * Get the storage directory path for recipes
 */
export function getStorageDirectory(isGlobal: boolean): string {
  if (isGlobal) {
    return '~/.config/goose/recipes';
  } else {
    // For directory recipes, build absolute path using working directory
    const workingDir = window.appConfig.get('GOOSE_WORKING_DIR') as string;
    return `${workingDir}/.goose/recipes`;
  }
}

export async function listSavedRecipes(): Promise<RecipeManifestResponse[]> {
  try {
    const listRecipeResponse = await listRecipes();
    return listRecipeResponse?.data?.recipe_manifest_responses ?? [];
  } catch (error) {
    console.warn('Failed to list saved recipes:', error);
    return [];
  }
}

export function convertToLocaleDateString(lastModified: string): string {
  if (lastModified) {
    return parseLastModified(lastModified).toLocaleDateString();
  }
  return '';
}
