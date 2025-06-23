import { ScrollArea } from '../ui/scroll-area';
import { SidebarTrigger, useSidebar } from '../ui/sidebar';
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

  const safeIsMacOS = (window?.electron?.platform || 'darwin') === 'darwin';

  // Calculate padding based on sidebar state and macOS
  const headerPadding = !isSidebarOpen ? (safeIsMacOS ? 'pl-20' : 'pl-12') : 'pl-4';

  return (
    <>
      <MainPanelLayout>
        <div className="h-12 flex items-center justify-between">
          <div className={`flex items-center ${headerPadding}`}>
            <SidebarTrigger className="no-drag" />
          </div>
        </div>

        <div className="flex-1 flex flex-col min-h-0">
          {/* Content Area */}
          <div className="flex flex-col mt-4 mb-6 px-6">
            <h1 className="text-4xl font-light">Settings</h1>
            <h3 className="text-sm text-text-muted mt-2">
              Configure your Goose experience with models, extensions, and preferences.
            </h3>
          </div>

          <div className="flex-1 min-h-0 relative px-6">
            <ScrollArea className="h-full">
              <div className="h-full relative">
                <div className="space-y-8 pt-4">
                  {/* Models Section */}
                  <ModelsSection setView={setView} />
                  {/* Extensions Section */}
                  <ExtensionsSection
                    deepLinkConfig={viewOptions.deepLinkConfig}
                    showEnvVars={viewOptions.showEnvVars}
                  />
                  {/* Goose Modes */}
                  <ModeSection setView={setView} />
                  {/*Session sharing*/}
                  <SessionSharingSection />
                  {/* Response Styles */}
                  <ResponseStylesSection />
                  {/* Tool Selection Strategy */}
                  <ToolSelectionStrategySection setView={setView} />
                  {/* App Settings */}
                  <AppSettingsSection scrollToSection={viewOptions.section} />
                </div>
              </div>
            </ScrollArea>
          </div>
        </div>
      </MainPanelLayout>
    </>
  );
}
