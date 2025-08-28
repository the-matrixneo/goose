import { describe, it, expect, vi, beforeEach } from 'vitest';
import { extensionApiCall, addExtensionToAgent, removeFromAgent, sanitizeName } from './agent-api';
import * as toasts from '../../../toasts';
import { ExtensionConfig } from '../../../api';

// Mock dependencies
vi.mock('../../../toasts');
vi.mock('./utils');
vi.mock('../../../api', () => ({
  addExtension: vi.fn(),
  removeExtension: vi.fn(),
}));

const mockToastService = vi.mocked(toasts.toastService);

describe('Agent API', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    mockToastService.configure = vi.fn();
    mockToastService.loading = vi.fn().mockReturnValue('toast-id');
    mockToastService.success = vi.fn();
    mockToastService.error = vi.fn();
    mockToastService.dismiss = vi.fn();
  });

  describe('sanitizeName', () => {
    it('should sanitize extension names correctly', () => {
      expect(sanitizeName('Test Extension')).toBe('testextension');
      expect(sanitizeName('My-Extension_Name')).toBe('myextensionname');
      expect(sanitizeName('UPPERCASE')).toBe('uppercase');
    });
  });

  describe('extensionApiCall', () => {
    const mockExtensionConfig: ExtensionConfig = {
      type: 'stdio',
      name: 'test-extension',
      cmd: 'python',
      args: ['script.py'],
    };

    it('should make successful API call for adding extension', async () => {
      const { addExtension } = await import('../../../api');
      const mockResponse = {
        data: '{"success": true}',
        error: undefined,
        request: {} as Request,
        response: {} as Response,
      };
      vi.mocked(addExtension).mockResolvedValue(mockResponse);

      const response = await extensionApiCall(true, mockExtensionConfig);

      expect(addExtension).toHaveBeenCalledWith({
        body: {
          name: 'test-extension',
          config: mockExtensionConfig,
          enabled: true,
        },
      });

      expect(mockToastService.loading).toHaveBeenCalledWith({
        title: 'test-extension',
        msg: 'Activating test-extension extension...',
      });

      expect(mockToastService.success).toHaveBeenCalledWith({
        title: 'test-extension',
        msg: 'Successfully activated extension',
      });

      expect(response).toBeDefined();
    });

    it('should make successful API call for removing extension', async () => {
      const { removeExtension } = await import('../../../api');
      const mockResponse = {
        data: '{"success": true}',
        error: undefined,
        request: {} as Request,
        response: {} as Response,
      };
      vi.mocked(removeExtension).mockResolvedValue(mockResponse);

      const response = await extensionApiCall(false, 'test-extension');

      expect(removeExtension).toHaveBeenCalledWith({
        path: { name: 'test-extension' },
      });

      expect(mockToastService.loading).not.toHaveBeenCalled(); // No loading toast for removal
      expect(mockToastService.success).toHaveBeenCalledWith({
        title: 'test-extension',
        msg: 'Successfully deactivated extension',
      });

      expect(response).toBeDefined();
    });

    it('should handle API error responses', async () => {
      const { addExtension } = await import('../../../api');
      const mockResponse = {
        data: undefined,
        error: { message: 'Extension not found' },
        request: {} as Request,
        response: {} as Response,
      };
      vi.mocked(addExtension).mockResolvedValue(mockResponse);

      await expect(extensionApiCall(true, mockExtensionConfig)).rejects.toThrow(
        'API error: Extension not found'
      );

      expect(mockToastService.error).toHaveBeenCalledWith({
        title: 'test-extension',
        msg: 'Failed to activate extension',
        traceback: 'API error: Extension not found',
      });
    });

    it('should configure toast service with options', async () => {
      const { addExtension } = await import('../../../api');
      const mockResponse = {
        data: '{"success": true}',
        error: undefined,
        request: {} as Request,
        response: {} as Response,
      };
      vi.mocked(addExtension).mockResolvedValue(mockResponse);

      await extensionApiCall(true, mockExtensionConfig, { silent: true });

      expect(mockToastService.configure).toHaveBeenCalledWith({ silent: true });
    });

    it('should handle network errors', async () => {
      const { addExtension } = await import('../../../api');
      const networkError = new Error('Network error');
      vi.mocked(addExtension).mockRejectedValue(networkError);

      await expect(extensionApiCall(true, mockExtensionConfig)).rejects.toThrow('Network error');

      expect(mockToastService.error).toHaveBeenCalledWith({
        title: 'test-extension',
        msg: 'Network error',
        traceback: 'Network error',
      });
    });
  });

  describe('addToAgent', () => {
    const mockExtensionConfig: ExtensionConfig = {
      type: 'stdio',
      name: 'Test Extension',
      cmd: 'python',
      args: ['script.py'],
    };

    it('should add stdio extension to agent with shim replacement', async () => {
      const { addExtension } = await import('../../../api');
      const { replaceWithShims } = await import('./utils');

      const mockResponse = {
        data: '{"success": true}',
        error: undefined,
        request: {} as Request,
        response: {} as Response,
      };
      vi.mocked(addExtension).mockResolvedValue(mockResponse);
      vi.mocked(replaceWithShims).mockResolvedValue('/path/to/python');

      await addExtensionToAgent(mockExtensionConfig);

      expect(addExtension).toHaveBeenCalledWith({
        body: {
          name: 'testextension',
          config: {
            ...mockExtensionConfig,
            name: 'testextension',
            cmd: '/path/to/python',
          },
          enabled: true,
        },
      });
    });

    it('should add non-stdio extension without shim replacement', async () => {
      const sseConfig: ExtensionConfig = {
        type: 'sse',
        name: 'SSE Extension',
        uri: 'http://localhost:8080/events',
      };

      const { addExtension } = await import('../../../api');
      const mockResponse = {
        data: '{"success": true}',
        error: undefined,
        request: {} as Request,
        response: {} as Response,
      };
      vi.mocked(addExtension).mockResolvedValue(mockResponse);

      await addExtensionToAgent(sseConfig);

      expect(addExtension).toHaveBeenCalledWith({
        body: {
          name: 'sseextension',
          config: {
            ...sseConfig,
            name: 'sseextension',
          },
          enabled: true,
        },
      });
    });

    it('should handle errors gracefully', async () => {
      const { addExtension } = await import('../../../api');
      const error = new Error('Some API error');
      vi.mocked(addExtension).mockRejectedValue(error);

      await expect(addExtensionToAgent(mockExtensionConfig)).rejects.toThrow('Some API error');
    });
  });

  describe('removeFromAgent', () => {
    it('should remove extension from agent', async () => {
      const { removeExtension } = await import('../../../api');
      const mockResponse = {
        data: '{"success": true}',
        error: undefined,
        request: {} as Request,
        response: {} as Response,
      };
      vi.mocked(removeExtension).mockResolvedValue(mockResponse);

      await removeFromAgent('Test Extension');

      expect(removeExtension).toHaveBeenCalledWith({
        path: { name: 'testextension' },
      });
    });

    it('should handle removal errors', async () => {
      const { removeExtension } = await import('../../../api');
      const error = new Error('Not found');
      vi.mocked(removeExtension).mockRejectedValue(error);

      await expect(removeFromAgent('Test Extension')).rejects.toThrow('Not found');

      expect(mockToastService.error).toHaveBeenCalled();
    });

    it('should handle delete option', async () => {
      const { removeExtension } = await import('../../../api');
      const mockResponse = {
        data: '{"success": true}',
        error: undefined,
        request: {} as Request,
        response: {} as Response,
      };
      vi.mocked(removeExtension).mockResolvedValue(mockResponse);

      await removeFromAgent('Test Extension', { isDelete: true });

      expect(removeExtension).toHaveBeenCalledWith({
        path: { name: 'testextension' },
      });
    });
  });
});
