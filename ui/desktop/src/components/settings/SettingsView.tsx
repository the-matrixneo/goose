import { ScrollArea } from '../ui/scroll-area';
import { useSidebar } from '../ui/sidebar';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../ui/tabs';
import type { View, ViewOptions } from '../../App';
import ModelsSection from './models/ModelsSection';
import SessionSharingSection from './sessions/SessionSharingSection';
import AppSettingsSection from './app/AppSettingsSection';
import { ExtensionConfig } from '../../api';
import { MainPanelLayout } from '../Layout/MainPanelLayout';
import {
  Bot,
  Settings,
  Share2,
  Palette,
  Wrench,
  Monitor,
  Target,
  MessageSquare,
} from 'lucide-react';
import { useState, useEffect } from 'react';
import ChatSettingsSection from './chat/ChatSettingsSection';

export type SettingsViewOptions = {
  deepLinkConfig?: ExtensionConfig;
  showEnvVars?: boolean;
  section?: string;
};

export default function SettingsView({
  onClose,
  setView,
  viewOptions,
}: {
  onClose: () => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  viewOptions: SettingsViewOptions;
}) {
  const [activeTab, setActiveTab] = useState('models');

  // Determine initial tab based on section prop
  useEffect(() => {
    if (viewOptions.section) {
      // Map section names to tab values
      const sectionToTab: Record<string, string> = {
        update: 'app',
        models: 'models',
        modes: 'chat',
        sharing: 'sharing',
        styles: 'chat',
        tools: 'chat',
        app: 'app',
        chat: 'chat',
      };

      const targetTab = sectionToTab[viewOptions.section];
      if (targetTab) {
        setActiveTab(targetTab);
      }
    }
  }, [viewOptions.section]);

  return (
    <>
      <MainPanelLayout>
        <div className="flex-1 flex flex-col min-h-0 mt-6">
          {/* Content Area */}
          <div className="flex flex-col mt-7 mb-6 px-2">
            <h1 className="text-4xl font-light">Settings</h1>
          </div>

          <div className="flex-1 min-h-0 relative">
            <Tabs value={activeTab} onValueChange={setActiveTab} className="h-full flex flex-col">
              <div className="px-1">
                <TabsList className="w-full mb-2 justify-start">
                  <TabsTrigger value="models" className="flex gap-2">
                    <Bot className="h-4 w-4" />
                    Models
                  </TabsTrigger>
                  <TabsTrigger value="chat" className="flex gap-2">
                    <MessageSquare className="h-4 w-4" />
                    Chat
                  </TabsTrigger>
                  <TabsTrigger value="sharing" className="flex gap-2">
                    <Share2 className="h-4 w-4" />
                    Session
                  </TabsTrigger>
                  <TabsTrigger value="app" className="flex gap-2">
                    <Monitor className="h-4 w-4" />
                    App
                  </TabsTrigger>
                </TabsList>
              </div>

              <ScrollArea className="flex-1 px-2">
                <TabsContent
                  value="models"
                  className="mt-0 focus-visible:outline-none focus-visible:ring-0"
                >
                  <ModelsSection setView={setView} />
                </TabsContent>

                <TabsContent
                  value="chat"
                  className="mt-0 focus-visible:outline-none focus-visible:ring-0"
                >
                  <ChatSettingsSection setView={setView} />
                </TabsContent>

                <TabsContent
                  value="sharing"
                  className="mt-0 focus-visible:outline-none focus-visible:ring-0"
                >
                  <SessionSharingSection />
                </TabsContent>

                <TabsContent
                  value="app"
                  className="mt-0 focus-visible:outline-none focus-visible:ring-0"
                >
                  <AppSettingsSection scrollToSection={viewOptions.section} />
                </TabsContent>
              </ScrollArea>
            </Tabs>
          </div>
        </div>
      </MainPanelLayout>
    </>
  );
}
