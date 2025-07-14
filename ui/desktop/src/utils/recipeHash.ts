import fs from 'node:fs/promises';
import path from 'node:path';
import crypto from 'crypto';
import { app } from 'electron';

export async function getRecipeHashesDir(): Promise<string> {
  const userDataPath = app.getPath('userData');
  const hashesDir = path.join(userDataPath, 'recipe_hashes');
  await fs.mkdir(hashesDir, { recursive: true });
  return hashesDir;
}

export function calculateRecipeHash(recipeConfig: unknown): string {
  const hash = crypto.createHash('sha256');
  hash.update(JSON.stringify(recipeConfig));
  return hash.digest('hex');
}

export async function hasAcceptedRecipeBefore(recipeConfig: unknown): Promise<boolean> {
  const hash = calculateRecipeHash(recipeConfig);
  const hashFile = path.join(await getRecipeHashesDir(), `${hash}.hash`);
  try {
    await fs.access(hashFile);
    return true;
  } catch (error) {
    if (error && typeof error === 'object' && 'code' in error && error.code === 'ENOENT') {
      return false;
    }
    throw error;
  }
}

export async function recordRecipeHash(recipeConfig: unknown): Promise<boolean> {
  const hash = calculateRecipeHash(recipeConfig);
  const dir = await getRecipeHashesDir();
  const filePath = path.join(dir, `${hash}.hash`);
  await fs.writeFile(filePath, new Date().toISOString());
  return true;
}
