import { useEffect, useRef, useState } from 'react';
import { IpcRendererEvent } from 'electron';
import { HashRouter, Routes, Route, useNavigate, useLocation } from 'react-router-dom';
import { openSharedSessionFromDeepLink, type SessionLinksViewOptions } from './sessionLinks';
import { type SharedSessionDetails } from './sharedSessions';
import { initializeSystem } from './utils/providerUtils';
import { initializeCostDatabase } from './utils/costDatabase';
import { ErrorUI } from './components/ErrorBoundary';
import { ConfirmationModal } from './components/ui/ConfirmationModal';
import { ToastContainer } from 'react-toastify';
import { extractExtensionName } from './components/settings/extensions/utils';
import { GoosehintsModal } from './components/GoosehintsModal';
import { type ExtensionConfig } from './extensions';
import AnnouncementModal from './components/AnnouncementModal';
import { generateSessionId } from './sessions';

import Hub, { type ChatType } from './components/hub';
import Pair from './components/pair';
import SettingsView, { SettingsViewOptions } from './components/settings/SettingsView';
import SessionsView from './components/sessions/SessionsView';
import SharedSessionView from './components/sessions/SharedSessionView';
import SchedulesView from './components/schedule/SchedulesView';
import ProviderSettings from './components/settings/providers/ProviderSettingsPage';
import { useChat } from './hooks/useChat';
import { AppLayout } from './components/Layout/AppLayout';
import { ChatProvider } from './contexts/ChatContext';

import 'react-toastify/dist/ReactToastify.css';
import { useConfig, MalformedConfigError } from './components/ConfigContext';
import { ModelAndProviderProvider } from './components/ModelAndProviderContext';
import { addExtensionFromDeepLink as addExtensionFromDeepLinkV2 } from './components/settings/extensions';
import {
  backupConfig,
  initConfig,
  readAllConfig,
  recoverConfig,
  validateConfig,
} from './api/sdk.gen';
import PermissionSettingsView from './components/settings/permission/PermissionSetting';

import { type SessionDetails } from './sessions';
import ExtensionsView, { ExtensionsViewOptions } from './components/extensions/ExtensionsView';
// import ProjectsContainer from './components/projects/ProjectsContainer';
import RecipesView from './components/RecipesView';
import RecipeEditor from './components/RecipeEditor';

export type View =
  | 'welcome'
  | 'chat'
  | 'pair'
  | 'settings'
  | 'extensions'
  | 'moreModels'
  | 'configureProviders'
  | 'configPage'
  | 'ConfigureProviders'
  | 'settingsV2'
  | 'sessions'
  | 'schedules'
  | 'sharedSession'
  | 'loading'
  | 'recipeEditor'
  | 'recipes'
  | 'permission';
// | 'projects';

export type ViewOptions = {
  // Settings view options
  extensionId?: string;
  showEnvVars?: boolean;
  deepLinkConfig?: ExtensionConfig;

  // Session view options
  resumedSession?: SessionDetails;
  sessionDetails?: SessionDetails;
  error?: string;
  shareToken?: string;
  baseUrl?: string;

  // Recipe editor options
  config?: unknown;

  // Permission view options
  parentView?: View;

  // Generic options
  [key: string]: unknown;
};

export type ViewConfig = {
  view: View;
  viewOptions?: ViewOptions;
};

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

  return (
    <Hub
      readyForAutoUserPrompt={true}
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
          default:
            navigate('/');
        }
      }}
      setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
    />
  );
};

const PairRouteWrapper = ({
  chat,
  setChat,
  setIsGoosehintsModalOpen,
}: {
  chat: ChatType;
  setChat: (chat: ChatType) => void;
  setIsGoosehintsModalOpen: (isOpen: boolean) => void;
}) => {
  const navigate = useNavigate();

  return (
    <Pair
      readyForAutoUserPrompt={true}
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
          default:
            navigate('/');
        }
      }}
      setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
    />
  );
};

const SettingsRoute = () => {
  const location = useLocation();
  const navigate = useNavigate();

  return (
    <SettingsView
      onClose={() => navigate('/')}
      setView={(view: View, options?: ViewOptions) => {
        // Convert view to route navigation
        switch (view) {
          case 'chat':
            navigate('/');
            break;
          case 'pair':
            navigate('/pair');
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
          default:
            navigate('/');
        }
      }}
      viewOptions={(location.state as SettingsViewOptions) || {}}
    />
  );
};

