import { ScrollArea } from '../ui/scroll-area';
import { View, ViewOptions } from '../../App';
import ExtensionsSection from '../settings/extensions/ExtensionsSection';
import { ExtensionConfig } from '../../api';
import { MainPanelLayout } from '../Layout/MainPanelLayout';
import { Button } from '../ui/button';
import { Plus } from 'lucide-react';
import { GPSIcon } from '../ui/icons';
import { useState, useEffect } from 'react';
import ExtensionModal from '../settings/extensions/modal/ExtensionModal';
import {
  getDefaultFormData,
  ExtensionFormData,
  createExtensionConfig,
} from '../settings/extensions/utils';
import { activateExtension } from '../settings/extensions/index';
import { useConfig } from '../ConfigContext';

export type ExtensionsViewOptions = {
  deepLinkConfig?: ExtensionConfig;
  showEnvVars?: boolean;
};

export default function ExtensionsView({
  viewOptions,
  setView,
}: {
  onClose: () => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  viewOptions: ExtensionsViewOptions;
}) {
  const [isAddModalOpen, setIsAddModalOpen] = useState(false);
  const [refreshKey, setRefreshKey] = useState(0);
  const { addExtension } = useConfig();

  const handleModalClose = () => {
    setIsAddModalOpen(false);
  };

  const handleAddExtension = async (formData: ExtensionFormData) => {
    // Close the modal immediately
    handleModalClose();

    const extensionConfig = createExtensionConfig(formData);
    try {
      await activateExtension({ addToConfig: addExtension, extensionConfig: extensionConfig });
      // Trigger a refresh of the extensions list
      setRefreshKey((prevKey) => prevKey + 1);
    } catch (error) {
      console.error('Failed to activate extension:', error);
      // Even if activation fails, we don't reopen the modal
    }
  };

  return (
    <MainPanelLayout>
      <div className="flex-1 flex flex-col min-h-0 mt-6">
        {/* Content Area */}
        <div className="flex flex-col mt-7 mb-4 px-2">
          <div className="flex justify-between items-center mb-1">
            <h1 className="text-4xl font-light">Extensions</h1>
          </div>
          <p className="text-sm text-text-muted">
            These extensions use the Model Context Protocol (MCP). They can expand Goose's
            capabilities using three main components: Prompts, Resources, and Tools.
          </p>
        </div>

        {/* Buttons between header and extensions list */}
        <div className="flex gap-4 mb-8 px-2">
          <Button
            className="flex items-center gap-2 justify-center"
            variant="default"
            onClick={() => setIsAddModalOpen(true)}
          >
            <Plus className="h-4 w-4" />
            Add custom extension
          </Button>
          <Button
            className="flex items-center gap-2 justify-center"
            variant="secondary"
            onClick={() => window.open('https://block.github.io/goose/v1/extensions/', '_blank')}
          >
            <GPSIcon size={12} />
            Browse extensions
          </Button>
        </div>

        <div className="flex-1 min-h-0 relative">
          <ScrollArea className="h-full pr-4">
            <ExtensionsSection
              key={refreshKey}
              deepLinkConfig={viewOptions.deepLinkConfig}
              showEnvVars={viewOptions.showEnvVars}
              hideButtons={true}
            />
          </ScrollArea>
        </div>
      </div>

      {/* Modal for adding a new extension */}
      {isAddModalOpen && (
        <ExtensionModal
          title="Add custom extension"
          initialData={getDefaultFormData()}
          onClose={handleModalClose}
          onSubmit={handleAddExtension}
          submitLabel="Add Extension"
          modalType={'add'}
        />
      )}
    </MainPanelLayout>
  );
}
