import { ipcMain, app, BrowserWindow } from 'electron';
import fs from 'node:fs/promises';
import path from 'node:path';
import crypto from 'crypto';

function calculateRecipeHash(recipe: unknown): string {
  const hash = crypto.createHash('sha256');
  hash.update(JSON.stringify(recipe));
  return hash.digest('hex');
}

async function getRecipeHashesDir(): Promise<string> {
  const userDataPath = app.getPath('userData');
  const hashesDir = path.join(userDataPath, 'recipe_hashes');
  await fs.mkdir(hashesDir, { recursive: true });
  return hashesDir;
}

ipcMain.handle('has-accepted-recipe-before', async (_event, recipe) => {
  const hash = calculateRecipeHash(recipe);
  const hashFile = path.join(await getRecipeHashesDir(), `${hash}.hash`);
  try {
    await fs.access(hashFile);
    return true;
  } catch (err) {
    if (typeof err === 'object' && err !== null && 'code' in err && err.code === 'ENOENT') {
      return false;
    }
    throw err;
  }
});

ipcMain.handle('record-recipe-hash', async (_event, recipe) => {
  const hash = calculateRecipeHash(recipe);
  const filePath = path.join(await getRecipeHashesDir(), `${hash}.hash`);
  const timestamp = new Date().toISOString();
  await fs.writeFile(filePath, timestamp);
  return true;
});

ipcMain.on('close-window', () => {
  const currentWindow = BrowserWindow.getFocusedWindow();
  if (currentWindow) {
    currentWindow.close();
  }
});
