import React, { useState } from 'react';
import { Hammer, Plus, Clock, FolderOpen } from 'lucide-react';
import { MainPanelLayout } from '../Layout/MainPanelLayout';
import { Button } from '../ui/button';
import { Card } from '../ui/card';
import { ScrollArea } from '../ui/scroll-area';
import { formatMessageTimestamp } from '../../utils/timeUtils';

// Mock data for demonstration - replace with real data
interface AppTile {
  id: string;
  app_name: string;
  last_edited: number;
  path: string;
}

const BuildView: React.FC = () => {
  // Placeholder apps data - you can replace this with real data
  const [apps] = useState<AppTile[]>([
    {
      id: '1',
      app_name: 'Customer Dashboard',
      last_edited: Math.floor((Date.now() - 1000 * 60 * 30) / 1000), // 30 mins ago
      path: '/apps/customer-dashboard',
    },
    {
      id: '2',
      app_name: 'API Documentation Generator',
      last_edited: Math.floor((Date.now() - 1000 * 60 * 60 * 2) / 1000), // 2 hours ago
      path: '/apps/api-docs-gen',
    },
    {
      id: '3',
      app_name: 'Data Pipeline Builder',
      last_edited: Math.floor((Date.now() - 1000 * 60 * 60 * 24) / 1000), // 1 day ago
      path: '/apps/data-pipeline',
    },
  ]);

  const handleCreateApp = () => {
    // TODO: Implement create app functionality
    console.log('Create App clicked - implement your logic here');
  };

  const handleAppClick = (app: AppTile) => {
    // TODO: Implement app click functionality
    console.log('App clicked:', app);
  };

  // Component for rendering individual app tiles (similar to SessionItem)
  const AppTile: React.FC<{ app: AppTile }> = ({ app }) => {
    return (
      <Card
        className="h-full py-3 px-4 flex flex-col justify-between cursor-pointer hover:bg-background-medium/50 transition-all duration-200"
        onClick={() => handleAppClick(app)}
      >
        <div className="flex-1">
          {/* App name - matching session title exactly */}
          <h3 className="text-base truncate mb-1">{app.app_name}</h3>

          {/* Timestamp - similar to session */}
          <div className="flex items-center text-text-muted text-xs mb-1">
            <Clock className="w-3 h-3 mr-1 flex-shrink-0" />
            <span>{formatMessageTimestamp(app.last_edited)}</span>
          </div>

          {/* Path */}
          <div className="flex items-center text-text-muted text-xs mb-1">
            <FolderOpen className="w-3 h-3 mr-1 flex-shrink-0" />
            <span className="truncate">{app.path}</span>
          </div>
        </div>
      </Card>
    );
  };

  return (
    <MainPanelLayout>
      <div className="flex-1 flex flex-col min-h-0">
        <div className="bg-background-default px-8 pb-8 pt-16">
          <div className="flex flex-col page-transition">
            <div className="flex justify-between items-center mb-1">
              <h1 className="text-4xl font-light">Build</h1>
              {/* Create App button on the right */}
              <Button
                onClick={handleCreateApp}
                variant="default"
                className="flex items-center gap-2"
              >
                <Plus className="w-4 h-4" />
                Create App
              </Button>
            </div>
          </div>
        </div>

        {/* Apps grid - similar to SessionListView */}
        <div className="flex-1 min-h-0 relative px-8">
          <ScrollArea className="h-full">
            <div className="space-y-8 pb-8">
              {/* Recent Apps section */}
              <div className="space-y-4">
                <div className="sticky top-0 z-10 bg-background-default/95 backdrop-blur-sm">
                  <h2 className="text-text-muted">Your Apps</h2>
                </div>

                {apps.length > 0 ? (
                  <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4">
                    {apps.map((app) => (
                      <AppTile key={app.id} app={app} />
                    ))}
                  </div>
                ) : (
                  <div className="flex flex-col items-center justify-center py-12 text-text-muted">
                    <Hammer className="h-12 w-12 mb-4" />
                    <p className="text-lg mb-2">No apps yet</p>
                    <p className="text-sm mb-4">Create your first app to get started</p>
                    <Button
                      onClick={handleCreateApp}
                      variant="default"
                      className="flex items-center gap-2"
                    >
                      <Plus className="w-4 h-4" />
                      Create Your First App
                    </Button>
                  </div>
                )}
              </div>
            </div>
          </ScrollArea>
        </div>
      </div>
    </MainPanelLayout>
  );
};

export default BuildView;