const SessionsRoute = () => {
  const navigate = useNavigate();

  return (
    <SessionsView
      setView={(view: View, options?: ViewOptions) => {
        // Convert view to route navigation
        switch (view) {
          case 'chat':
            navigate('/', { state: options });
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
          default:
            navigate('/');
        }
      }}
    />
  );
};

const SchedulesRoute = () => {
  const navigate = useNavigate();
  return <SchedulesView onClose={() => navigate('/')} />;
};

const RecipesRoute = () => {
  return <RecipesView />;
};

const RecipeEditorRoute = () => {
  const location = useLocation();
  const config = location.state?.config || window.electron.getConfig().recipeConfig;

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

  return <ProviderSettings onClose={() => navigate('/')} isOnboarding={false} />;
};

const WelcomeRoute = () => {
  const navigate = useNavigate();

  return <ProviderSettings onClose={() => navigate('/')} isOnboarding={true} />;
};

// Wrapper component for SharedSessionRoute to access parent state
const SharedSessionRouteWrapper = ({
  isLoadingSharedSession,
  setIsLoadingSharedSession,
  sharedSessionError,
}: {
  isLoadingSharedSession: boolean;
  setIsLoadingSharedSession: (loading: boolean) => void;
  sharedSessionError: string | null;
}) => {
  const location = useLocation();
  const navigate = useNavigate();

  const sessionDetails = location.state?.sessionDetails as SharedSessionDetails | null;
  const error = location.state?.error || sharedSessionError;
  const shareToken = location.state?.shareToken;
  const baseUrl = location.state?.baseUrl;

  return (
    <SharedSessionView
      session={sessionDetails}
      isLoading={isLoadingSharedSession}
      error={error}
      onBack={() => navigate('/sessions')}
      onRetry={async () => {
        if (shareToken && baseUrl) {
          setIsLoadingSharedSession(true);
          try {
            await openSharedSessionFromDeepLink(
              `goose://sessions/${shareToken}`,
              (view: View, _options?: SessionLinksViewOptions) => {
                // Convert view to route navigation
                switch (view) {
                  case 'chat':
                    navigate('/', { state: _options });
                    break;
                  case 'pair':
                    navigate('/pair', { state: _options });
                    break;
                  case 'settings':
                    navigate('/settings', { state: _options });
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
                    navigate('/permission', { state: _options });
                    break;
                  case 'ConfigureProviders':
                    navigate('/configure-providers');
                    break;
                  case 'sharedSession':
                    navigate('/shared-session', { state: _options });
                    break;
                  case 'recipeEditor':
                    navigate('/recipe-editor', { state: _options });
                    break;
                  default:
                    navigate('/');
                }
              },
              baseUrl
            );
          } catch (error) {
            console.error('Failed to retry loading shared session:', error);
          } finally {
            setIsLoadingSharedSession(false);
          }
        }
      }}
    />
  );
};

const ExtensionsRoute = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const viewOptions = location.state as ExtensionsViewOptions;

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
      viewOptions={viewOptions || {}}
    />
  );
};

// const ProjectsRoute = () => {
//   const navigate = useNavigate();
//
//   const setView = (view: View, viewOptions?: ViewOptions) => {
//     // Convert view to route navigation
//     switch (view) {
//       case 'chat':
//         navigate('/');
//         break;
//       case 'pair':
//         navigate('/pair', { state: viewOptions });
//         break;
//       case 'settings':
//         navigate('/settings', { state: viewOptions });
//         break;
//       case 'sessions':
//         navigate('/sessions');
//         break;
//       case 'schedules':
//         navigate('/schedules');
//         break;
//       case 'recipes':
//         navigate('/recipes');
//         break;
//       case 'permission':
//         navigate('/permission', { state: viewOptions });
//         break;
//       case 'ConfigureProviders':
//         navigate('/configure-providers');
//         break;
//       case 'sharedSession':
//         navigate('/shared-session', { state: viewOptions });
//         break;
//       case 'recipeEditor':
//         navigate('/recipe-editor', { state: viewOptions });
//         break;
//       case 'welcome':
//         navigate('/welcome');
//         break;
//       default:
//         navigate('/');
//     }
//   };
//
//   return (
//     <React.Suspense fallback={<div>Loading projects...</div>}>
//       <ProjectsContainer setView={setView} />
//     </React.Suspense>
//   );
// };

