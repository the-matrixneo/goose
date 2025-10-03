import type { ExtensionConfig } from '../../../api';
import { toastService } from '../../../toasts';
import { DEFAULT_EXTENSION_TIMEOUT } from './utils';

/**
 * Build an extension config for stdio from the deeplink URL
 */
function getStdioConfig(
  cmd: string,
  parsedUrl: URL,
  name: string,
  description: string,
  timeout: number
) {
  // Validate that the command is one of the allowed commands
  const allowedCommands = [
    'cu',
    'docker',
    'jbang',
    'npx',
    'uvx',
    'goosed',
    'npx.cmd',
    'i-ching-mcp-server',
  ];
  if (!allowedCommands.includes(cmd)) {
    toastService.handleError(
      'Invalid Command',
      `Failed to install extension: Invalid command: ${cmd}. Only ${allowedCommands.join(', ')} are allowed.`,
      { shouldThrow: true }
    );
  }

  // Check for security risk with npx -c command
  const args = parsedUrl.searchParams.getAll('arg');
  if (cmd === 'npx' && args.includes('-c')) {
    toastService.handleError(
      'Security Risk',
      'Failed to install extension: npx with -c argument can lead to code injection',
      { shouldThrow: true }
    );
  }

  const envList = parsedUrl.searchParams.getAll('env');

  // Create the extension config
  const config: ExtensionConfig = {
    name: name,
    type: 'stdio',
    cmd: cmd,
    description,
    args: args,
    envs:
      envList.length > 0
        ? Object.fromEntries(
            envList.map((env) => {
              const [key] = env.split('=');
              return [key, '']; // Initialize with empty string as value
            })
          )
        : undefined,
    timeout: timeout,
  };

  return config;
}

/**
 * Build an extension config for SSE from the deeplink URL
 */
function getSseConfig(remoteUrl: string, name: string, description: string, timeout: number) {
  const config: ExtensionConfig = {
    name,
    type: 'sse',
    uri: remoteUrl,
    description,
    timeout: timeout,
  };

  return config;
}

/**
 * Build an extension config for Streamable HTTP from the deeplink URL
 */
function getStreamableHttpConfig(
  remoteUrl: string,
  name: string,
  description: string,
  timeout: number,
  headers?: { [key: string]: string },
  envs?: { [key: string]: string }
) {
  const config: ExtensionConfig = {
    name,
    type: 'streamable_http',
    uri: remoteUrl,
    description,
    timeout: timeout,
    headers: headers,
    envs: envs,
  };

  return config;
}

/**
 * Handles adding an extension from a deeplink URL
 */
export async function addExtensionFromDeepLink(
  url: string,
  addExtensionFn: (
    name: string,
    extensionConfig: ExtensionConfig,
    enabled: boolean
  ) => Promise<void>,
  setView: (
    view: string,
    options: { showEnvVars: boolean; deepLinkConfig?: ExtensionConfig }
  ) => void
) {
  const parsedUrl = new URL(url);

  if (parsedUrl.protocol !== 'goose:') {
    toastService.handleError(
      'Invalid Protocol',
      'Failed to install extension: Invalid protocol: URL must use the goose:// scheme',
      { shouldThrow: true }
    );
  }

  // Check that all required fields are present and not empty
  const requiredFields = ['name'];

  for (const field of requiredFields) {
    const value = parsedUrl.searchParams.get(field);
    if (!value || value.trim() === '') {
      toastService.handleError(
        'Missing Field',
        `Failed to install extension: The link is missing required field '${field}'`,
        { shouldThrow: true }
      );
    }
  }

  const name = parsedUrl.searchParams.get('name')!;
  const parsedTimeout = parsedUrl.searchParams.get('timeout');
  const timeout = parsedTimeout ? parseInt(parsedTimeout, 10) : DEFAULT_EXTENSION_TIMEOUT;
  const description = parsedUrl.searchParams.get('description');

  const cmd = parsedUrl.searchParams.get('cmd');
  const remoteUrl = parsedUrl.searchParams.get('url');
  // Support both 'transport' and 'type' parameters for consistency
  const transportType =
    parsedUrl.searchParams.get('transport') || parsedUrl.searchParams.get('type') || 'sse'; // Default to SSE for backward compatibility

  const headerParams = parsedUrl.searchParams.getAll('header');
  const headers =
    headerParams.length > 0
      ? Object.fromEntries(
          headerParams.map((header) => {
            const [key, value] = header.split('=');
            return [key, decodeURIComponent(value || '')];
          })
        )
      : undefined;

  // Parse env vars for remote extensions (same logic as stdio)
  const envList = parsedUrl.searchParams.getAll('env');
  const envs =
    envList.length > 0
      ? Object.fromEntries(
          envList.map((env) => {
            const [key] = env.split('=');
            return [key, ''];
          })
        )
      : undefined;

  const config = remoteUrl
    ? transportType === 'streamable_http'
      ? getStreamableHttpConfig(remoteUrl, name, description || '', timeout, headers, envs)
      : getSseConfig(remoteUrl, name, description || '', timeout)
    : getStdioConfig(cmd!, parsedUrl, name, description || '', timeout);

  // Check if extension requires env vars or headers and go to settings if so
  const hasEnvVars = config.envs && Object.keys(config.envs).length > 0;
  const hasHeaders =
    config.type === 'streamable_http' && config.headers && Object.keys(config.headers).length > 0;

  if (hasEnvVars || hasHeaders) {
    console.log(
      'Environment variables or headers required, redirecting to extensions with env variables modal showing'
    );
    setView('extensions', { deepLinkConfig: config, showEnvVars: true });
    return;
  }

  console.log('No env vars required, activating extension directly');
  // Note: deeplink activation doesn't have access to sessionId
  // The extension will be added to config but not activated in the current session
  // It will be activated when the next session starts
  await addExtensionFn(config.name, config, true);

  // Show success toast and navigate to extensions page
  toastService.success({
    title: 'Extension Installed',
    msg: `${config.name} extension has been installed successfully. Start a new chat session to use it.`,
  });

  // Navigate to extensions page to show the newly installed extension
  setView('extensions', { deepLinkConfig: config, showEnvVars: false });
}
