#!/usr/bin/env node

/**
 * Enhanced test MCP server for Remote DOM integration testing
 * 
 * This server provides comprehensive Remote DOM resources to test our Goose implementation
 * with all 5 missing primitives and complex layouts.
 * Usage: node test-remote-dom-server.js
 */

const { Server } = require('@modelcontextprotocol/sdk/server/index.js');
const { StdioServerTransport } = require('@modelcontextprotocol/sdk/server/stdio.js');

// Create the server
const server = new Server(
  {
    name: 'test-remote-dom-server',
    version: '2.0.0',
  },
  {
    capabilities: {
      tools: {},
      resources: {},
    },
  }
);

// Add tools that return Remote DOM resources
server.setRequestHandler('tools/list', async () => {
  return {
    tools: [
      {
        name: 'create_simple_ui',
        description: 'Creates a simple Remote DOM UI for basic testing',
        inputSchema: {
          type: 'object',
          properties: {
            title: {
              type: 'string',
              description: 'Title for the UI',
              default: 'Simple Test'
            }
          }
        }
      },
      {
        name: 'create_complex_layout',
        description: 'Creates a complex layout testing all 5 primitives',
        inputSchema: {
          type: 'object',
          properties: {
            theme: {
              type: 'string',
              description: 'Theme for the layout',
              enum: ['dashboard', 'form', 'content'],
              default: 'dashboard'
            }
          }
        }
      },
      {
        name: 'test_all_components',
        description: 'Creates a comprehensive test of all 20+ components',
        inputSchema: {
          type: 'object',
          properties: {}
        }
      }
    ]
  };
});

