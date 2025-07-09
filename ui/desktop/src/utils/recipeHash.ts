import fs from 'node:fs/promises';
import path from 'node:path';
import crypto from 'crypto';
import { app } from 'electron';

// File to store recipe hashes
const RECIPE_HASHES_FILE = 'recipe_hashes.json';

interface RecipeHash {
  hash: string;
  firstSeenAt: string;
  lastExecutedAt: string;
  executionCount: number;
}

interface RecipeHashes {
  [hash: string]: RecipeHash;
}

/**
 * Get the path to the recipe hashes file
 */
export async function getRecipeHashesPath(): Promise<string> {
  const userDataPath = app.getPath('userData');
  const hashesPath = path.join(userDataPath, RECIPE_HASHES_FILE);
  
  // Ensure the directory exists
  await fs.mkdir(path.dirname(hashesPath), { recursive: true });
  
  return hashesPath;
}

/**
 * Load stored recipe hashes
 */
export async function loadRecipeHashes(): Promise<RecipeHashes> {
  try {
    const hashesPath = await getRecipeHashesPath();
    const data = await fs.readFile(hashesPath, 'utf8');
    return JSON.parse(data);
  } catch (error) {
    if (error && typeof error === 'object' && 'code' in error && error.code === 'ENOENT') {
      // File doesn't exist yet, return empty object
      return {};
    }
    throw error;
  }
}

/**
 * Save recipe hashes to storage
 */
export async function saveRecipeHashes(hashes: RecipeHashes): Promise<void> {
  const hashesPath = await getRecipeHashesPath();
  await fs.writeFile(hashesPath, JSON.stringify(hashes, null, 2));
}

/**
 * Calculate hash for a recipe configuration
 */
export function calculateRecipeHash(recipeConfig: unknown): string {
  const hash = crypto.createHash('sha256');
  hash.update(JSON.stringify(recipeConfig));
  return hash.digest('hex');
}

/**
 * Check if a recipe has been seen before and record this execution
 * Returns true if recipe has been seen before, false if it's new
 */
export async function checkAndRecordRecipe(recipeConfig: unknown): Promise<boolean> {
  const hash = calculateRecipeHash(recipeConfig);
  const hashes = await loadRecipeHashes();
  
  const now = new Date().toISOString();
  
  if (hash in hashes) {
    // Update existing recipe hash record
    hashes[hash] = {
      ...hashes[hash],
      lastExecutedAt: now,
      executionCount: (hashes[hash].executionCount || 0) + 1
    };
    await saveRecipeHashes(hashes);
    return true;
  }
  
  // Record new recipe hash
  hashes[hash] = {
    hash,
    firstSeenAt: now,
    lastExecutedAt: now,
    executionCount: 1
  };
  await saveRecipeHashes(hashes);
  return false;
}
