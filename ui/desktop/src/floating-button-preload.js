const { contextBridge, ipcRenderer } = require('electron');

// Parse config from command line arguments
const config = JSON.parse(process.argv.find(arg => arg.startsWith('{')) || '{}');

console.log('Floating button preload - config:', config);

// Expose floating button API
contextBridge.exposeInMainWorld('electronFloating', {
  checkDocking: () => {
    // This will be called when drag ends to check for docking
    ipcRenderer.send('check-floating-dock', config.buttonId);
  },
  focusParent: () => {
    // Focus the parent window when clicked
    ipcRenderer.send('focus-parent-window', config.parentWindowId);
  },
  getConfig: () => {
    console.log('getConfig called, returning:', config);
    return config;
  }
});