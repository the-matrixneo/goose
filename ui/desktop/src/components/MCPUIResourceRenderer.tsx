import { UIResourceRenderer, UIActionResult } from '@mcp-ui/client';
import { ResourceContent } from '../types/message';
import { useCallback } from 'react';
import { toast } from 'react-toastify';
import { gooseComponentLibrary } from './GooseComponentLibrary';

interface MCPUIResourceRendererProps {
  content: ResourceContent;
}

export default function MCPUIResourceRenderer({ content }: MCPUIResourceRendererProps) {
  const handleAction = (action: UIActionResult) => {
    console.log(
      `MCP UI message received (but only handled with a toast notification for now):`,
      action
    );
    toast.info(`${action.type} message sent from MCP UI, refer to console for more info`, {
      data: action,
    });
    return { status: 'handled', message: `${action.type} action logged` };
  };

  const handleUIAction = useCallback(async (result: UIActionResult) => {
    switch (result.type) {
      case 'intent': {
        // TODO: Implement intent handling
        handleAction(result);
        break;
      }

      case 'link': {
        // TODO: Implement link handling
        handleAction(result);
        break;
      }

      case 'notify': {
        // TODO: Implement notify handling
        handleAction(result);
        break;
      }

      case 'prompt': {
        // TODO: Implement prompt handling
        handleAction(result);
        break;
      }

      case 'tool': {
        // TODO: Implement tool call handling
        handleAction(result);
        break;
      }

      default: {
        console.warn('unsupported message sent from MCP-UI:', result);
        break;
      }
    }
  }, []);

  // Determine rendering props based on MIME type
  const isRemoteDOM = content.resource.mimeType === 'application/vnd.mcp-ui.remote-dom';
  
  return (
    <div className="mt-3 p-4 border border-borderSubtle rounded-lg bg-background-muted">
      <div className="overflow-hidden rounded-sm">
        <UIResourceRenderer
          resource={content.resource}
          onUIAction={handleUIAction}
          htmlProps={!isRemoteDOM ? {
            autoResizeIframe: {
              height: true,
              width: false, // set to false to allow for responsive design
            },
          } : undefined}
          remoteDomProps={isRemoteDOM ? {
            library: gooseComponentLibrary,
            framework: 'react' as const
          } : undefined}
        />
      </div>
    </div>
  );
}
