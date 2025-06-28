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
      className={`flex flex-col flex-1 min-w-0 h-[calc(100dvh-56px)] shadow-default bg-background-default mr-2 mb-2 rounded-xl ${animationClasses}`}
    >
      {children}
    </div>
  );
};
