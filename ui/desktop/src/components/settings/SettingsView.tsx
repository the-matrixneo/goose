import { ScrollArea } from '../ui/scroll-area';
import { SidebarTrigger, useSidebar } from '../ui/sidebar';
import { Tabs, TabsContent, TabsList, TabsTrigger } from '../ui/vertical-tabs';
import type { View, ViewOptions } from '../../App';
import ExtensionsSection from './extensions/ExtensionsSection';
import ModelsSection from './models/ModelsSection';
import { ModeSection } from './mode/ModeSection';
import { ToolSelectionStrategySection } from './tool_selection_strategy/ToolSelectionStrategySection';
import SessionSharingSection from './sessions/SessionSharingSection';
import { ResponseStylesSection } from './response_styles/ResponseStylesSection';
import AppSettingsSection from './app/AppSettingsSection';
import { ExtensionConfig } from '../../api';
import MoreMenuLayout from '../more_menu/MoreMenuLayout';
import { MainPanelLayout } from '../Layout/MainPanelLayout';
import { Bot, Puzzle, Settings, Share2, Palette, Wrench, Monitor, Target } from 'lucide-react';
import { useState, useEffect } from 'react';

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
  const { open: isSidebarOpen } = useSidebar();
  const [activeTab, setActiveTab] = useState('models');

  const safeIsMacOS = (window?.electron?.platform || 'darwin') === 'darwin';

  // Calculate padding based on sidebar state and macOS
  const headerPadding = !isSidebarOpen ? (safeIsMacOS ? 'pl-20' : 'pl-12') : 'pl-4';

  // Determine initial tab based on section prop
  useEffect(() => {
    if (viewOptions.section) {
      // Map section names to tab values
      const sectionToTab: Record<string, string> = {
        update: 'app',
        models: 'models',
        extensions: 'extensions',
        modes: 'modes',
        sharing: 'sharing',
        styles: 'styles',
        tools: 'tools',
        app: 'app',
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
          <div className="flex flex-col mt-8 mb-6 px-4">
            <h1 className="text-4xl font-light">Settings</h1>
          </div>

          <div className="flex-1 min-h-0 relative">
            <Tabs value={activeTab} onValueChange={setActiveTab} className="h-full flex">
              <TabsList className="w-48 h-full">
                <TabsTrigger value="models" className="flex gap-2">
                  <Bot className="h-4 w-4" />
                  Models
                </TabsTrigger>
                <TabsTrigger value="extensions" className="flex gap-2">
                  <Puzzle className="h-4 w-4" />
                  Extensions
                </TabsTrigger>
                <TabsTrigger value="modes" className="flex gap-2">
                  <Wrench className="h-4 w-4" />
                  Modes
                </TabsTrigger>
                <TabsTrigger value="sharing" className="flex gap-2">
                  <Share2 className="h-4 w-4" />
                  Session Sharing
                </TabsTrigger>
                <TabsTrigger value="styles" className="flex gap-2">
                  <Palette className="h-4 w-4" />
                  Response Styles
                </TabsTrigger>
                <TabsTrigger value="tools" className="flex gap-2">
                  <Target className="h-4 w-4" />
                  Tool Selection
                </TabsTrigger>
                <TabsTrigger value="app" className="flex gap-2">
                  <Monitor className="h-4 w-4" />
                  App Settings
                </TabsTrigger>
              </TabsList>

              <div className="flex-1 ml-6">
                <ScrollArea className="h-full">
                  <TabsContent value="models" className="mt-0">
                    <ModelsSection setView={setView} />
                  </TabsContent>

                  <TabsContent value="extensions" className="mt-0">
                    <ExtensionsSection
                      deepLinkConfig={viewOptions.deepLinkConfig}
                      showEnvVars={viewOptions.showEnvVars}
                    />
                  </TabsContent>

                  <TabsContent value="modes" className="mt-0">
                    <ModeSection setView={setView} />
                  </TabsContent>

                  <TabsContent value="sharing" className="mt-0">
                    <SessionSharingSection />
                  </TabsContent>

                  <TabsContent value="styles" className="mt-0">
                    <ResponseStylesSection />
                  </TabsContent>

                  <TabsContent value="tools" className="mt-0">
                    <ToolSelectionStrategySection setView={setView} />
                  </TabsContent>

                  <TabsContent value="app" className="mt-0">
                    <AppSettingsSection scrollToSection={viewOptions.section} />
                  </TabsContent>
                </ScrollArea>
              </div>
            </Tabs>
          </div>
        </div>
      </MainPanelLayout>
    </>
  );
}
