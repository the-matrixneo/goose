import React from 'react';

export const MainPanelLayout: React.FC<{
  children: React.ReactNode;
  disableAnimation?: boolean;
}> = ({ children, disableAnimation = false }) => {
  const animationClasses = disableAnimation
    ? ''
    : 'animate-in fade-in slide-in-from-right-8 duration-500';

  return (
    <div
      className={`flex flex-col flex-1 min-w-0 h-dvh shadow-default bg-background-default ${animationClasses} pl-4 pr-2`}
    >
      {children}
    </div>
  );
};
