import { useCallback, useEffect, useMemo, useState } from 'react';
import { IpcRendererEvent } from 'electron';
import { HashRouter, Routes, Route, useNavigate, useLocation } from 'react-router-dom';
import { ErrorUI } from './components/ErrorBoundary';
import { ConfirmationModal } from './components/ui/ConfirmationModal';
import { ToastContainer } from 'react-toastify';
import { extractExtensionName } from './components/settings/extensions/utils';
import { GoosehintsModal } from './components/GoosehintsModal';
import AnnouncementModal from './components/AnnouncementModal';
import { generateSessionId } from './sessions';
import ProviderGuard from './components/ProviderGuard';

import { ChatType } from './types/chat';
import Hub from './components/hub';
import Pair from './components/pair';
import SettingsView, { SettingsViewOptions } from './components/settings/SettingsView';
import SessionsView from './components/sessions/SessionsView';
import SchedulesView from './components/schedule/SchedulesView';
import ProviderSettings from './components/settings/providers/ProviderSettingsPage';
import { AppLayout } from './components/Layout/AppLayout';
import { ChatProvider } from './contexts/ChatContext';
import { DraftProvider } from './contexts/DraftContext';

import 'react-toastify/dist/ReactToastify.css';
import { useConfig } from './components/ConfigContext';
import { ModelAndProviderProvider } from './components/ModelAndProviderContext';
import { addExtensionFromDeepLink as addExtensionFromDeepLinkV2 } from './components/settings/extensions';
import PermissionSettingsView from './components/settings/permission/PermissionSetting';

import ExtensionsView, { ExtensionsViewOptions } from './components/extensions/ExtensionsView';
import { Recipe } from './recipe';
import RecipesView from './components/RecipesView';
import RecipeEditor from './components/RecipeEditor';
import { createNavigationHandler, View, ViewOptions } from './utils/navigationUtils';

// Route Components
const HubRouteWrapper = ({
  chat,
  setChat,
  setIsGoosehintsModalOpen,
}: {
  chat: ChatType;
  setChat: (chat: ChatType) => void;
  setIsGoosehintsModalOpen: (isOpen: boolean) => void;
}) => {
  const navigate = useNavigate();
  const setView = useMemo(() => createNavigationHandler(navigate), [navigate]);

  return (
    <Hub
      readyForAutoUserPrompt={true}
      chat={chat}
      setChat={setChat}
      setView={setView}
      setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
    />
  );
};

const PairRouteWrapper = ({
  chat,
  setChat,
  setIsGoosehintsModalOpen,
  setAgentWaitingMessage,
  setFatalError,
}: {
  chat: ChatType;
  setChat: (chat: ChatType) => void;
  setIsGoosehintsModalOpen: (isOpen: boolean) => void;
  setAgentWaitingMessage: (msg: string | null) => void;
  setFatalError: (value: ((prevState: string | null) => string | null) | string | null) => void;
}) => {
  const navigate = useNavigate();
  const setView = useMemo(() => createNavigationHandler(navigate), [navigate]);

  return (
    <Pair
      chat={chat}
      setChat={setChat}
      setView={setView}
      setFatalError={setFatalError}
      setAgentWaitingMessage={setAgentWaitingMessage}
      setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
    />
  );
};

const SettingsRoute = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const setView = useMemo(() => createNavigationHandler(navigate), [navigate]);

  // Get viewOptions from location.state or history.state
  const viewOptions =
    (location.state as SettingsViewOptions) || (window.history.state as SettingsViewOptions) || {};
  return <SettingsView onClose={() => navigate('/')} setView={setView} viewOptions={viewOptions} />;
};

const SessionsRoute = () => {
  const navigate = useNavigate();
  const setView = useMemo(() => createNavigationHandler(navigate), [navigate]);

  return <SessionsView setView={setView} />;
};

const SchedulesRoute = () => {
  const navigate = useNavigate();
  return <SchedulesView onClose={() => navigate('/')} />;
};

