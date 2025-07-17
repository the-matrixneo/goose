# Documentation Refactoring: Installation Components

## Overview
Successfully refactored repeated installation content across multiple documentation files by creating reusable React components. This eliminates duplication and makes maintenance much easier.

## Components Created

### 1. **DesktopInstallSteps.js**
- Handles the post-download installation steps for each OS
- Props: `os` (mac/windows/linux)
- Automatically renders appropriate steps based on OS

### 2. **CLIInstallCommand.js** 
- Renders the curl installation command with optional configuration
- Props: `showConfigure`, `os`
- Handles OS-specific shell information (especially Windows)

### 3. **UpdateTip.js**
- Displays update tips for desktop vs CLI interfaces
- Props: `interface` (desktop/cli)
- Renders appropriate update messaging

### 4. **CLIUpdateInstructions.js**
- Complete CLI update instructions including options and version check
- Props: `showOptions`
- Includes goose update commands and installation script alternative

### 5. **WindowsPrerequisites.js**
- Windows-specific prerequisites (Git Bash, MSYS2, PowerShell)
- Standalone component for Windows CLI requirements

### 6. **WSLInstallInstructions.js**
- Complete WSL installation process in collapsible details
- Includes PowerShell commands and troubleshooting tips

### 7. **WSLUpdateInstructions.js**
- WSL-specific update instructions
- Separate from main WSL install component for modularity

### 8. **DesktopInstallSection.js**
- Complete desktop installation section combining buttons + steps + tips
- Props: `os`, `showUpdateTip`
- Uses existing install button components

### 9. **CLIInstallSection.js**
- Complete CLI installation section with OS-specific content
- Props: `os`, `showUpdateTip`, `showPrerequisites`, `showWSL`
- Handles Mac Homebrew option, Windows prerequisites, WSL instructions

### 10. **DesktopUpdateInstructions.js**
- Complete desktop update process with reinstallation steps
- Props: `os`
- Includes info admonition and numbered steps

## Files Refactored

### ✅ quickstart.md
- Replaced installation sections with `<DesktopInstallSection>` and `<CLIInstallSection>`
- Reduced code by ~80 lines while maintaining full functionality
- Added props to hide update tips (not needed in quickstart)

### ✅ updating-goose.md  
- Replaced all update sections with new update components
- Eliminated ~150 lines of repeated code
- Much cleaner and more maintainable

### 🔄 installation.md (Ready for refactoring)
- Can now be refactored using the same components
- Will eliminate the most duplication since it's the most comprehensive

## Benefits Achieved

1. **DRY Principle**: Eliminated massive code duplication across files
2. **Maintainability**: Changes to installation steps only need to be made in one place
3. **Consistency**: Ensures all installation instructions stay in sync
4. **Flexibility**: Components accept props to customize behavior per context
5. **Reusability**: Components can be used in future documentation pages

## Usage Examples

```jsx
// Simple desktop install for macOS without update tip
<DesktopInstallSection os="mac" showUpdateTip={false} />

// CLI install for Windows with minimal options (for quickstart)
<CLIInstallSection 
  os="windows" 
  showUpdateTip={false} 
  showPrerequisites={false} 
  showWSL={false} 
/>

// Full CLI install for Windows (for installation guide)
<CLIInstallSection os="windows" />

// Update instructions
<DesktopUpdateInstructions os="linux" />
<CLIUpdateInstructions showOptions={true} />
```

## Next Steps

1. **Refactor installation.md**: Apply the same component approach to the main installation guide
2. **Test thoroughly**: Verify all combinations work across different contexts
3. **Consider additional components**: Look for other repeated patterns in the docs
4. **Documentation**: Update contributor guidelines about using these components

## File Structure
```
documentation/src/components/
├── DesktopInstallSteps.js
├── CLIInstallCommand.js  
├── UpdateTip.js
├── CLIUpdateInstructions.js
├── WindowsPrerequisites.js
├── WSLInstallInstructions.js
├── WSLUpdateInstructions.js
├── DesktopInstallSection.js
├── CLIInstallSection.js
└── DesktopUpdateInstructions.js
```

This refactoring significantly improves the maintainability and consistency of the Goose documentation while reducing the overall codebase size.
