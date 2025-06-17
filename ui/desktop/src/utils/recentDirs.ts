import fs from 'fs';
import path from 'path';
import { app } from 'electron';

const RECENT_DIRS_FILE = path.join(app.getPath('userData'), 'recent-dirs.json');
const MAX_RECENT_DIRS = 10;

interface RecentDirs {
  dirs: string[];
}

export function loadRecentDirs(): string[] {
  try {
    if (fs.existsSync(RECENT_DIRS_FILE)) {
      const data = fs.readFileSync(RECENT_DIRS_FILE, 'utf8');
      const recentDirs: RecentDirs = JSON.parse(data);

      // Filter out invalid directories (non-existent or not directories)
      const validDirs = recentDirs.dirs.filter((dir) => {
        try {
          const stats = fs.statSync(dir);
          return stats.isDirectory();
        } catch (error) {
          // Directory doesn't exist or can't be accessed
          console.warn(`Removing invalid recent directory: ${dir}`);
          return false;
        }
      });

      // Save the cleaned list back if it changed
      if (validDirs.length !== recentDirs.dirs.length) {
        fs.writeFileSync(RECENT_DIRS_FILE, JSON.stringify({ dirs: validDirs }, null, 2));
      }

      return validDirs;
    }
  } catch (error) {
    console.error('Error loading recent directories:', error);
  }
  return [];
}

export function addRecentDir(dir: string): void {
  try {
    // Validate that the path is actually a directory before adding it
    try {
      const stats = fs.statSync(dir);
      if (!stats.isDirectory()) {
        console.warn(`Cannot add recent directory: ${dir} is not a directory`);
        return;
      }
    } catch (error) {
      console.warn(`Cannot add recent directory: ${dir} does not exist or cannot be accessed`);
      return;
    }

    let dirs = loadRecentDirs();
    // Remove the directory if it already exists
    dirs = dirs.filter((d) => d !== dir);
    // Add the new directory at the beginning
    dirs.unshift(dir);
    // Keep only the most recent MAX_RECENT_DIRS
    dirs = dirs.slice(0, MAX_RECENT_DIRS);

    fs.writeFileSync(RECENT_DIRS_FILE, JSON.stringify({ dirs }, null, 2));
  } catch (error) {
    console.error('Error saving recent directory:', error);
  }
}
