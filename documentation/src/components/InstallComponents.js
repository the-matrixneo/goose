import InstallInstructions from '@site/src/components/InstallInstructions';

// Simple wrapper components for common use cases
export const DesktopInstall = ({ os, showUpdateTip = true }) => (
  <InstallInstructions 
    type="install" 
    interface="desktop" 
    os={os} 
    showUpdateTip={showUpdateTip} 
  />
);

export const CLIInstall = ({ os, showUpdateTip = true, showPrerequisites = true, showWSL = true }) => (
  <InstallInstructions 
    type="install" 
    interface="cli" 
    os={os} 
    showUpdateTip={showUpdateTip}
    showPrerequisites={showPrerequisites}
    showWSL={showWSL}
  />
);

export const DesktopUpdate = ({ os }) => (
  <InstallInstructions 
    type="update" 
    interface="desktop" 
    os={os} 
  />
);

export const CLIUpdate = ({ os, showOptions = true, showWSL = true }) => (
  <InstallInstructions 
    type="update" 
    interface="cli" 
    os={os} 
    showOptions={showOptions}
    showWSL={showWSL}
  />
);
