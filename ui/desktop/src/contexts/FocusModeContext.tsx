import React, { createContext, useContext, useState, ReactNode } from 'react';

interface FocusModeContextType {
  isInFocusMode: boolean;
  setIsInFocusMode: (isInFocusMode: boolean) => void;
}

const FocusModeContext = createContext<FocusModeContextType | undefined>(undefined);

interface FocusModeProviderProps {
  children: ReactNode;
}

export const FocusModeProvider: React.FC<FocusModeProviderProps> = ({ children }) => {
  const [isInFocusMode, setIsInFocusMode] = useState(false);

  const value: FocusModeContextType = {
    isInFocusMode,
    setIsInFocusMode,
  };

  return <FocusModeContext.Provider value={value}>{children}</FocusModeContext.Provider>;
};

export const useFocusMode = (): FocusModeContextType => {
  const context = useContext(FocusModeContext);
  if (context === undefined) {
    throw new Error('useFocusMode must be used within a FocusModeProvider');
  }
  return context;
};
