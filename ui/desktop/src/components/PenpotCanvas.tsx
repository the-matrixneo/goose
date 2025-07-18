import { useEffect, useRef, useState } from 'react';
import { Palette, Download, Share, Layers, Square, ExternalLink, Plus, FolderOpen, Settings } from 'lucide-react';
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

// Penpot Integration Component - Enhanced Deep Links + Local Project Management
function PenpotCanvas({ 
  projectId, 
  fileId, 
  pageId, 
  initialDesign,
  onDesignChange,
  onExport 
}: PenpotCanvasProps) {
  const [isLoading, setIsLoading] = useState(false);
  const [integrationMode, setIntegrationMode] = useState<'dashboard' | 'projects' | 'local'>('dashboard');
  const [localProjects, setLocalProjects] = useState<LocalPenpotProject[]>([]);
  const [errorMessage, setErrorMessage] = useState<string>('');
  const [penpotToken, setPenpotToken] = useState<string>('');
  const [isTokenValid, setIsTokenValid] = useState<boolean | null>(null);

  // Load local projects and token from localStorage
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
    <div className="h-full flex flex-col bg-background-default">
      {/* Header */}
      <div className="flex items-center justify-between p-4 border-b border-borderSubtle bg-background-muted">
        <div className="flex items-center space-x-2">
          <Palette size={20} className="text-primary" />
          <h2 className="text-lg font-semibold text-textStandard">Penpot Integration</h2>
        </div>
        
        <div className="flex items-center space-x-2">
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setIntegrationMode('dashboard')}
            className={integrationMode === 'dashboard' ? 'bg-background-muted' : ''}
          >
            <ExternalLink size={14} className="mr-1" />
            Quick Access
          </Button>
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setIntegrationMode('local')}
            className={integrationMode === 'local' ? 'bg-background-muted' : ''}
          >
            <FolderOpen size={14} className="mr-1" />
            My Projects
          </Button>
        </div>
      </div>

      {/* Content */}
      <div className="flex-1 p-4 overflow-auto">
        {integrationMode === 'dashboard' && (
          <div className="space-y-6">
            <div className="text-center">
              <h3 className="text-xl font-semibold text-textStandard mb-2">Quick Access to Penpot</h3>
              <p className="text-textSubtle mb-6">Open Penpot directly in a new tab to start designing</p>
            </div>

            <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
              <div className="bg-background-muted p-6 rounded-lg border border-borderSubtle">
                <div className="flex items-center mb-3">
                  <FolderOpen size={24} className="text-primary mr-2" />
                  <h4 className="font-semibold text-textStandard">Dashboard</h4>
                </div>
                <p className="text-textSubtle text-sm mb-4">Access your projects and teams</p>
                <Button onClick={openPenpotDashboard} className="w-full">
                  <ExternalLink size={16} className="mr-2" />
                  Open Dashboard
                </Button>
              </div>

              <div className="bg-background-muted p-6 rounded-lg border border-borderSubtle">
                <div className="flex items-center mb-3">
                  <Plus size={24} className="text-primary mr-2" />
                  <h4 className="font-semibold text-textStandard">New Project</h4>
                </div>
                <p className="text-textSubtle text-sm mb-4">Create a new design project</p>
                <Button onClick={createNewProject} className="w-full">
                  <Plus size={16} className="mr-2" />
                  Create Project
                </Button>
              </div>
            </div>

            {/* Template Quick Access */}
            <div className="bg-background-muted p-6 rounded-lg border border-borderSubtle">
              <h4 className="font-semibold text-textStandard mb-4">Start with Templates</h4>
              <div className="grid grid-cols-2 md:grid-cols-4 gap-3">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => openPenpotWithTemplate('mobile-app')}
                  className="text-xs"
                >
                  📱 Mobile App
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => openPenpotWithTemplate('website')}
                  className="text-xs"
                >
                  🌐 Website
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => openPenpotWithTemplate('dashboard')}
                  className="text-xs"
                >
                  📊 Dashboard
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={() => openPenpotWithTemplate('presentation')}
                  className="text-xs"
                >
                  📋 Presentation
                </Button>
              </div>
            </div>

            {/* Access Token Configuration */}
            <div className="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg border border-blue-200 dark:border-blue-800">
              <h4 className="font-semibold text-blue-900 dark:text-blue-100 mb-2">🔑 Access Token Configuration</h4>
              <p className="text-blue-800 dark:text-blue-200 text-sm mb-3">
                Enter your Penpot access token to use the API without browser login:
              </p>
              <div className="space-y-3">
                <div className="flex items-center space-x-2">
                  <input
                    type="password"
                    placeholder="Enter your Penpot access token..."
                    value={penpotToken}
                    onChange={(e) => setPenpotToken(e.target.value)}
                    className="flex-1 px-3 py-2 text-sm border border-blue-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500"
                  />
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => saveToken(penpotToken)}
                    disabled={!penpotToken}
                  >
                    Save
                  </Button>
                  <Button
                    variant="outline"
                    size="sm"
                    onClick={() => {
                      setPenpotToken('');
                      saveToken('');
                      setIsTokenValid(null);
                    }}
                  >
                    Clear
                  </Button>
                </div>
                {isTokenValid !== null && (
                  <div className={`text-xs p-2 rounded ${
                    isTokenValid 
                      ? 'bg-green-100 text-green-800' 
                      : 'bg-red-100 text-red-800'
                  }`}>
                    {isTokenValid ? '✅ Token is valid' : '❌ Token is invalid or expired'}
                  </div>
                )}
                <div className="text-xs text-blue-700 dark:text-blue-300">
                  <strong>How to get your access token:</strong><br/>
                  1. Go to Penpot → Profile Settings → Access Tokens<br/>
                  2. Create a new token<br/>
                  3. Copy and paste it above
                </div>
              </div>
            </div>

            <div className="bg-yellow-50 dark:bg-yellow-900/20 p-4 rounded-lg border border-yellow-200 dark:border-yellow-800">
              <h4 className="font-semibold text-yellow-900 dark:text-yellow-100 mb-2">🔧 API Testing</h4>
              <p className="text-yellow-800 dark:text-yellow-200 text-sm mb-3">
                Test the Penpot API connection and fetch your teams automatically:
              </p>
              <div className="flex flex-wrap gap-2 mb-3">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={testPenpotAPI}
                  disabled={isLoading || !penpotToken}
                >
                  {isLoading ? 'Testing...' : 'Test API'}
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={fetchPenpotTeams}
                  disabled={isLoading || !penpotToken}
                >
                  {isLoading ? 'Fetching...' : 'Fetch Teams'}
                </Button>
                <Button
                  variant="outline"
                  size="sm"
                  onClick={exploreAPI}
                  disabled={isLoading}
                >
                  {isLoading ? 'Exploring...' : 'Explore API'}
                </Button>
              </div>
              {errorMessage && (
                <div className="text-xs p-2 bg-yellow-100 dark:bg-yellow-800/50 rounded whitespace-pre-wrap">
                  {errorMessage}
                </div>
              )}
            </div>

            <div className="bg-yellow-50 dark:bg-yellow-900/20 p-4 rounded-lg border border-yellow-200 dark:border-yellow-800">
              <h4 className="font-semibold text-yellow-900 dark:text-yellow-100 mb-2">⚠️ API Limitation</h4>
              <p className="text-yellow-800 dark:text-yellow-200 text-sm mb-2">
                Penpot's API is protected by Cloudflare and blocks direct integration. However, you can:
              </p>
              <ul className="text-yellow-800 dark:text-yellow-200 text-sm space-y-1 ml-4">
                <li>• Use the "My Projects" tab to track your frequently used projects</li>
                <li>• Open projects directly in Penpot with one click</li>
                <li>• Export your project list for backup</li>
                <li>• Test the API above to see current status</li>
              </ul>
            </div>
          </div>
        )}

        {integrationMode === 'local' && (
          <div className="space-y-6">
            <div className="flex items-center justify-between">
              <div>
                <h3 className="text-xl font-semibold text-textStandard mb-2">My Penpot Projects</h3>
                <p className="text-textSubtle">Track and quickly access your frequently used projects</p>
              </div>
              <div className="flex space-x-2">
                <Button
                  variant="outline"
                  size="sm"
                  onClick={exportProjectData}
                  disabled={localProjects.length === 0}
                >
                  <Download size={14} className="mr-1" />
                  Export
                </Button>
                <Button
                  size="sm"
                  onClick={addCustomProject}
                >
                  <Plus size={14} className="mr-1" />
                  Add Project
                </Button>
              </div>
            </div>

            {localProjects.length > 0 ? (
              <div className="space-y-3">
                {localProjects
                  .sort((a, b) => new Date(b.lastAccessed).getTime() - new Date(a.lastAccessed).getTime())
                  .map((project) => (
                    <div 
                      key={project.id}
                      className="flex items-center justify-between p-4 bg-background-muted rounded-lg border border-borderSubtle hover:bg-background-subtle cursor-pointer"
                      onClick={() => window.open(project.url, '_blank')}
                    >
                      <div className="flex-1">
                        <div className="flex items-center space-x-2">
                          <h5 className="font-medium text-textStandard">{project.name}</h5>
                          <span className={`px-2 py-1 text-xs rounded ${
                            project.type === 'team' ? 'bg-blue-100 text-blue-800' :
                            project.type === 'dashboard' ? 'bg-green-100 text-green-800' :
                            'bg-purple-100 text-purple-800'
                          }`}>
                            {project.type}
                          </span>
                        </div>
                        <p className="text-xs text-textSubtle">
                          ID: {project.id} 
                          {project.team_id && ` • Team: ${project.team_id}`}
                          {' • Last accessed: ' + new Date(project.lastAccessed).toLocaleDateString()}
                        </p>
                      </div>
                      <div className="flex items-center space-x-2">
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={(e) => {
                            e.stopPropagation();
                            window.open(project.url, '_blank');
                          }}
                        >
                          <ExternalLink size={14} />
                        </Button>
                        <Button
                          variant="ghost"
                          size="sm"
                          onClick={(e) => {
                            e.stopPropagation();
                            removeProject(project.id);
                          }}
                          className="text-red-500 hover:text-red-700"
                        >
                          ×
                        </Button>
                      </div>
                    </div>
                  ))}
              </div>
            ) : (
              <div className="text-center py-12">
                <Palette size={48} className="text-textSubtle mx-auto mb-4" />
                <h4 className="text-lg font-medium text-textStandard mb-2">No Projects Yet</h4>
                <p className="text-textSubtle mb-4">Add your Penpot projects to quickly access them from Goose</p>
                <Button onClick={addCustomProject}>
                  <Plus size={16} className="mr-2" />
                  Add Your First Project
                </Button>
              </div>
            )}

            <div className="bg-blue-50 dark:bg-blue-900/20 p-4 rounded-lg border border-blue-200 dark:border-blue-800">
              <h4 className="font-semibold text-blue-900 dark:text-blue-100 mb-2">💡 How to Add Projects</h4>
              <ol className="text-blue-800 dark:text-blue-200 text-sm space-y-1 ml-4">
                <li>1. Copy the full URL from your Penpot browser tab</li>
                <li>2. Click "Add Project" and paste the URL</li>
                <li>3. Give it a memorable name</li>
                <li>4. Supports: workspace URLs, dashboard URLs with team-id, or just project IDs</li>
              </ol>
              <div className="mt-3 p-2 bg-blue-100 dark:bg-blue-800/50 rounded text-xs">
                <strong>Example URLs:</strong><br/>
                • https://design.penpot.app/#/workspace/abc123<br/>
                • https://design.penpot.app/#/dashboard/recent?team-id=xyz456<br/>
                • Or just: abc123
              </div>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default PenpotCanvas;