server.setRequestHandler('tools/call', async (request) => {
  const { name, arguments: args } = request.params;
  
  if (name === 'create_simple_ui') {
    const title = args?.title || 'Simple Test';
    
    const remoteDOMScript = `
// Simple Remote DOM test - Basic primitives
import { ui } from '@mcp-ui/remote-dom';

const container = ui.container({ padding: 'lg' });

const content = ui.stack({ direction: 'vertical', gap: 'md' }, [
  ui.heading({ level: 1 }, '${title}'),
  ui.text({ variant: 'body' }, 'Testing basic Remote DOM integration with Goose components.'),
  ui.button({ 
    variant: 'default',
    onClick: () => ui.action('tool', { toolName: 'test_tool', params: { message: 'Button clicked!' } })
  }, 'Test Action'),
  ui.separator(),
  ui.text({ variant: 'caption' }, 'This uses the new Text and Heading primitives.')
]);

container.appendChild(content);
export default container;
`;

    return {
      content: [
        {
          type: 'resource',
          resource: {
            uri: 'ui://simple-test',
            mimeType: 'application/vnd.mcp-ui.remote-dom',
            text: remoteDOMScript
          }
        }
      ]
    };
  }
  
  if (name === 'create_complex_layout') {
    const theme = args?.theme || 'dashboard';
    
    const remoteDOMScript = `
// Complex layout test - All 5 primitives working together
import { ui } from '@mcp-ui/remote-dom';

const root = ui.container({ padding: 'lg' });

// Header section
const header = ui.stack({ direction: 'horizontal', gap: 'md', justify: 'between', align: 'center' }, [
  ui.heading({ level: 1 }, '${theme.charAt(0).toUpperCase() + theme.slice(1)} Layout'),
  ui.badge({}, 'Remote DOM')
]);

// Main content grid
const mainGrid = ui.grid({ columns: 2, gap: 'lg' }, [
  // Left column
  ui.stack({ direction: 'vertical', gap: 'md' }, [
    ui.heading({ level: 2 }, 'Content Section'),
    ui.text({ variant: 'body' }, 'This demonstrates the Stack component with vertical layout.'),
    ui.card({}, [
      ui.text({ variant: 'large' }, 'Card with nested content'),
      ui.separator(),
      ui.text({ variant: 'caption' }, 'Using Container, Stack, Grid, Text, and Heading primitives')
    ])
  ]),
  
  // Right column
  ui.container({ padding: 'md' }, [
    ui.heading({ level: 3 }, 'Interactive Elements'),
    ui.stack({ direction: 'vertical', gap: 'sm' }, [
      ui.button({ 
        variant: 'default',
        onClick: () => ui.action('prompt', { prompt: 'Test prompt from Remote DOM' })
      }, 'Send Prompt'),
      ui.button({ 
        variant: 'outline',
        onClick: () => ui.action('notify', { message: 'Notification from Remote DOM!' })
      }, 'Show Notification'),
      ui.button({ 
        variant: 'ghost',
        onClick: () => ui.action('link', { url: 'https://github.com/block/goose' })
      }, 'Open Link')
    ])
  ])
]);

// Footer
const footer = ui.container({ padding: 'sm' }, [
  ui.text({ variant: 'small' }, 'All 5 primitives: Text, Heading, Container, Stack, Grid ✓')
]);

const layout = ui.stack({ direction: 'vertical', gap: 'lg' }, [
  header,
  mainGrid,
  footer
]);

root.appendChild(layout);
export default root;
`;

    return {
      content: [
        {
          type: 'resource',
          resource: {
            uri: 'ui://complex-layout',
            mimeType: 'application/vnd.mcp-ui.remote-dom',
            text: remoteDOMScript
          }
        }
      ]
    };
  }
  
  if (name === 'test_all_components') {
    const remoteDOMScript = `
// Comprehensive component test - All 20+ components
import { ui } from '@mcp-ui/remote-dom';

const root = ui.container({ padding: 'lg' });

const content = ui.stack({ direction: 'vertical', gap: 'lg' }, [
  // Header
  ui.heading({ level: 1 }, 'Complete Component Library Test'),
  ui.text({ variant: 'body' }, 'Testing all available Goose components in Remote DOM context.'),
  
  // Form Components Section
  ui.heading({ level: 2 }, 'Form Components'),
  ui.grid({ columns: 2, gap: 'md' }, [
    ui.stack({ direction: 'vertical', gap: 'sm' }, [
      ui.label({}, 'Input Field'),
      ui.input({ placeholder: 'Test input' }),
      ui.checkbox({}, 'Checkbox option'),
      ui.switch({})
    ]),
    ui.stack({ direction: 'vertical', gap: 'sm' }, [
      ui.label({}, 'Textarea'),
      ui.textarea({ placeholder: 'Test textarea' }),
      ui.button({ variant: 'default' }, 'Primary Button'),
      ui.button({ variant: 'outline' }, 'Outline Button')
    ])
  ]),
  
  ui.separator(),
  
  // Layout Components Section
  ui.heading({ level: 2 }, 'Layout Components'),
  ui.tabs({}, [
    ui['tabs-list']({}, [
      ui['tabs-trigger']({ value: 'tab1' }, 'Tab 1'),
      ui['tabs-trigger']({ value: 'tab2' }, 'Tab 2')
    ]),
    ui['tabs-content']({ value: 'tab1' }, [
      ui.card({}, [
        ui.text({}, 'Card content in tab 1'),
        ui.badge({}, 'Badge')
      ])
    ]),
    ui['tabs-content']({ value: 'tab2' }, [
      ui.text({}, 'Content for tab 2')
    ])
  ]),
  
  ui.separator(),
  
  // Feedback Components Section
  ui.heading({ level: 2 }, 'Feedback Components'),
  ui.stack({ direction: 'horizontal', gap: 'md', align: 'center' }, [
    ui.badge({ variant: 'default' }, 'Default'),
    ui.badge({ variant: 'secondary' }, 'Secondary'),
    ui.skeleton({ className: 'h-4 w-20' }),
    ui.tooltip({}, 'Hover me')
  ]),
  
  ui.separator(),
  
  // Primitives Section
  ui.heading({ level: 2 }, 'New Primitives'),
  ui.container({ padding: 'md' }, [
    ui.text({ variant: 'large' }, 'Large text variant'),
    ui.text({ variant: 'body' }, 'Body text variant'),
    ui.text({ variant: 'small' }, 'Small text variant'),
    ui.text({ variant: 'caption' }, 'Caption text variant'),
    ui.stack({ direction: 'horizontal', gap: 'lg' }, [
      ui.heading({ level: 3 }, 'H3 Heading'),
      ui.heading({ level: 4 }, 'H4 Heading'),
      ui.heading({ level: 5 }, 'H5 Heading'),
      ui.heading({ level: 6 }, 'H6 Heading')
    ])
  ]),
  
  // Action Test Section
  ui.heading({ level: 2 }, 'Action Tests'),
  ui.stack({ direction: 'horizontal', gap: 'sm' }, [
    ui.button({ 
      onClick: () => ui.action('tool', { toolName: 'echo', params: { text: 'Tool action test' } })
    }, 'Tool Action'),
    ui.button({ 
      onClick: () => ui.action('prompt', { prompt: 'What is 2+2?' })
    }, 'Prompt Action'),
    ui.button({ 
      onClick: () => ui.action('notify', { message: 'Test notification!' })
    }, 'Notify Action'),
    ui.button({ 
      onClick: () => ui.action('intent', { intent: 'open-settings' })
    }, 'Intent Action')
  ])
]);

root.appendChild(content);
export default root;
`;

    return {
      content: [
        {
          type: 'resource',
          resource: {
            uri: 'ui://component-test',
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
  console.error('Enhanced Remote DOM MCP server started');
}

main().catch(console.error);
