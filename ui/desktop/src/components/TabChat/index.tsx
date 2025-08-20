import React from 'react';
import { FocusModeProvider } from '../../contexts/FocusModeContext';
import TabChatPair from './TabChatPair';

// Wrap TabChatPair with FocusModeProvider to fix the context error
const TabChatPairWithFocusMode = (props) => {
  return (
    <FocusModeProvider>
      <TabChatPair {...props} />
    </FocusModeProvider>
  );
};

export { TabChatPairWithFocusMode as TabChatPair };
export { default as TabBar } from './TabBar';
export { default as TabPill } from './TabPill';
export { default as TabChatManager } from './TabChatManager';
export { default } from './TabChatPair';
