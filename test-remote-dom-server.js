#!/usr/bin/env node

/**
 * Simple test MCP server for Remote DOM integration testing
 * 
 * This server provides a basic Remote DOM resource to test our Goose implementation.
 * Usage: node test-remote-dom-server.js
 */

const { Server } = require('@modelcontextprotocol/sdk/server/index.js');
const { StdioServerTransport } = require('@modelcontextprotocol/sdk/server/stdio.js');

// Create the server
const server = new Server(
  {
    name: 'test-remote-dom-server',
    version: '1.0.0',
  },
  {
    capabilities: {
      tools: {},
      resources: {},
    },
  }
);

// Add a simple tool that returns a Remote DOM resource
server.setRequestHandler('tools/list', async () => {
  return {
    tools: [
      {
        name: 'create_remote_dom_ui',
        description: 'Creates a simple Remote DOM UI for testing',
        inputSchema: {
          type: 'object',
          properties: {
            title: {
              type: 'string',
              description: 'Title for the UI',
              default: 'Test Remote DOM'
            }
          }
        }
      }
    ]
  };
});

server.setRequestHandler('tools/call', async (request) => {
  const { name, arguments: args } = request.params;
  
  if (name === 'create_remote_dom_ui') {
    const title = args?.title || 'Test Remote DOM';
    
    // Create a simple Remote DOM script
    const remoteDOMScript = `
// Simple Remote DOM test script
import { ui } from '@mcp-ui/remote-dom';

const container = ui.container({ padding: 'lg' });

const heading = ui.heading({ level: 1 }, '${title}');
const text = ui.text({ variant: 'body' }, 'This is a test of Remote DOM integration with Goose components.');

const stack = ui.stack({ direction: 'vertical', gap: 'md' }, [
  heading,
  text,
  ui.button({ 
    variant: 'default',
    onClick: () => ui.action('tool', { toolName: 'test_tool', params: { message: 'Button clicked!' } })
  }, 'Test Button'),
  ui.separator(),
  ui.card({}, [
    ui.text({ variant: 'caption' }, 'This card uses Goose design system components')
  ])
]);

container.appendChild(stack);
export default container;
`;

    return {
      content: [
        {
          type: 'resource',
          resource: {
            uri: 'ui://test-remote-dom',
            mimeType: 'application/vnd.mcp-ui.remote-dom',
            text: remoteDOMScript
          }
        }
      ]
    };
  }
  
  throw new Error(`Unknown tool: ${name}`);
});

// Start the server
async function main() {
  const transport = new StdioServerTransport();
  await server.connect(transport);
  console.error('Test Remote DOM MCP server started');
}

main().catch(console.error);
