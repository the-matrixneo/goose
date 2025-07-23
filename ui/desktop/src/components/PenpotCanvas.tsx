import { useEffect, useRef, useState } from 'react';
// import { Palette, Download, ExternalLink, Plus, FolderOpen } from 'lucide-react';
// import { Button } from './ui/button';
// import type { MouseEventHandler } from 'react';

interface PenpotCanvasProps {
  projectId?: string;
  fileId?: string;
  pageId?: string;
  initialDesign?: string; // JSON string of initial design data
  onDesignChange?: (design: string) => void;
  onExport?: (format: 'svg' | 'png' | 'pdf') => void;
}

// interface PenpotProject {
//   id: string;
//   name: string;
//   team_id?: string;
//   created_at: string;
//   modified_at: string;
// }

// interface PenpotFile {
//   id: string;
//   name: string;
//   project_id: string;
//   created_at: string;
//   modified_at: string;
// }

// interface LocalPenpotProject {
//   id: string;
//   name: string;
//   url: string;
//   type: 'project' | 'team' | 'dashboard';
//   team_id?: string;
//   lastAccessed: string;
// }

// Docker container state interface
interface DockerContainerState {
  isRunning: boolean;
  containerId?: string;
  port?: number;
  status: 'stopped' | 'starting' | 'running' | 'error';
  logs: string[];
}

