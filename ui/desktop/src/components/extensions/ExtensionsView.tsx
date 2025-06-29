import { ScrollArea } from '../ui/scroll-area';
import { SidebarTrigger, useSidebar } from '../ui/sidebar';
import { View, ViewOptions } from '../../App';
import ExtensionsSection from '../settings/extensions/ExtensionsSection';
import { ExtensionConfig } from '../../api';
import { MainPanelLayout } from '../Layout/MainPanelLayout';

export type ExtensionsViewOptions = {
  deepLinkConfig?: ExtensionConfig;
  showEnvVars?: boolean;
};

export default function ExtensionsView({
  onClose,
  setView,
  viewOptions,
}: {
  onClose: () => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  viewOptions: ExtensionsViewOptions;
}) {
  const { open: isSidebarOpen } = useSidebar();
  const safeIsMacOS = (window?.electron?.platform || 'darwin') === 'darwin';

  // Calculate padding based on sidebar state and macOS
  const headerPadding = !isSidebarOpen ? (safeIsMacOS ? 'pl-20' : 'pl-12') : 'pl-4';

  return (
    <MainPanelLayout>
      <div className="flex-1 flex flex-col min-h-0 mt-6">
        {/* Content Area */}
        <div className="flex flex-col mt-7 mb-6 px-2">
          <h1 className="text-4xl font-light">Extensions</h1>
        </div>

        <div className="flex-1 min-h-0 relative">
          <ScrollArea className="h-full pr-4">
            <ExtensionsSection
              deepLinkConfig={viewOptions.deepLinkConfig}
              showEnvVars={viewOptions.showEnvVars}
            />
          </ScrollArea>
        </div>
      </div>
    </MainPanelLayout>
  );
}
