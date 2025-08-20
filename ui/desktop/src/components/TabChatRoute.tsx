import React from 'react';
import { useNavigate, useLocation } from 'react-router-dom';
import { type View, ViewOptions } from '../App';
import { TabChatPair } from './TabChat';

const TabChatRoute = ({
  chat,
  setChat,
  setPairChat,
  setIsGoosehintsModalOpen,
}: {
  chat: any;
  setChat: (chat: any) => void;
  setPairChat: (chat: any) => void;
  setIsGoosehintsModalOpen: (isOpen: boolean) => void;
}) => {
  const navigate = useNavigate();
  
  return (
    <TabChatPair
      chat={chat}
      setChat={setChat}
      setView={(view: View, options?: ViewOptions) => {
        // Convert view to route navigation
        switch (view) {
          case 'chat':
            navigate('/');
            break;
          case 'pair':
            navigate('/pair', { state: options });
            break;
          case 'settings':
            navigate('/settings', { state: options });
            break;
          case 'sessions':
            navigate('/sessions');
            break;
          case 'schedules':
            navigate('/schedules');
            break;
          case 'recipes':
            navigate('/recipes');
            break;
          case 'permission':
            navigate('/permission', { state: options });
            break;
          case 'ConfigureProviders':
            navigate('/configure-providers');
            break;
          case 'sharedSession':
            navigate('/shared-session', { state: options });
            break;
          case 'recipeEditor':
            navigate('/recipe-editor', { state: options });
            break;
          case 'welcome':
            navigate('/welcome');
            break;
          default:
            navigate('/');
        }
      }}
      setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
    />
  );
};

export default TabChatRoute;
