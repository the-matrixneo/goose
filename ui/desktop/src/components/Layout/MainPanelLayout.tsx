import React from 'react';

export const MainPanelLayout: React.FC<{
  children: React.ReactNode;
}> = ({ children }) => {
  return (
    <div className={`h-dvh`}>
      {/* Padding top matches the app toolbar drag area */}
      <div className={`flex flex-col bg-background-default flex-1 min-w-0 h-full pt-[32px]`}>
        {children}
      </div>
    </div>
  );
};
