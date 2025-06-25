import React from 'react';

export const MainPanelLayout: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return (
    <div className="flex flex-col flex-1 min-w-0 h-[calc(100dvh-40px)] shadow-default bg-background-default mt-4 mr-2 mb-4 rounded-2xl animate-in fade-in slide-in-from-right-8 duration-500">
      {children}
    </div>
  );
};
