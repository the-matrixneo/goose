import React from 'react';

export const MainPanelLayout: React.FC<{
  children: React.ReactNode;
}> = ({ children }) => {
  return (
    <div className={` h-dvh `}>
      <div className={`flex flex-col bg-background-default flex-1 min-w-0 h-full`}>{children}</div>
    </div>
  );
};