export default function App() {
  const [fatalError, setFatalError] = useState<string | null>(null);
  const [modalVisible, setModalVisible] = useState(false);
  const [pendingLink, setPendingLink] = useState<string | null>(null);
  const [modalMessage, setModalMessage] = useState<string>('');
  const [extensionConfirmLabel, setExtensionConfirmLabel] = useState<string>('');
  const [extensionConfirmTitle, setExtensionConfirmTitle] = useState<string>('');
  const [isLoadingSession, setIsLoadingSession] = useState(false);
  const [isGoosehintsModalOpen, setIsGoosehintsModalOpen] = useState(false);
  const [isLoadingSharedSession, setIsLoadingSharedSession] = useState(false);
  const [sharedSessionError, setSharedSessionError] = useState<string | null>(null);

  // Add separate state for pair chat to maintain its own conversation
  const [pairChat, setPairChat] = useState<ChatType>({
    id: generateSessionId(),
    title: 'Pair Chat',
    messages: [],
    messageHistoryIndex: 0,
  });

  const { getExtensions, addExtension, read } = useConfig();
  const initAttemptedRef = useRef(false);

  // Create a setView function for useChat hook - we'll use window.history instead of navigate
  const setView = (view: View, viewOptions: ViewOptions = {}) => {
    console.log(`Setting view to: ${view}`, viewOptions);
    // Convert view to route navigation using hash routing
    switch (view) {
      case 'chat':
        window.location.hash = '#/';
        break;
      case 'pair':
        window.location.hash = '#/pair';
        break;
      case 'settings':
        window.location.hash = '#/settings';
        break;
      case 'extensions':
        window.location.hash = '#/extensions';
        break;
      case 'sessions':
        window.location.hash = '#/sessions';
        break;
      case 'schedules':
        window.location.hash = '#/schedules';
        break;
      case 'recipes':
        window.location.hash = '#/recipes';
        break;
      case 'permission':
        window.location.hash = '#/permission';
        break;
      case 'ConfigureProviders':
        window.location.hash = '#/configure-providers';
        break;
      case 'sharedSession':
        window.location.hash = '#/shared-session';
        break;
      case 'recipeEditor':
        window.location.hash = '#/recipe-editor';
        break;
      case 'welcome':
        window.location.hash = '#/welcome';
        break;
      default:
        window.location.hash = '#/';
    }
  };

  const { chat, setChat } = useChat({ setIsLoadingSession, setView, setPairChat });

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
    if (initAttemptedRef.current) {
      console.log('Initialization already attempted, skipping...');
      return;
    }
    initAttemptedRef.current = true;

    console.log(`Initializing app with settings v2`);

    const urlParams = new URLSearchParams(window.location.search);
    const viewType = urlParams.get('view');
    const recipeConfig = window.appConfig.get('recipeConfig');

    if (viewType) {
      if (viewType === 'recipeEditor' && recipeConfig) {
        console.log('Setting view to recipeEditor with config:', recipeConfig);
        // Handle recipe editor deep link
        window.history.replaceState({ config: recipeConfig }, '', '/recipe-editor');
      } else {
        // Handle other deep links by redirecting to appropriate route
        const routeMap: Record<string, string> = {
          chat: '/',
          pair: '/pair',
          settings: '/settings',
          sessions: '/sessions',
          schedules: '/schedules',
          recipes: '/recipes',
          permission: '/permission',
          ConfigureProviders: '/configure-providers',
          sharedSession: '/shared-session',
          recipeEditor: '/recipe-editor',
          welcome: '/welcome',
        };

        const route = routeMap[viewType];
        if (route) {
          window.history.replaceState({}, '', route);
        }
      }
      return;
    }

    const initializeApp = async () => {
      try {
        // Initialize cost database early to pre-load pricing data
        initializeCostDatabase().catch((error) => {
          console.error('Failed to initialize cost database:', error);
        });

        await initConfig();
        try {
          await readAllConfig({ throwOnError: true });
        } catch (error) {
          const configVersion = localStorage.getItem('configVersion');
          const shouldMigrateExtensions = !configVersion || parseInt(configVersion, 10) < 3;
          if (shouldMigrateExtensions) {
            await backupConfig({ throwOnError: true });
            await initConfig();
          } else {
            // Config appears corrupted, try recovery
            console.warn('Config file appears corrupted, attempting recovery...');
            try {
              // First try to validate the config
              try {
                await validateConfig({ throwOnError: true });
                // Config is valid but readAllConfig failed for another reason
                throw new Error('Unable to read config file, it may be malformed');
              } catch (validateError) {
                console.log('Config validation failed, attempting recovery...');

                // Try to recover the config
                try {
                  const recoveryResult = await recoverConfig({ throwOnError: true });
                  console.log('Config recovery result:', recoveryResult);

                  // Try to read config again after recovery
                  try {
                    await readAllConfig({ throwOnError: true });
                    console.log('Config successfully recovered and loaded');
                  } catch (retryError) {
                    console.warn('Config still corrupted after recovery, reinitializing...');
                    await initConfig();
                  }
                } catch (recoverError) {
                  console.warn('Config recovery failed, reinitializing...');
                  await initConfig();
                }
              }
            } catch (recoveryError) {
              console.error('Config recovery process failed:', recoveryError);
              throw new Error('Unable to read config file, it may be malformed');
            }
          }
        }

        if (recipeConfig === null) {
          setFatalError('Cannot read recipe config. Please check the deeplink and try again.');
          return;
        }

        const config = window.electron.getConfig();
        const provider = (await read('GOOSE_PROVIDER', false)) ?? config.GOOSE_DEFAULT_PROVIDER;
        const model = (await read('GOOSE_MODEL', false)) ?? config.GOOSE_DEFAULT_MODEL;

        if (provider && model) {
          // Navigate to chat route
          window.history.replaceState({}, '', '/');
          try {
            await initializeSystem(provider as string, model as string, {
              getExtensions,
              addExtension,
            });
          } catch (error) {
            console.error('Error in initialization:', error);
            if (error instanceof MalformedConfigError) {
              throw error;
            }
            // Navigate to welcome route
            window.history.replaceState({}, '', '/welcome');
          }
        } else {
          // Navigate to welcome route
          window.history.replaceState({}, '', '/welcome');
        }
      } catch (error) {
        console.error('Fatal error during initialization:', error);
        setFatalError(error instanceof Error ? error.message : 'Unknown error occurred');
      }
    };

    initializeApp();
  }, [getExtensions, addExtension, read]);

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

  useEffect(() => {
    const handleOpenSharedSession = async (_event: IpcRendererEvent, ...args: unknown[]) => {
      const link = args[0] as string;
      window.electron.logInfo(`Opening shared session from deep link ${link}`);
      setIsLoadingSession(true);
      setSharedSessionError(null);
      try {
        await openSharedSessionFromDeepLink(
          link,
          (view: View, _options?: SessionLinksViewOptions) => {
            // Convert view to route navigation
            switch (view) {
              case 'chat':
                window.history.replaceState({}, '', '/');
                break;
              case 'settings':
                window.history.replaceState({}, '', '/settings');
                break;
              case 'sessions':
                window.history.replaceState({}, '', '/sessions');
                break;
              case 'schedules':
                window.history.replaceState({}, '', '/schedules');
                break;
              case 'recipes':
                window.history.replaceState({}, '', '/recipes');
                break;
              case 'permission':
                window.history.replaceState({}, '', '/permission');
                break;
              case 'ConfigureProviders':
                window.history.replaceState({}, '', '/configure-providers');
                break;
              case 'sharedSession':
                window.history.replaceState({}, '', '/shared-session');
                break;
              case 'recipeEditor':
                window.history.replaceState({}, '', '/recipe-editor');
                break;
              default:
                window.history.replaceState({}, '', '/');
            }
          }
        );
      } catch (error) {
        console.error('Unexpected error opening shared session:', error);
        window.history.replaceState({}, '', '/sessions');
      } finally {
        setIsLoadingSession(false);
      }
    };
    window.electron.on('open-shared-session', handleOpenSharedSession);
    return () => {
      window.electron.off('open-shared-session', handleOpenSharedSession);
    };
  }, [setSharedSessionError]);

  useEffect(() => {
    console.log('Setting up keyboard shortcuts');
    const handleKeyDown = (event: KeyboardEvent) => {
      const isMac = window.electron.platform === 'darwin';
      if ((isMac ? event.metaKey : event.ctrlKey) && event.key === 'n') {
        event.preventDefault();
        try {
          const workingDir = window.appConfig.get('GOOSE_WORKING_DIR');
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

  useEffect(() => {
    console.log('Setting up fatal error handler');
    const handleFatalError = (_event: IpcRendererEvent, ...args: unknown[]) => {
      const errorMessage = args[0] as string;
      console.error('Encountered a fatal error: ', errorMessage);
      console.error('Is loading session:', isLoadingSession);
      setFatalError(errorMessage);
    };
    window.electron.on('fatal-error', handleFatalError);
    return () => {
      window.electron.off('fatal-error', handleFatalError);
    };
  }, [isLoadingSession]);

  useEffect(() => {
    console.log('Setting up view change handler');
    const handleSetView = (_event: IpcRendererEvent, ...args: unknown[]) => {
      const newView = args[0] as View;
      const section = args[1] as string | undefined;
      console.log(
        `Received view change request to: ${newView}${section ? `, section: ${section}` : ''}`
      );

      if (section && newView === 'settings') {
        window.history.replaceState({}, '', `/settings?section=${section}`);
      } else {
        window.history.replaceState({}, '', `/${newView}`);
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
  const STRICT_ALLOWLIST = config.GOOSE_ALLOWLIST_WARNING === true ? false : true;

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
              const isCommandAllowed = allowedCommands.some((allowedCmd) =>
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
        await addExtensionFromDeepLinkV2(pendingLink, addExtension, (view: string, _options) => {
          window.history.replaceState({}, '', `/${view}`);
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

  if (isLoadingSession)
    return (
      <div className="flex justify-center items-center py-12">
        <div className="animate-spin rounded-full h-8 w-8 border-t-2 border-b-2 border-textStandard"></div>
      </div>
    );

  return (
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
            <Route
              path="/"
              element={
                <ChatProvider chat={chat} setChat={setChat}>
                  <AppLayout setIsGoosehintsModalOpen={setIsGoosehintsModalOpen} />
                </ChatProvider>
              }
            >
              <Route
                index
                element={
                  <HubRouteWrapper
                    chat={chat}
                    setChat={setChat}
                    setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
                  />
                }
              />
              <Route
                path="pair"
                element={
                  <ChatProvider chat={pairChat} setChat={setPairChat}>
                    <PairRouteWrapper
                      chat={pairChat}
                      setChat={setPairChat}
                      setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
                    />
                  </ChatProvider>
                }
              />
              <Route path="settings" element={<SettingsRoute />} />
              <Route path="extensions" element={<ExtensionsRoute />} />
              <Route path="sessions" element={<SessionsRoute />} />
              <Route path="schedules" element={<SchedulesRoute />} />
              <Route path="recipes" element={<RecipesRoute />} />
              <Route path="recipe-editor" element={<RecipeEditorRoute />} />
              <Route
                path="shared-session"
                element={
                  <SharedSessionRouteWrapper
                    isLoadingSharedSession={isLoadingSharedSession}
                    setIsLoadingSharedSession={setIsLoadingSharedSession}
                    sharedSessionError={sharedSessionError}
                  />
                }
              />
              <Route path="permission" element={<PermissionRoute />} />
              <Route path="configure-providers" element={<ConfigureProvidersRoute />} />
              <Route path="welcome" element={<WelcomeRoute />} />
              {/*<Route*/}
              {/*  path="projects"*/}
              {/*  element={*/}
              {/*    <ChatProvider chat={chat} setChat={setChat}>*/}
              {/*      <ProjectsRoute />*/}
              {/*    </ChatProvider>*/}
              {/*  }*/}
              {/*/>*/}
            </Route>
          </Routes>
        </div>
        {isGoosehintsModalOpen && (
          <GoosehintsModal
            directory={window.appConfig.get('GOOSE_WORKING_DIR') as string}
            setIsGoosehintsModalOpen={setIsGoosehintsModalOpen}
          />
        )}
      </HashRouter>
      <AnnouncementModal />
    </ModelAndProviderProvider>
  );
}