const RecipesRoute = () => {
  const navigate = useNavigate();

  return (
    <RecipesView
      onLoadRecipe={(recipe) => {
        // Navigate to pair view with the recipe configuration in state
        navigate('/pair', {
          state: {
            recipeConfig: recipe,
            // Reset the pair chat to start fresh with the recipe
            resetChat: true,
          },
        });
      }}
    />
  );
};

const RecipeEditorRoute = () => {
  const location = useLocation();

  // Check for config from multiple sources:
  // 1. Location state (from navigation)
  // 2. localStorage (from "View Recipe" button)
  // 3. Window electron config (from deeplinks)
  let config = location.state?.config;

  if (!config) {
    const storedConfig = localStorage.getItem('viewRecipeConfig');
    if (storedConfig) {
      try {
        config = JSON.parse(storedConfig);
        // Clear the stored config after using it
        localStorage.removeItem('viewRecipeConfig');
      } catch (error) {
        console.error('Failed to parse stored recipe config:', error);
      }
    }
  }

  if (!config) {
    const electronConfig = window.electron.getConfig();
    config = electronConfig.recipe;
  }

  return <RecipeEditor config={config} />;
};

const PermissionRoute = () => {
  const location = useLocation();
  const navigate = useNavigate();
  const parentView = location.state?.parentView as View;
  const parentViewOptions = location.state?.parentViewOptions as ViewOptions;

  return (
    <PermissionSettingsView
      onClose={() => {
        // Navigate back to parent view with options
        switch (parentView) {
          case 'chat':
            navigate('/');
            break;
          case 'pair':
            navigate('/pair');
            break;
          case 'settings':
            navigate('/settings', { state: parentViewOptions });
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
          default:
            navigate('/');
        }
      }}
    />
  );
};

const ConfigureProvidersRoute = () => {
  const navigate = useNavigate();

  return (
    <div className="w-screen h-screen bg-background-default">
      <ProviderSettings
        onClose={() => navigate('/settings', { state: { section: 'models' } })}
        isOnboarding={false}
      />
    </div>
  );
};

const WelcomeRoute = () => {
  const navigate = useNavigate();

  return (
    <div className="w-screen h-screen bg-background-default">
      <ProviderSettings onClose={() => navigate('/')} isOnboarding={true} />
    </div>
  );
};

const ExtensionsRoute = () => {
  const navigate = useNavigate();
  const location = useLocation();

  // Get viewOptions from location.state or history.state (for deep link extensions)
  const viewOptions =
    (location.state as ExtensionsViewOptions) ||
    (window.history.state as ExtensionsViewOptions) ||
    {};

  return (
    <ExtensionsView
      onClose={() => navigate(-1)}
      setView={(view, options) => {
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
          default:
            navigate('/');
        }
      }}
      viewOptions={viewOptions}
    />
  );
};

