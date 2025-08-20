import React from 'react';
import { FolderDot } from 'lucide-react';
import { Tooltip, TooltipContent, TooltipProvider, TooltipTrigger } from '../ui/Tooltip';

interface DirSwitcherProps {
  className?: string;
}

export const DirSwitcher: React.FC<DirSwitcherProps> = ({ className = '' }) => {
  const workingDir = window.appConfig.get('GOOSE_WORKING_DIR') as string;

  const handleDirectoryChange = async () => {
    window.electron.directoryChooser(true);
  };

  const handleDirectoryClick = async (event: React.MouseEvent) => {
    const isCmdOrCtrlClick = event.metaKey || event.ctrlKey;

    if (isCmdOrCtrlClick) {
      event.preventDefault();
      event.stopPropagation();
      await window.electron.openDirectoryInExplorer(workingDir);
    } else {
      await handleDirectoryChange();
    }
  };

  return (
    <TooltipProvider>
      <Tooltip>
        <TooltipTrigger asChild>
          <button
            className={`z-[100] hover:cursor-pointer text-text-default/70 hover:text-text-default text-xs flex items-center transition-colors pl-1 [&>svg]:size-4 ${className}`}
            onClick={handleDirectoryClick}
          >
            <FolderDot className="mr-1" size={16} />
            <div className="max-w-[200px] truncate [direction:rtl]">
              {workingDir}
            </div>
          </button>
        </TooltipTrigger>
        <TooltipContent side="top">
          {workingDir}
        </TooltipContent>
      </Tooltip>
    </TooltipProvider>
  );
};
