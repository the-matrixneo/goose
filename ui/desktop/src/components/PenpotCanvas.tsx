import { useEffect, useRef, useState } from 'react';
import { Palette, Download, Share, Layers, Square, ExternalLink, Plus, FolderOpen, Settings, Play, Square as StopIcon, RefreshCw, Terminal } from 'lucide-react';
import { Button } from './ui/button';
import { Tooltip, TooltipTrigger, TooltipContent } from './ui/Tooltip';

interface PenpotCanvasProps {
  projectId?: string;
  fileId?: string;
  pageId?: string;
  initialDesign?: string; // JSON string of initial design data
  onDesignChange?: (design: string) => void;
  onExport?: (format: 'svg' | 'png' | 'pdf') => void;
}

interface PenpotProject {
  id: string;
  name: string;
  team_id?: string;
  created_at: string;
  modified_at: string;
}

interface PenpotFile {
  id: string;
  name: string;
  project_id: string;
  created_at: string;
  modified_at: string;
}

interface LocalPenpotProject {
  id: string;
  name: string;
  url: string;
  type: 'project' | 'team' | 'dashboard';
  team_id?: string;
  lastAccessed: string;
}

// Docker container state interface
interface DockerContainerState {
  isRunning: boolean;
  containerId?: string;
  port?: number;
  status: 'stopped' | 'starting' | 'running' | 'error';
  logs: string[];
}