export default function App() {
  const [fatalError, setFatalError] = useState<string | null>(null);
  const [modalVisible, setModalVisible] = useState(false);
  const [pendingLink, setPendingLink] = useState<string | null>(null);
  const [modalMessage, setModalMessage] = useState<string>('');
  const [extensionConfirmLabel, setExtensionConfirmLabel] = useState<string>('');
  const [extensionConfirmTitle, setExtensionConfirmTitle] = useState<string>('');
  const [isGoosehintsModalOpen, setIsGoosehintsModalOpen] = useState(false);
  const [agentWaitingMessage, setAgentWaitingMessage] = useState<string | null>(null);

  const [chat, _setChat] = useState<ChatType>({
    sessionId: generateSessionId(),
    title: 'Pair Chat',
    messages: [],
    messageHistoryIndex: 0,
    recipeConfig: null,
  });

  const setChat = useCallback<typeof _setChat>(
    (update) => {
      console.log('setChat called with:', update);
      _setChat(update);
    },
    [_setChat]
  );

  const { addExtension } = useConfig();

  function extractCommand(link: string): string {
    const url = new URL(link);
    const cmd = url.searchParams.get('cmd') || 'Unknown Command';
    const args = url.searchParams.getAll('arg').map(decodeURIComponent);
    return `${cmd} ${args.join(' ')}`.trim();
  }

  function extractRemoteUrl(link: string): string | null {
    const url = new URL(link);
    return url.searchParams.get('url');
  }

  useEffect(() => {
    console.log('Sending reactReady signal to Electron');
    try {
      window.electron.reactReady();
    } catch (error) {
      console.error('Error sending reactReady:', error);
      setFatalError(
        `React ready notification failed: ${error instanceof Error ? error.message : 'Unknown error'}`
      );
    }
  }, []);

  // Handle URL parameters and deeplinks on app startup
  useEffect(() => {
    const urlParams = new URLSearchParams(window.location.search);
    const viewType = urlParams.get('view');
    const resumeSessionId = urlParams.get('resumeSessionId');
    const recipeConfig = window.appConfig?.get('recipe');

    if (resumeSessionId || (recipeConfig && typeof recipeConfig === 'object')) {
      window.location.hash = '#/pair';
      window.history.replaceState({ resumeSessionId: resumeSessionId }, '', '#/pair');
      return;
    }

    if (!viewType) {
      if (window.location.hash === '' || window.location.hash === '#') {
        window.location.hash = '#/';
        window.history.replaceState({}, '', '#/');
      }
    } else {
      if (viewType === 'recipeEditor' && recipeConfig) {
        window.location.hash = '#/recipe-editor';
        window.history.replaceState({ config: recipeConfig }, '', '#/recipe-editor');
      } else {
        const routeMap: Record<string, string> = {
          chat: '#/',
          pair: '#/pair',
          settings: '#/settings',
          sessions: '#/sessions',
          schedules: '#/schedules',
          recipes: '#/recipes',
          permission: '#/permission',
          ConfigureProviders: '#/configure-providers',
          sharedSession: '#/shared-session',
          recipeEditor: '#/recipe-editor',
          welcome: '#/welcome',
        };

        const route = routeMap[viewType];
        if (route) {
          window.location.hash = route;
          window.history.replaceState({}, '', route);
        }
      }
    }
  }, []);

  // Handle recipe decode events from main process
  useEffect(() => {
    const handleLoadRecipeDeeplink = (_event: IpcRendererEvent, ...args: unknown[]) => {
      const recipeDeeplink = args[0] as string;
      const scheduledJobId = args[1] as string | undefined;

      // Store the deeplink info in app config for processing
      const config = window.electron.getConfig();
      config.recipeDeeplink = recipeDeeplink;
      if (scheduledJobId) {
        config.scheduledJobId = scheduledJobId;
      }

      // Navigate to pair view to handle the recipe loading
      if (window.location.hash !== '#/pair') {
        window.location.hash = '#/pair';
      }
    };

    const handleRecipeDecoded = (_event: IpcRendererEvent, ...args: unknown[]) => {
      const decodedRecipe = args[0] as Recipe;

      // Update the pair chat with the decoded recipe
      setChat((prevChat) => ({
        ...prevChat,
        recipeConfig: decodedRecipe,
        title: decodedRecipe.title || 'Recipe Chat',
        messages: [], // Start fresh for recipe
        messageHistoryIndex: 0,
      }));

      // Navigate to pair view if not already there
      if (window.location.hash !== '#/pair') {
        window.location.hash = '#/pair';
      }
    };

    const handleRecipeDecodeError = (_event: IpcRendererEvent, ...args: unknown[]) => {
      const errorMessage = args[0] as string;
      console.error('[App] Recipe decode error:', errorMessage);

      // Show error to user - you could add a toast notification here
      // For now, just log the error and navigate to recipes page
      window.location.hash = '#/recipes';
    };

    window.electron.on('load-recipe-deeplink', handleLoadRecipeDeeplink);
    window.electron.on('recipe-decoded', handleRecipeDecoded);
    window.electron.on('recipe-decode-error', handleRecipeDecodeError);

    return () => {
      window.electron.off('load-recipe-deeplink', handleLoadRecipeDeeplink);
      window.electron.off('recipe-decoded', handleRecipeDecoded);
      window.electron.off('recipe-decode-error', handleRecipeDecodeError);
    };
  }, [setChat, chat.sessionId]);

  useEffect(() => {
    console.log('Setting up keyboard shortcuts');
    const handleKeyDown = (event: KeyboardEvent) => {
      const isMac = window.electron.platform === 'darwin';
      if ((isMac ? event.metaKey : event.ctrlKey) && event.key === 'n') {
        event.preventDefault();
        try {
          const workingDir = window.appConfig?.get('GOOSE_WORKING_DIR');
          console.log(`Creating new chat window with working dir: ${workingDir}`);
          window.electron.createChatWindow(undefined, workingDir as string);
        } catch (error) {
          console.error('Error creating new window:', error);
        }
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => {
      window.removeEventListener('keydown', handleKeyDown);
    };
  }, []);

  // Prevent default drag and drop behavior globally to avoid opening files in new windows
  // but allow our React components to handle drops in designated areas
  useEffect(() => {
    const preventDefaults = (e: globalThis.DragEvent) => {
      // Only prevent default if we're not over a designated drop zone
      const target = e.target as HTMLElement;
      const isOverDropZone = target.closest('[data-drop-zone="true"]') !== null;

      if (!isOverDropZone) {
        e.preventDefault();
        e.stopPropagation();
      }
    };

    const handleDragOver = (e: globalThis.DragEvent) => {
      // Always prevent default for dragover to allow dropping
      e.preventDefault();
      e.stopPropagation();
    };

    const handleDrop = (e: globalThis.DragEvent) => {
      // Only prevent default if we're not over a designated drop zone
      const target = e.target as HTMLElement;
      const isOverDropZone = target.closest('[data-drop-zone="true"]') !== null;

      if (!isOverDropZone) {
        e.preventDefault();
        e.stopPropagation();
      }
    };

    // Add event listeners to document to catch drag events
    document.addEventListener('dragenter', preventDefaults, false);
    document.addEventListener('dragleave', preventDefaults, false);
    document.addEventListener('dragover', handleDragOver, false);
    document.addEventListener('drop', handleDrop, false);

    return () => {
      document.removeEventListener('dragenter', preventDefaults, false);
      document.removeEventListener('dragleave', preventDefaults, false);
      document.removeEventListener('dragover', handleDragOver, false);
      document.removeEventListener('drop', handleDrop, false);
    };
  }, []);

  useEffect(() => {
    const handleFatalError = (_event: IpcRendererEvent, ...args: unknown[]) => {
      const errorMessage = args[0] as string;
      console.error('Encountered a fatal error:', errorMessage);
      setFatalError(errorMessage);
    };
    window.electron.on('fatal-error', handleFatalError);
    return () => {
      window.electron.off('fatal-error', handleFatalError);
    };
  });

  useEffect(() => {
    const handleSetView = (_event: IpcRendererEvent, ...args: unknown[]) => {
      const newView = args[0] as View;
      const section = args[1] as string | undefined;
      console.log(
        `Received view change request to: ${newView}${section ? `, section: ${section}` : ''}`
      );

      if (section && newView === 'settings') {
        window.location.hash = `#/settings?section=${section}`;
      } else {
        window.location.hash = `#/${newView}`;
      }
    };
    const urlParams = new URLSearchParams(window.location.search);
    const viewFromUrl = urlParams.get('view');
    if (viewFromUrl) {
      const windowConfig = window.electron.getConfig();
      if (viewFromUrl === 'recipeEditor') {
        const initialViewOptions = {
          recipeConfig: JSON.stringify(windowConfig?.recipeConfig),
          view: viewFromUrl,
        };
        window.history.replaceState(
          {},
          '',
          `/recipe-editor?${new URLSearchParams(initialViewOptions).toString()}`
        );
      } else {
        window.history.replaceState({}, '', `/${viewFromUrl}`);
      }
    }
    window.electron.on('set-view', handleSetView);
    return () => window.electron.off('set-view', handleSetView);
  }, []);

  const config = window.electron.getConfig();
  const STRICT_ALLOWLIST = config.GOOSE_ALLOWLIST_WARNING !== true;

  useEffect(() => {
    console.log('Setting up extension handler');
    const handleAddExtension = async (_event: IpcRendererEvent, ...args: unknown[]) => {
      const link = args[0] as string;
      try {
        console.log(`Received add-extension event with link: ${link}`);
        const command = extractCommand(link);
        const remoteUrl = extractRemoteUrl(link);
        const extName = extractExtensionName(link);
        window.electron.logInfo(`Adding extension from deep link ${link}`);
        setPendingLink(link);
        let warningMessage = '';
        let label = 'OK';
        let title = 'Confirm Extension Installation';
        let isBlocked = false;
        let useDetailedMessage = false;
        if (remoteUrl) {
          useDetailedMessage = true;
        } else {
          try {
            const allowedCommands = await window.electron.getAllowedExtensions();
            if (allowedCommands && allowedCommands.length > 0) {
              const isCommandAllowed = allowedCommands.some((allowedCmd: string) =>
                command.startsWith(allowedCmd)
              );
              if (!isCommandAllowed) {
                useDetailedMessage = true;
                title = '⛔️ Untrusted Extension ⛔️';
                if (STRICT_ALLOWLIST) {
                  isBlocked = true;
                  label = 'Extension Blocked';
                  warningMessage =
                    '\n\n⛔️ BLOCKED: This extension command is not in the allowed list. ' +
                    'Installation is blocked by your administrator. ' +
                    'Please contact your administrator if you need this extension.';
                } else {
                  label = 'Override and install';
                  warningMessage =
                    '\n\n⚠️ WARNING: This extension command is not in the allowed list. ' +
                    'Installing extensions from untrusted sources may pose security risks. ' +
                    'Please contact an admin if you are unsure or want to allow this extension.';
                }
              }
            }
          } catch (error) {
            console.error('Error checking allowlist:', error);
          }
        }
        if (useDetailedMessage) {
          const detailedMessage = remoteUrl
            ? `You are about to install the ${extName} extension which connects to:\n\n${remoteUrl}\n\nThis extension will be able to access your conversations and provide additional functionality.`
            : `You are about to install the ${extName} extension which runs the command:\n\n${command}\n\nThis extension will be able to access your conversations and provide additional functionality.`;
          setModalMessage(`${detailedMessage}${warningMessage}`);
        } else {
          const messageDetails = `Command: ${command}`;
          setModalMessage(
            `Are you sure you want to install the ${extName} extension?\n\n${messageDetails}`
          );
        }
        setExtensionConfirmLabel(label);
        setExtensionConfirmTitle(title);
        if (isBlocked) {
          setPendingLink(null);
        }
        setModalVisible(true);
      } catch (error) {
        console.error('Error handling add-extension event:', error);
      }
    };
    window.electron.on('add-extension', handleAddExtension);
    return () => {
      window.electron.off('add-extension', handleAddExtension);
    };
  }, [STRICT_ALLOWLIST]);

  useEffect(() => {
    const handleFocusInput = (_event: IpcRendererEvent, ..._args: unknown[]) => {
      const inputField = document.querySelector('input[type="text"], textarea') as HTMLInputElement;
      if (inputField) {
        inputField.focus();
      }
    };
    window.electron.on('focus-input', handleFocusInput);
    return () => {
      window.electron.off('focus-input', handleFocusInput);
    };
  }, []);

  const handleConfirm = async () => {
    if (pendingLink) {
      console.log(`Confirming installation of extension from: ${pendingLink}`);
      setModalVisible(false);
      try {
        await addExtensionFromDeepLinkV2(pendingLink, addExtension, (view: string, options) => {
          console.log('Extension deep link handler called with view:', view, 'options:', options);
          switch (view) {
            case 'settings':
              window.location.hash = '#/extensions';
              // Store the config for the extensions route
              window.history.replaceState(options, '', '#/extensions');
              break;
            default:
              window.location.hash = `#/${view}`;
          }
        });
        console.log('Extension installation successful');
      } catch (error) {
        console.error('Failed to add extension:', error);
      } finally {
        setPendingLink(null);
      }
    } else {
      console.log('Extension installation blocked by allowlist restrictions');
      setModalVisible(false);
    }
  };

  const handleCancel = () => {
    console.log('Cancelled extension installation.');
    setModalVisible(false);
    setPendingLink(null);
  };

  if (fatalError) {
    return <ErrorUI error={new Error(fatalError)} />;
  }

  return (
    <DraftProvider>
      <ModelAndProviderProvider>
        <HashRouter>
          <ToastContainer
            aria-label="Toast notifications"
            toastClassName={() =>
              `relative min-h-16 mb-4 p-2 rounded-lg
               flex justify-between overflow-hidden cursor-pointer
               text-text-on-accent bg-background-inverse
              `
            }
            style={{ width: '380px' }}
            className="mt-6"
            position="top-right"
            autoClose={3000}
            closeOnClick
            pauseOnHover
          />
          {modalVisible && (
            <ConfirmationModal
              isOpen={modalVisible}
              message={modalMessage}
              confirmLabel={extensionConfirmLabel}
              title={extensionConfirmTitle}
              onConfirm={handleConfirm}
              onCancel={handleCancel}
            />
          )}
          <div className="relative w-screen h-screen overflow-hidden bg-background-muted flex flex-col">
            <div className="titlebar-drag-region" />
            <Routes>
              <Route path="welcome" element={<WelcomeRoute />} />
              <Route path="configure-providers" element={<ConfigureProvidersRoute />} />
              <Route
                path="/"
                element={
                  <ChatProvider
                    chat={chat}
                    setChat={setChat}
                    contextKey="hub"
                    agentWaitingMessage={agentWaitingMessage}
                  >
                    <AppLayout setIsGoosehintsModalOpen={setIsGoosehintsModalOpen} />
                  </ChatProvider>
                }
              >
                <Route
                  index
                  element={
                    <ProviderGuard>
                      <HubRouteWrapper
                        chat={chat}
                        setChat={setChat}
                        setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
                      />
                    </ProviderGuard>
                  }
                />
                <Route
                  path="pair"
                  element={
                    <ProviderGuard>
                      <ChatProvider
                        chat={chat}
                        setChat={setChat}
                        contextKey={`pair-${chat.sessionId}`}
                        agentWaitingMessage={agentWaitingMessage}
                        key={chat.sessionId}
                      >
                        <PairRouteWrapper
                          chat={chat}
                          setChat={setChat}
                          setFatalError={setFatalError}
                          setAgentWaitingMessage={setAgentWaitingMessage}
                          setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
                        />
                      </ChatProvider>
                    </ProviderGuard>
                  }
                />
                <Route
                  path="settings"
                  element={
                    <ProviderGuard>
                      <SettingsRoute />
                    </ProviderGuard>
                  }
                />
                <Route
                  path="extensions"
                  element={
                    <ProviderGuard>
                      <ExtensionsRoute />
                    </ProviderGuard>
                  }
                />
                <Route
                  path="sessions"
                  element={
                    <ProviderGuard>
                      <SessionsRoute />
                    </ProviderGuard>
                  }
                />
                <Route
                  path="schedules"
                  element={
                    <ProviderGuard>
                      <SchedulesRoute />
                    </ProviderGuard>
                  }
                />
                <Route
                  path="recipes"
                  element={
                    <ProviderGuard>
                      <RecipesRoute />
                    </ProviderGuard>
                  }
                />
                <Route
                  path="recipe-editor"
                  element={
                    <ProviderGuard>
                      <RecipeEditorRoute />
                    </ProviderGuard>
                  }
                />
                <Route
                  path="permission"
                  element={
                    <ProviderGuard>
                      <PermissionRoute />
                    </ProviderGuard>
                  }
                />
              </Route>
            </Routes>
          </div>
          {isGoosehintsModalOpen && (
            <GoosehintsModal
              directory={window.appConfig?.get('GOOSE_WORKING_DIR') as string}
              setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
            />
          )}
        </HashRouter>
        <AnnouncementModal />
      </ModelAndProviderProvider>
    </DraftProvider>
  );
}