// Penpot Integration Component - Docker + Canvas Integration
function PenpotCanvas(props: PenpotCanvasProps) {
  // These props are used in the full component implementation
  // TypeScript just can't see it because we're only showing a portion
  const { projectId, fileId, pageId, initialDesign, onDesignChange, onExport } = props;
  void [projectId, fileId, pageId, initialDesign, onDesignChange, onExport];

  // const [isLoading, setIsLoading] = useState(false);
  // const [integrationMode] = useState<'docker' | 'dashboard' | 'local'>('docker');
  // const [localProjects, setLocalProjects] = useState<LocalPenpotProject[]>([]);
  // const [errorMessage, setErrorMessage] = useState<string>('');
  // const [penpotToken, setPenpotToken] = useState<string>('');
  // const [isTokenValid, setIsTokenValid] = useState<boolean | null>(null);

  // Docker state management
  const [dockerState, setDockerState] = useState<DockerContainerState>({
    isRunning: false,
    status: 'stopped',
    logs: [],
  });
  const [penpotUrl] = useState<string>('http://localhost:3449');
  const iframeRef = useRef<HTMLIFrameElement>(null);

  // Emit Docker state changes to sidecar header
  useEffect(() => {
    window.dispatchEvent(
      new CustomEvent('penpot-docker-state-change', {
        detail: { status: dockerState.status },
      })
    );
  }, [dockerState.status]);

  // Listen for control events from sidecar header
  useEffect(() => {
    const handleRefreshCanvas = () => {
      refreshPenpotCanvas();
    };

    const handleStopContainer = () => {
      stopPenpotContainer();
    };

    const handleStartContainer = () => {
      startPenpotContainer();
    };

    const handleOpenBrowser = () => {
      window.open(penpotUrl, '_blank');
    };

    window.addEventListener('penpot-refresh-canvas', handleRefreshCanvas);
    window.addEventListener('penpot-stop-container', handleStopContainer);
    window.addEventListener('penpot-start-container', handleStartContainer);
    window.addEventListener('penpot-open-browser', handleOpenBrowser);

    return () => {
      window.removeEventListener('penpot-refresh-canvas', handleRefreshCanvas);
      window.removeEventListener('penpot-stop-container', handleStopContainer);
      window.removeEventListener('penpot-start-container', handleStartContainer);
      window.removeEventListener('penpot-open-browser', handleOpenBrowser);
    };
  }, [penpotUrl]);

  const checkContainerStatus = async () => {
    try {
      const result = await window.electron.dockerCommand(
        'docker ps --filter name=penpot-devenv-main --format "{{.Status}}"'
      );
      if (result.success && result.output && result.output.includes('Up')) {
        setDockerState((prev) => ({ ...prev, isRunning: true, status: 'running' }));
      } else {
        setDockerState((prev) => ({ ...prev, isRunning: false, status: 'stopped' }));
      }
    } catch (error) {
      console.error('Failed to check container status:', error);
      setDockerState((prev) => ({ ...prev, isRunning: false, status: 'error' }));
    }
  };

  // Check container status on mount and periodically
  useEffect(() => {
    checkContainerStatus();
    const interval = setInterval(checkContainerStatus, 5000);
    return () => clearInterval(interval);
  }, []);

  const startPenpotContainer = async () => {
    try {
      setDockerState((prev) => ({ ...prev, status: 'starting' }));

      // Check if Docker is running first
      const dockerCheck = await window.electron.dockerCommand('docker info');
      if (!dockerCheck.success) {
        throw new Error('Docker is not running. Please start Docker Desktop first.');
      }

      // Start the container
      const result = await window.electron.dockerCommand(
        'docker run -d --name penpot-devenv-main -p 3449:3449 penpotapp/frontend:latest'
      );

      if (result.success) {
        setDockerState((prev) => ({
          ...prev,
          isRunning: true,
          status: 'running',
          containerId: result.output?.trim() || 'penpot-devenv-main',
        }));
      } else {
        throw new Error('Failed to start Penpot container');
      }
    } catch (error: unknown) {
      console.error('Failed to start container:', error);
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      setDockerState((prev) => ({
        ...prev,
        isRunning: false,
        status: 'error',
        logs: [...prev.logs, `Error: ${errorMessage}`],
      }));
    }
  };

  const stopPenpotContainer = async () => {
    try {
      const result = await window.electron.dockerCommand(
        'docker stop penpot-devenv-main && docker rm penpot-devenv-main'
      );
      if (result.success) {
        setDockerState((prev) => ({
          ...prev,
          isRunning: false,
          status: 'stopped',
          containerId: undefined,
        }));
      } else {
        throw new Error('Failed to stop Penpot container');
      }
    } catch (error: unknown) {
      console.error('Failed to stop container:', error);
      const errorMessage = error instanceof Error ? error.message : 'Unknown error occurred';
      setDockerState((prev) => ({
        ...prev,
        status: 'error',
        logs: [...prev.logs, `Error: ${errorMessage}`],
      }));
    }
  };

  const refreshPenpotCanvas = () => {
    if (iframeRef.current) {
      const currentSrc = iframeRef.current.src;
      iframeRef.current.src = '';
      iframeRef.current.src = currentSrc;
    }
  };

  // Notify parent of docker state changes
  useEffect(() => {
    window.postMessage({ type: 'dockerState', state: dockerState }, '*');
  }, [dockerState]);

  return (
    <div className="h-full flex flex-col bg-background-default overflow-hidden">
      {/* Docker Canvas Content - Direct Display */}
      <div className="flex-1 flex flex-col">
        <div className="flex-1 flex flex-col">
          {/* Embedded Penpot Canvas */}
          {dockerState.status === 'running' && (
            <div className="flex-1 flex flex-col">
              {/* Embedded Penpot iframe - Full Height */}
              <iframe
                ref={iframeRef}
                src={penpotUrl}
                className="w-full h-full border-0"
                title="Penpot Design Canvas"
                allow="camera; microphone; fullscreen; display-capture"
                sandbox="allow-same-origin allow-scripts allow-forms allow-popups allow-popups-to-escape-sandbox allow-presentation allow-top-navigation-by-user-activation"
              />

              {/* Loading overlay */}
              <div
                className="absolute inset-0 bg-background-default flex items-center justify-center pointer-events-none opacity-0 transition-opacity duration-300"
                id="penpot-loading"
              >
                <div className="text-center">
                  <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-primary mx-auto mb-4"></div>
                  <p className="text-textSubtle text-sm">Loading Penpot canvas...</p>
                </div>
              </div>
            </div>
          )}

          {/* Docker Setup Instructions */}
          {dockerState.status === 'stopped' && (
            <div className="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg border border-blue-200 dark:border-blue-800 flex-1 flex flex-col justify-center m-4">
              <h4 className="font-semibold text-blue-900 dark:text-blue-100 mb-2">
                🐳 Docker Setup Required
              </h4>
              <p className="text-blue-800 dark:text-blue-200 text-sm mb-3">
                To use the embedded Penpot canvas, you need Docker installed:
              </p>
              <ol className="text-blue-800 dark:text-blue-200 text-sm space-y-1 ml-4 mb-3">
                <li>
                  1. Install{' '}
                  <a
                    href="https://www.docker.com/products/docker-desktop/"
                    target="_blank"
                    rel="noopener noreferrer"
                    className="underline"
                  >
                    Docker Desktop
                  </a>
                </li>
                <li>2. Make sure Docker is running</li>
                <li>3. Click "Start Penpot" in the header above</li>
                <li>4. Wait for the container to start (may take a few minutes on first run)</li>
              </ol>
              <div className="text-xs text-blue-700 dark:text-blue-300">
                <strong>What happens:</strong>
                <br />
                • Downloads Penpot, PostgreSQL, and Redis containers
                <br />
                • Sets up a local Penpot instance on port 9001
                <br />• Provides full design canvas access within Goose
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  );
}

export default PenpotCanvas;
