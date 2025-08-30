import { useSidecar } from './SidecarContext';
import MCPUIResourceRenderer from '../MCPUIResourceRenderer';

export default function SidecarPanel() {
  const { isOpen, content, close } = useSidecar();

  return (
    <div
      className={`bg-background-default border-l border-borderSubtle h-full overflow-hidden flex-shrink-0 pointer-events-auto`}
      aria-hidden={!isOpen}
    >
      {isOpen && (
        <div className="h-full flex flex-col pointer-events-auto">
          <div className="sticky top-0 z-[200] flex items-center justify-between px-3 py-2 border-b border-borderSubtle bg-background-default/95 backdrop-blur supports-[backdrop-filter]:bg-background-default/70 pointer-events-auto">
            <div className="text-xs font-sans text-textSubtle uppercase tracking-wide">
              MCP‑UI sidecar
            </div>
            <button
              className="no-drag inline-flex items-center justify-center w-6 h-6 cursor-pointer rounded hover:bg-bgSubtle text-textSubtle hover:text-textStandard relative z-[250] pointer-events-auto"
              onClick={() => {
                console.log('Close button clicked!');
                close();
              }}
              aria-label="Close side panel"
              title="Close"
            >
              ×
            </button>
          </div>
          <div className="flex-1 min-h-0 overflow-auto p-3">
            {content.kind === 'mcp-ui' && (
              <MCPUIResourceRenderer
                content={content.resource}
                appendPromptToChat={content.appendPromptToChat}
              />
            )}
          </div>
        </div>
      )}
    </div>
  );
}