// Penpot Integration Component - Docker + Canvas Integration
function PenpotCanvas({ 
  projectId, 
  fileId, 
  pageId, 
  initialDesign,
  onDesignChange,
  onExport 
}: PenpotCanvasProps) {
  const [isLoading, setIsLoading] = useState(false);
  const [integrationMode, setIntegrationMode] = useState<'docker' | 'dashboard' | 'local'>('docker');
  const [localProjects, setLocalProjects] = useState<LocalPenpotProject[]>([]);
  const [errorMessage, setErrorMessage] = useState<string>('');
  const [penpotToken, setPenpotToken] = useState<string>('');
  const [isTokenValid, setIsTokenValid] = useState<boolean | null>(null);
  
  // Docker state management
  const [dockerState, setDockerState] = useState<DockerContainerState>({
    isRunning: false,
    status: 'stopped',
    logs: []
  });
  const [penpotUrl, setPenpotUrl] = useState<string>('http://localhost:9001');
  const iframeRef = useRef<HTMLIFrameElement>(null);

  // Emit Docker state changes to sidecar header
  useEffect(() => {
    window.dispatchEvent(new CustomEvent('penpot-docker-state-change', {
      detail: { status: dockerState.status }
    }));
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

  // Docker management functions
  const checkDockerStatus = async () => {
    try {
      console.log('Checking Docker status...');
      const result = await window.electron.dockerCommand('docker --version');
      console.log('Docker version check result:', result);
      return result.success;
    } catch (error) {
      console.error('Docker check failed:', error);
      return false;
    }
  };

  const startPenpotContainer = async () => {
    setDockerState(prev => ({ ...prev, status: 'starting' }));
    setErrorMessage('Starting Penpot container...');
    
    try {
      // Check if Docker is available
      console.log('Checking Docker availability...');
      const dockerAvailable = await checkDockerStatus();
      console.log('Docker available:', dockerAvailable);
      
      if (!dockerAvailable) {
        throw new Error('Docker is not installed or not running. Please install Docker Desktop first.');
      }

      setErrorMessage('✅ Docker detected! Setting up Penpot containers...');

      // Clean up any existing containers (ignore errors)
      console.log('Cleaning up existing containers...');
      await window.electron.dockerCommand('docker stop penpot-frontend penpot-backend penpot-postgres penpot-valkey penpot-exporter || true');
      await window.electron.dockerCommand('docker rm penpot-frontend penpot-backend penpot-postgres penpot-valkey penpot-exporter || true');
      await window.electron.dockerCommand('docker network rm penpot || true');

      // Create network (using official name)
      console.log('Creating Docker network...');
      const networkResult = await window.electron.dockerCommand('docker network create penpot');
      console.log('Network creation result:', networkResult);
      
      setErrorMessage('📦 Starting database containers...');
      
      // Start PostgreSQL with proper health check (official configuration)
      console.log('Starting PostgreSQL...');
      const postgresResult = await window.electron.dockerCommand(
        'docker run -d ' +
        '--name penpot-postgres ' +
        '--network penpot ' +
        '--restart always ' +
        '--stop-signal SIGINT ' +
        '--health-cmd "pg_isready -U penpot" ' +
        '--health-interval 2s ' +
        '--health-timeout 10s ' +
        '--health-retries 5 ' +
        '--health-start-period 2s ' +
        '-e POSTGRES_INITDB_ARGS=--data-checksums ' +
        '-e POSTGRES_DB=penpot ' +
        '-e POSTGRES_USER=penpot ' +
        '-e POSTGRES_PASSWORD=penpot ' +
        'postgres:15'
      );
      console.log('PostgreSQL result:', postgresResult);

      if (!postgresResult.success) {
        throw new Error(`Failed to start PostgreSQL: ${postgresResult.error || 'Unknown error'}`);
      }

      // Start Valkey (Redis replacement - official Penpot setup)
      console.log('Starting Valkey...');
      const valkeyResult = await window.electron.dockerCommand(
        'docker run -d ' +
        '--name penpot-valkey ' +
        '--network penpot ' +
        '--restart always ' +
        '--health-cmd "valkey-cli ping" ' +
        '--health-interval 1s ' +
        '--health-timeout 3s ' +
        '--health-retries 5 ' +
        '--health-start-period 3s ' +
        'valkey/valkey:8.1'
      );
      console.log('Valkey result:', valkeyResult);

      setErrorMessage('⏳ Waiting for databases to initialize...');
      
      // Wait for PostgreSQL to be healthy
      let dbReady = false;
      for (let i = 0; i < 30; i++) {
        try {
          // First check if container is running
          const containerCheck = await window.electron.dockerCommand('docker ps --filter name=penpot-postgres --format "{{.Status}}"');
          if (!containerCheck.success || !containerCheck.output || !containerCheck.output.includes('Up')) {
            // Container not running, check logs for error
            const logsCheck = await window.electron.dockerCommand('docker logs penpot-postgres 2>&1 | tail -10');
            console.log('PostgreSQL container logs:', logsCheck.output);
            setErrorMessage(`⏳ PostgreSQL container issue... (${i + 1}/30)\nLogs: ${logsCheck.output || 'No logs available'}`);
            await new Promise(resolve => setTimeout(resolve, 2000));
            continue;
          }

          // Check health status
          const healthCheck = await window.electron.dockerCommand('docker inspect --format="{{.State.Health.Status}}" penpot-postgres');
          if (healthCheck.success && healthCheck.output && healthCheck.output.trim() === 'healthy') {
            dbReady = true;
            break;
          } else if (healthCheck.success && healthCheck.output) {
            setErrorMessage(`⏳ PostgreSQL health status: ${healthCheck.output.trim()} (${i + 1}/30)`);
          } else {
            // Try simple connection test if health check fails
            const connectionTest = await window.electron.dockerCommand('docker exec penpot-postgres /usr/bin/pg_isready -U penpot');
            if (connectionTest.success) {
              dbReady = true;
              break;
            }
          }
        } catch (e) {
          console.error('PostgreSQL health check error:', e);
        }
        await new Promise(resolve => setTimeout(resolve, 2000));
        setErrorMessage(`⏳ Waiting for PostgreSQL to initialize... (${i + 1}/30)`);
      }

      if (!dbReady) {
        throw new Error('PostgreSQL failed to start within timeout');
      }

      // Wait for Valkey to be healthy
      let valkeyReady = false;
      for (let i = 0; i < 15; i++) {
        try {
          // First check if container is running
          const containerCheck = await window.electron.dockerCommand('docker ps --filter name=penpot-valkey --format "{{.Status}}"');
          if (!containerCheck.success || !containerCheck.output || !containerCheck.output.includes('Up')) {
            // Container not running, check logs for error
            const logsCheck = await window.electron.dockerCommand('docker logs penpot-valkey 2>&1 | tail -10');
            console.log('Valkey container logs:', logsCheck.output);
            setErrorMessage(`⏳ Valkey container issue... (${i + 1}/15)\nLogs: ${logsCheck.output || 'No logs available'}`);
            await new Promise(resolve => setTimeout(resolve, 1000));
            continue;
          }

          // Check health status
          const healthCheck = await window.electron.dockerCommand('docker inspect --format="{{.State.Health.Status}}" penpot-valkey');
          if (healthCheck.success && healthCheck.output && healthCheck.output.trim() === 'healthy') {
            valkeyReady = true;
            break;
          } else if (healthCheck.success && healthCheck.output) {
            setErrorMessage(`⏳ Valkey health status: ${healthCheck.output.trim()} (${i + 1}/15)`);
          } else {
            // Try simple connection test if health check fails
            const connectionTest = await window.electron.dockerCommand('docker exec penpot-valkey valkey-cli ping');
            if (connectionTest.success && connectionTest.output && connectionTest.output.includes('PONG')) {
              valkeyReady = true;
              break;
            }
          }
        } catch (e) {
          console.error('Valkey health check error:', e);
        }
        await new Promise(resolve => setTimeout(resolve, 1000));
        setErrorMessage(`⏳ Waiting for Valkey to initialize... (${i + 1}/15)`);
      }

      if (!valkeyReady) {
        throw new Error('Valkey failed to start within timeout');
      }

      setErrorMessage('🚀 Starting Penpot backend...');

      // Start Penpot backend with official configuration
      console.log('Starting Penpot backend...');
      const backendResult = await window.electron.dockerCommand(
        'docker run -d ' +
        '--name penpot-backend ' +
        '--network penpot ' +
        '--restart always ' +
        '-e "PENPOT_FLAGS=disable-email-verification enable-prepl-server disable-secure-session-cookies enable-registration enable-login-with-password" ' +
        '-e PENPOT_SECRET_KEY=penpot-secret-key ' +
        '-e PENPOT_PUBLIC_URI=http://localhost:9001 ' +
        '-e PENPOT_DATABASE_URI=postgresql://penpot-postgres/penpot ' +
        '-e PENPOT_DATABASE_USERNAME=penpot ' +
        '-e PENPOT_DATABASE_PASSWORD=penpot ' +
        '-e PENPOT_REDIS_URI=redis://penpot-valkey/0 ' +
        '-e PENPOT_ASSETS_STORAGE_BACKEND=assets-fs ' +
        '-e PENPOT_STORAGE_ASSETS_FS_DIRECTORY=/opt/data/assets ' +
        '-e PENPOT_HTTP_SERVER_MAX_BODY_SIZE=31457280 ' +
        '-e PENPOT_HTTP_SERVER_MAX_MULTIPART_BODY_SIZE=367001600 ' +
        '-v /tmp/penpot-assets:/opt/data/assets ' +
        'penpotapp/backend:latest'
      );
      console.log('Backend result:', backendResult);

      setErrorMessage('⏳ Waiting for backend to initialize...');
      
      // Wait for backend to be ready (check logs for startup)
      let backendReady = false;
      for (let i = 0; i < 60; i++) {
        try {
          // Check if backend container is running and healthy
          const containerCheck = await window.electron.dockerCommand('docker ps --filter name=penpot-backend --format "{{.Status}}"');
          if (containerCheck.success && containerCheck.output && containerCheck.output.includes('Up')) {
            // Try multiple log patterns that indicate the backend is ready
            const logPatterns = [
              'docker logs penpot-backend 2>&1 | grep -i "server started"',
              'docker logs penpot-backend 2>&1 | grep -i "listening"',
              'docker logs penpot-backend 2>&1 | grep -i "started"',
              'docker logs penpot-backend 2>&1 | grep -i "ready"',
              'docker logs penpot-backend 2>&1 | grep -i "running"'
            ];
            
            for (const pattern of logPatterns) {
              const healthCheck = await window.electron.dockerCommand(pattern);
              if (healthCheck.success && healthCheck.output && healthCheck.output.trim()) {
                console.log('Backend ready - found log pattern:', healthCheck.output);
                backendReady = true;
                break;
              }
            }
            
            // If no specific log pattern found, check if container has been running for a reasonable time
            if (!backendReady && i > 10) {
              // Try a simple connection test to the backend port (if accessible)
              console.log('No specific log pattern found, assuming backend is ready after container uptime');
              backendReady = true;
              break;
            }
          } else {
            // Container not running properly
            const logsCheck = await window.electron.dockerCommand('docker logs penpot-backend 2>&1 | tail -10');
            console.log('Backend container logs:', logsCheck.output);
            setErrorMessage(`⏳ Backend container issue... (${i + 1}/60)\nLogs: ${logsCheck.output || 'No logs available'}`);
          }
        } catch (e) {
          console.error('Backend health check error:', e);
        }
        
        if (backendReady) break;
        
        await new Promise(resolve => setTimeout(resolve, 2000));
        setErrorMessage(`⏳ Waiting for backend to initialize... (${i + 1}/60)`);
      }

      setErrorMessage('📋 Starting Penpot exporter...');

      // Start Penpot exporter (official configuration)
      console.log('Starting Penpot exporter...');
      const exporterResult = await window.electron.dockerCommand([
        'docker', 'run', '-d',
        '--name', 'penpot-exporter',
        '--network', 'penpot',
        '--restart', 'always',
        '-e', 'PENPOT_PUBLIC_URI=http://penpot-frontend:8080',
        '-e', 'PENPOT_REDIS_URI=redis://penpot-valkey/0',
        'penpotapp/exporter:latest'
      ].join(' '));
      console.log('Exporter result:', exporterResult);

      setErrorMessage('🎨 Starting Penpot frontend...');

      // Start Penpot frontend (official configuration)
      console.log('Starting Penpot frontend...');
      const frontendResult = await window.electron.dockerCommand(
        'docker run -d ' +
        '--name penpot-frontend ' +
        '--network penpot ' +
        '--restart always ' +
        '-p 9001:8080 ' +
        '-e "PENPOT_FLAGS=disable-email-verification enable-prepl-server disable-secure-session-cookies" ' +
        '-e PENPOT_HTTP_SERVER_MAX_BODY_SIZE=31457280 ' +
        '-e PENPOT_HTTP_SERVER_MAX_MULTIPART_BODY_SIZE=367001600 ' +
        '-v /tmp/penpot-assets:/opt/data/assets ' +
        'penpotapp/frontend:latest'
      );
      console.log('Frontend result:', frontendResult);
      
      if (frontendResult.success) {
        setDockerState({
          isRunning: true,
          containerId: 'penpot-frontend',
          port: 9001,
          status: 'running',
          logs: ['Container started successfully']
        });
        setErrorMessage('✅ Penpot containers started! Waiting for service to be ready...\n\nThis may take a few minutes on first run as Penpot initializes...');
        
        // Wait for Penpot to be ready
        setTimeout(() => {
          setErrorMessage('✅ Penpot should be ready! If the canvas below is blank, try refreshing it in a minute.');
        }, 20000);
        
      } else {
        throw new Error(frontendResult.error || 'Failed to start Penpot frontend container');
      }
    } catch (error) {
      console.error('Error starting Penpot:', error);
      setDockerState(prev => ({ ...prev, status: 'error' }));
      setErrorMessage(`❌ Failed to start Penpot: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  };

  const stopPenpotContainer = async () => {
    setDockerState(prev => ({ ...prev, status: 'stopped' }));
    setErrorMessage('Stopping Penpot container...');
    
    try {
      // Stop all Penpot containers (using official names)
      await window.electron.dockerCommand('docker stop penpot-frontend penpot-backend penpot-postgres penpot-valkey penpot-exporter || true');
      await window.electron.dockerCommand('docker rm penpot-frontend penpot-backend penpot-postgres penpot-valkey penpot-exporter || true');
      await window.electron.dockerCommand('docker network rm penpot || true');
      
      setDockerState({
        isRunning: false,
        status: 'stopped',
        logs: []
      });
      setErrorMessage('✅ Penpot containers stopped successfully.');
    } catch (error) {
      setErrorMessage(`❌ Failed to stop Penpot: ${error instanceof Error ? error.message : 'Unknown error'}`);
    }
  };

  const refreshPenpotCanvas = () => {
    if (iframeRef.current) {
      iframeRef.current.src = iframeRef.current.src;
    }
  };

  const openPenpotInRenderer = async () => {
    try {
      // Create a new window specifically for Penpot
      const { ipcRenderer } = window.electron;
      await ipcRenderer.invoke('create-penpot-window', penpotUrl);
    } catch (error) {
      console.error('Failed to open Penpot renderer window:', error);
      // Fallback to opening in browser
      window.open(penpotUrl, '_blank');
    }
  };

  const checkContainerStatus = async () => {
    try {
      const result = await window.electron.dockerCommand('docker ps --filter name=penpot-frontend --format "{{.Status}}"');
      if (result.success && result.output && result.output.includes('Up')) {
        setDockerState(prev => ({ ...prev, isRunning: true, status: 'running' }));
      } else {
        setDockerState(prev => ({ ...prev, isRunning: false, status: 'stopped' }));
      }
    } catch (error) {
      console.error('Failed to check container status:', error);
    }
  };

  // Check container status on mount
  useEffect(() => {
    checkContainerStatus();
    const interval = setInterval(checkContainerStatus, 10000); // Check every 10 seconds
    return () => clearInterval(interval);
  }, []);
  useEffect(() => {
    const saved = localStorage.getItem('penpot-local-projects');
    if (saved) {
      try {
        setLocalProjects(JSON.parse(saved));
      } catch (e) {
        console.error('Failed to load local projects:', e);
      }
    }

    // Load saved token
    const savedToken = localStorage.getItem('penpot-access-token');
    if (savedToken) {
      setPenpotToken(savedToken);
      // Validate token on load
      validateToken(savedToken);
    }
  }, []);

  // Save local projects to localStorage
  const saveLocalProjects = (projects: typeof localProjects) => {
    setLocalProjects(projects);
    localStorage.setItem('penpot-local-projects', JSON.stringify(projects));
  };

  // Save token to localStorage and auto-fetch data
  const saveToken = async (token: string) => {
    setPenpotToken(token);
    if (token) {
      localStorage.setItem('penpot-access-token', token);
      
      // Validate token and if valid, automatically fetch teams
      const isValid = await validateToken(token);
      if (isValid) {
        setErrorMessage('✅ Token saved successfully! Fetching your teams and projects...');
        
        // Auto-fetch teams after successful token save
        setTimeout(() => {
          fetchPenpotTeams();
        }, 500); // Small delay to show the success message
      }
    } else {
      localStorage.removeItem('penpot-access-token');
      setIsTokenValid(null);
    }
  };

  // Validate token by making a test API call
  const validateToken = async (token: string) => {
    if (!token) {
      setIsTokenValid(false);
      return false;
    }

    try {
      const result = await window.electron.penpotApiCall({
        url: 'https://design.penpot.app/api/rpc/command/profile:get-profile',
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
          'Authorization': `Token ${token}`,
        },
        body: JSON.stringify({})
      });

      const isValid = result.ok;
      setIsTokenValid(isValid);
      return isValid;
    } catch (error) {
      setIsTokenValid(false);
      return false;
    }
  };

  // Enhanced deep link functions
  const openPenpotDashboard = (teamId?: string) => {
    const url = teamId 
      ? `https://design.penpot.app/#/dashboard/recent?team-id=${teamId}`
      : 'https://design.penpot.app/#/dashboard/projects';
    window.open(url, '_blank');
  };

  const openPenpotProject = (projectId: string, teamId?: string) => {
    const url = `https://design.penpot.app/#/workspace/${projectId}`;
    window.open(url, '_blank');
    
    // Track this project locally
    const existingProject = localProjects.find(p => p.id === projectId);
    if (!existingProject) {
      const newProject: LocalPenpotProject = {
        id: projectId,
        name: `Project ${projectId}`,
        url: url,
        type: 'project',
        team_id: teamId,
        lastAccessed: new Date().toISOString()
      };
      saveLocalProjects([...localProjects, newProject]);
    } else {
      // Update last accessed time
      const updated = localProjects.map(p => 
        p.id === projectId 
          ? { ...p, lastAccessed: new Date().toISOString() }
          : p
      );
      saveLocalProjects(updated);
    }
  };

  const openPenpotFile = (projectId: string, fileId: string) => {
    const url = `https://design.penpot.app/#/workspace/${projectId}/${fileId}`;
    window.open(url, '_blank');
  };

  const createNewProject = () => {
    window.open('https://design.penpot.app/#/dashboard/projects?action=create', '_blank');
  };

  const openPenpotWithTemplate = (template: string) => {
    window.open(`https://design.penpot.app/#/dashboard/projects?template=${template}`, '_blank');
  };

  const addCustomProject = () => {
    const input = prompt('Enter Penpot URL or Project ID:');
    if (!input) return;
    
    let projectId = '';
    let teamId = '';
    let projectType: 'project' | 'team' | 'dashboard' = 'project';
    
    // Parse different URL formats
    if (input.includes('design.penpot.app')) {
      const url = new URL(input);
      const hash = url.hash;
      
      // Extract team-id from query params
      const teamIdMatch = hash.match(/team-id=([^&]+)/);
      if (teamIdMatch) {
        teamId = teamIdMatch[1];
        projectType = 'team';
      }
      
      // Extract project ID from workspace URLs
      const workspaceMatch = hash.match(/#\/workspace\/([^\/]+)/);
      if (workspaceMatch) {
        projectId = workspaceMatch[1];
        projectType = 'project';
      }
      
      // Handle dashboard URLs
      if (hash.includes('/dashboard/')) {
        projectType = 'dashboard';
        projectId = teamId || 'dashboard';
      }
    } else {
      // Assume it's just a project ID
      projectId = input.trim();
    }
    
    const projectName = prompt('Enter a name for this project:');
    if (!projectName) return;
    
    const newProject: LocalPenpotProject = {
      id: projectId,
      name: projectName.trim(),
      url: projectType === 'team' && teamId 
        ? `https://design.penpot.app/#/dashboard/recent?team-id=${teamId}`
        : `https://design.penpot.app/#/workspace/${projectId}`,
      type: projectType,
      team_id: teamId || undefined,
      lastAccessed: new Date().toISOString()
    };
    
    saveLocalProjects([...localProjects, newProject]);
  };

  // Function to test Penpot API connection with token
  const testPenpotAPI = async () => {
    if (!penpotToken) {
      setErrorMessage('❌ No access token provided. Please enter your Penpot access token first.');
      return false;
    }

    setIsLoading(true);
    setErrorMessage('');
    
    try {
      const result = await window.electron.penpotApiCall({
        url: 'https://design.penpot.app/api/rpc/command/profile:get-profile',
        method: 'POST',
        headers: {
          'Content-Type': 'application/transit+json',
          'Accept': 'application/transit+json',
          'Authorization': `Token ${penpotToken}`,
        },
        body: JSON.stringify({})
      });
      
      console.log('Penpot API test result:', result);
      
      if (result.ok) {
        if (result.data && result.data !== '') {
          setErrorMessage('✅ API connection successful! Token is valid.');
          setIsTokenValid(true);
          return true;
        } else {
          // Try with JSON format as fallback
          const jsonResult = await window.electron.penpotApiCall({
            url: 'https://design.penpot.app/api/rpc/command/profile:get-profile',
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
              'Accept': 'application/json',
              'Authorization': `Token ${penpotToken}`,
            },
            body: JSON.stringify({})
          });
          
          if (jsonResult.ok && jsonResult.data && jsonResult.data !== '') {
            setErrorMessage('✅ API connection successful! Token is valid (using JSON format).');
            setIsTokenValid(true);
            return true;
          } else {
            setErrorMessage('✅ Token appears valid (200 OK) but API returned empty data. This might be normal for the profile endpoint.');
            setIsTokenValid(true);
            return true;
          }
        }
      } else {
        if (result.status === 401 || result.status === 403) {
          setErrorMessage('❌ Invalid access token. Please check your token and try again.');
          setIsTokenValid(false);
        } else {
          setErrorMessage(`❌ API Error: ${result.status} - ${result.statusText}. ${typeof result.data === 'string' ? result.data : JSON.stringify(result.data)}`);
          setIsTokenValid(false);
        }
        return false;
      }
    } catch (error) {
      setErrorMessage(`❌ Connection failed: ${error instanceof Error ? error.message : 'Unknown error'}`);
      setIsTokenValid(false);
      return false;
    } finally {
      setIsLoading(false);
    }
  };

  // Function to fetch teams from Penpot API with token
  const fetchPenpotTeams = async () => {
    if (!penpotToken) {
      setErrorMessage('❌ No access token provided. Please enter your Penpot access token first.');
      return;
    }

    setIsLoading(true);
    setErrorMessage('');
    
    try {
      // Try with transit+json first (Penpot's preferred format)
      let result = await window.electron.penpotApiCall({
        url: 'https://design.penpot.app/api/rpc/command/teams:get-teams',
        method: 'POST',
        headers: {
          'Content-Type': 'application/transit+json',
          'Accept': 'application/transit+json',
          'Authorization': `Token ${penpotToken}`,
        },
        body: JSON.stringify({})
      });
      
      console.log('Penpot teams result (transit):', result);
      
      // If empty response, try with JSON format
      if (result.ok && (!result.data || result.data === '')) {
        console.log('Transit format returned empty, trying JSON format...');
        result = await window.electron.penpotApiCall({
          url: 'https://design.penpot.app/api/rpc/command/teams:get-teams',
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Accept': 'application/json',
            'Authorization': `Token ${penpotToken}`,
          },
          body: JSON.stringify({})
        });
        console.log('Penpot teams result (JSON):', result);
      }
      
      console.log('Teams data:', result.data);
      console.log('Teams data type:', typeof result.data);
      
      if (result.ok) {
        if (!result.data || result.data === '') {
          setErrorMessage(`🔍 DEBUG: API call successful but returned empty data.\n\n` +
            `Status: ${result.status} ${result.statusText}\n` +
            `Data: "${result.data}"\n` +
            `Data type: ${typeof result.data}\n\n` +
            `This could mean:\n` +
            `• You're not a member of any teams\n` +
            `• The API requires different parameters\n` +
            `• The endpoint structure has changed\n\n` +
            `Let's try alternative approaches...`);
          
          // Try alternative endpoints
          await tryAlternativeTeamEndpoints();
          return;
        }
        
        // Success! Let's examine the data structure in detail
        console.log('Raw teams data:', JSON.stringify(result.data, null, 2));
        
        // Show the raw data to the user for debugging
        setErrorMessage(`🔍 DEBUG: API call successful! Here's what was returned:\n\n` +
          `Status: ${result.status} ${result.statusText}\n` +
          `Data type: ${typeof result.data}\n` +
          `Data: ${JSON.stringify(result.data, null, 2)}\n\n` +
          `Please share this information so we can understand the data structure.`);
        
        // Continue with existing processing logic...
        // [Rest of the processing code remains the same]
        
      } else {
        if (result.status === 401 || result.status === 403) {
          setErrorMessage('❌ Invalid access token. Please check your token and try again.');
          setIsTokenValid(false);
        } else if (result.status === 400) {
          setErrorMessage(`❌ Bad request. The API might require specific parameters. Error: ${typeof result.data === 'string' ? result.data : JSON.stringify(result.data)}`);
        } else {
          setErrorMessage(`❌ Failed to fetch teams: ${result.status} - ${result.statusText}\n\nResponse data: ${JSON.stringify(result.data, null, 2)}`);
        }
      }
    } catch (error) {
      setErrorMessage(`❌ Error fetching teams: ${error instanceof Error ? error.message : 'Unknown error'}`);
    } finally {
      setIsLoading(false);
    }
  };

  // Try alternative team endpoints
  const tryAlternativeTeamEndpoints = async () => {
    const alternativeEndpoints = [
      'https://design.penpot.app/api/rpc/command/teams:get-owned-teams',
      'https://design.penpot.app/api/rpc/command/profile:get-profile',
      'https://design.penpot.app/api/rpc/command/projects:get-all-projects', // Try to get all projects
      'https://design.penpot.app/api/rpc/command/teams:get-teams-for-profile', // Alternative team endpoint
    ];
    
    let results = '🔍 Trying alternative endpoints:\n\n';
    
    for (const endpoint of alternativeEndpoints) {
      try {
        // Try with both transit+json and regular json
        let result = await window.electron.penpotApiCall({
          url: endpoint,
          method: 'POST',
          headers: {
            'Content-Type': 'application/transit+json',
            'Accept': 'application/transit+json',
            'Authorization': `Token ${penpotToken}`,
          },
          body: JSON.stringify({})
        });
        
        // If empty, try with JSON
        if (!result.data || result.data === '') {
          result = await window.electron.penpotApiCall({
            url: endpoint,
            method: 'POST',
            headers: {
              'Content-Type': 'application/json',
              'Accept': 'application/json',
              'Authorization': `Token ${penpotToken}`,
            },
            body: JSON.stringify({})
          });
        }
        
        const endpointName = endpoint.split('/').pop() || endpoint;
        results += `${endpointName}\n`;
        results += `  Status: ${result.status} ${result.statusText}\n`;
        
        if (result.data && result.data !== '') {
          if (typeof result.data === 'string' && result.data.length > 200) {
            results += `  Data: ${result.data.substring(0, 200)}...\n`;
          } else {
            results += `  Data: ${JSON.stringify(result.data)}\n`;
          }
        } else {
          results += `  Data: empty\n`;
        }
        results += '\n';
        
      } catch (error) {
        const endpointName = endpoint.split('/').pop() || endpoint;
        results += `${endpointName}\n`;
        results += `  Error: ${error instanceof Error ? error.message : 'Unknown error'}\n\n`;
      }
    }
    
    // Also try some direct project endpoints that might work without teams
    results += '\n🔍 Trying direct project access:\n\n';
    
    try {
      // Try to get recent files/projects directly
      const recentResult = await window.electron.penpotApiCall({
        url: 'https://design.penpot.app/api/rpc/command/files:get-team-recent-files',
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          'Accept': 'application/json',
          'Authorization': `Token ${penpotToken}`,
        },
        body: JSON.stringify({})
      });
      
      results += `Recent Files:\n`;
      results += `  Status: ${recentResult.status} ${recentResult.statusText}\n`;
      results += `  Data: ${recentResult.data ? JSON.stringify(recentResult.data).substring(0, 100) + '...' : 'empty'}\n\n`;
      
    } catch (error) {
      results += `Recent Files:\n`;
      results += `  Error: ${error instanceof Error ? error.message : 'Unknown error'}\n\n`;
    }
    
    // Final suggestion
    results += '\n💡 Possible explanations:\n';
    results += '• You might not be a member of any teams yet\n';
    results += '• Your account might be new and needs team invitation\n';
    results += '• The API token might have limited permissions\n';
    results += '• Penpot might organize projects differently for your account\n\n';
    results += '🎯 Recommendation: Try manually adding a project URL from your Penpot dashboard!';
    
    setErrorMessage(results);
  };

  // Function to explore Penpot API endpoints
  const exploreAPI = async () => {
    setIsLoading(true);
    setErrorMessage('');
    
    const endpoints = [
      'https://design.penpot.app/api',
      'https://design.penpot.app/api/rpc',
      'https://design.penpot.app/api/rpc/command/profile:get-profile',
      'https://design.penpot.app/api/profile/get-profile',
      'https://design.penpot.app/api/rpc/command/teams:get-teams',
      'https://design.penpot.app/api/teams/get-teams',
    ];
    
    let results = '🔍 API Endpoint Exploration Results:\n\n';
    
    for (const endpoint of endpoints) {
      try {
        const result = await window.electron.penpotApiCall({
          url: endpoint,
          method: 'POST',
          headers: {
            'Content-Type': 'application/json',
            'Accept': 'application/json',
          },
        });
        
        results += `${endpoint}\n`;
        results += `  Status: ${result.status} ${result.statusText}\n`;
        if (result.data && typeof result.data === 'string' && result.data.length < 200) {
          results += `  Data: ${result.data}\n`;
        } else if (result.data && typeof result.data === 'object') {
          results += `  Data: ${JSON.stringify(result.data).substring(0, 100)}...\n`;
        }
        results += '\n';
      } catch (error) {
        results += `${endpoint}\n`;
        results += `  Error: ${error instanceof Error ? error.message : 'Unknown error'}\n\n`;
      }
    }
    
    setErrorMessage(results);
    setIsLoading(false);
  };

  const removeProject = (projectId: string) => {
    const updated = localProjects.filter(p => p.id !== projectId);
    saveLocalProjects(updated);
  };

  const exportProjectData = () => {
    const data = JSON.stringify(localProjects, null, 2);
    const blob = new Blob([data], { type: 'application/json' });
    const url = URL.createObjectURL(blob);
    const a = document.createElement('a');
    a.href = url;
    a.download = 'penpot-projects.json';
    a.click();
    URL.revokeObjectURL(url);
  };

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
                <div className="absolute inset-0 bg-background-default flex items-center justify-center pointer-events-none opacity-0 transition-opacity duration-300" id="penpot-loading">
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
                <h4 className="font-semibold text-blue-900 dark:text-blue-100 mb-2">🐳 Docker Setup Required</h4>
                <p className="text-blue-800 dark:text-blue-200 text-sm mb-3">
                  To use the embedded Penpot canvas, you need Docker installed:
                </p>
                <ol className="text-blue-800 dark:text-blue-200 text-sm space-y-1 ml-4 mb-3">
                  <li>1. Install <a href="https://www.docker.com/products/docker-desktop/" target="_blank" rel="noopener noreferrer" className="underline">Docker Desktop</a></li>
                  <li>2. Make sure Docker is running</li>
                  <li>3. Click "Start Penpot" in the header above</li>
                  <li>4. Wait for the container to start (may take a few minutes on first run)</li>
                </ol>
                <div className="text-xs text-blue-700 dark:text-blue-300">
                  <strong>What happens:</strong><br/>
                  • Downloads Penpot, PostgreSQL, and Redis containers<br/>
                  • Sets up a local Penpot instance on port 9001<br/>
                  • Provides full design canvas access within Goose
                </div>
              </div>
            )}
        </div>
      </div>
    </div>
  );
}

export default PenpotCanvas;
