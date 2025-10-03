import { View, ViewOptions } from '../../utils/navigationUtils';
import { useChatContext } from '../../contexts/ChatContext';
import ExtensionsSection from '../settings/extensions/ExtensionsSection';
import { ExtensionConfig } from '../../api';
import { MainPanelLayout } from '../Layout/MainPanelLayout';
import { Button } from '../ui/button';
import { Plus } from 'lucide-react';
import { GPSIcon } from '../ui/icons';
import { useState, useEffect } from 'react';
import kebabCase from 'lodash/kebabCase';
import ExtensionModal from '../settings/extensions/modal/ExtensionModal';
import {
  getDefaultFormData,
  ExtensionFormData,
  createExtensionConfig,
} from '../settings/extensions/utils';
import { activateExtension } from '../settings/extensions';
import { useConfig } from '../ConfigContext';

export type ExtensionsViewOptions = {
  deepLinkConfig?: ExtensionConfig;
  showEnvVars?: boolean;
};

export default function ExtensionsView({
  viewOptions,
}: {
  onClose: () => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  viewOptions: ExtensionsViewOptions;
}) {
  const [isAddModalOpen, setIsAddModalOpen] = useState(false);
  const [refreshKey, setRefreshKey] = useState(0);
  const { addExtension } = useConfig();
  const chatContext = useChatContext();
  const sessionId = chatContext?.chat.sessionId || '';

  if (!sessionId) {
    console.error('ExtensionsView: No session ID available');
  }

  // Only trigger refresh when deep link config changes AND we don't need to show env vars
  useEffect(() => {
    if (viewOptions.deepLinkConfig && !viewOptions.showEnvVars) {
      setRefreshKey((prevKey) => prevKey + 1);
    }
  }, [viewOptions.deepLinkConfig, viewOptions.showEnvVars]);

  const scrollToExtension = (extensionName: string) => {
    setTimeout(() => {
      const element = document.getElementById(`extension-${kebabCase(extensionName)}`);
      if (element) {
        element.scrollIntoView({
          behavior: 'smooth',
          block: 'center',
        });
        // Add a subtle highlight effect
        element.style.boxShadow = '0 0 0 2px rgba(59, 130, 246, 0.5)';
        setTimeout(() => {
          element.style.boxShadow = '';
        }, 2000);
      }
    }, 200);
  };

  // Scroll to extension whenever extensionId is provided (after refresh)
  useEffect(() => {
    if (viewOptions.deepLinkConfig?.name && refreshKey > 0) {
      scrollToExtension(viewOptions.deepLinkConfig?.name);
    }
  }, [viewOptions.deepLinkConfig?.name, refreshKey]);

  const handleModalClose = () => {
    setIsAddModalOpen(false);
  };

  const handleAddExtension = async (formData: ExtensionFormData) => {
    // Close the modal immediately
    handleModalClose();

    if (!sessionId) {
      console.warn('Cannot activate extension without session');
      setRefreshKey((prevKey) => prevKey + 1);
      return;
    }

    const extensionConfig = createExtensionConfig(formData);

    try {
      await activateExtension({
        addToConfig: addExtension,
        extensionConfig: extensionConfig,
        sessionId: sessionId,
      });
      // Trigger a refresh of the extensions list
      setRefreshKey((prevKey) => prevKey + 1);
    } catch (error) {
      console.error('Failed to activate extension:', error);
      setRefreshKey((prevKey) => prevKey + 1);
    }
  };

  return (
    <MainPanelLayout>
      <div className="flex flex-col min-w-0 flex-1 overflow-y-auto relative">
        <div className="bg-background-default px-8 pb-4 pt-16">
          <div className="flex flex-col page-transition">
            <div className="flex justify-between items-center mb-1">
              <h1 className="text-4xl font-light">Extensions</h1>
            </div>
            <p className="text-sm text-text-muted mb-6">
              These extensions use the Model Context Protocol (MCP). They can expand Goose's
              capabilities using three main components: Prompts, Resources, and Tools.
            </p>

            {/* Action Buttons */}
            <div className="flex gap-4 mb-8">
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
                onClick={() =>
                  window.open('https://block.github.io/goose/v1/extensions/', '_blank')
                }
              >
                <GPSIcon size={12} />
                Browse extensions
              </Button>
            </div>
          </div>
        </div>

        <div className="px-8 pb-16">
          <ExtensionsSection
            key={refreshKey}
            sessionId={sessionId}
            deepLinkConfig={viewOptions.deepLinkConfig}
            showEnvVars={viewOptions.showEnvVars}
            hideButtons={true}
            onModalClose={(extensionName: string) => {
              scrollToExtension(extensionName);
            }}
          />
        </div>

        {/* Bottom padding space - same as in hub.tsx */}
        <div className="block h-8" />
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
