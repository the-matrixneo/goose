import { describe, it, expect, beforeEach, afterEach, vi } from 'vitest';
import { QueueStorage, QueuedMessage } from './queueStorage';

// Mock sessionStorage
const mockSessionStorage = {
  getItem: vi.fn(),
  setItem: vi.fn(),
  removeItem: vi.fn(),
  clear: vi.fn(),
  length: 0,
  key: vi.fn(),
};

Object.defineProperty(window, 'sessionStorage', {
  value: mockSessionStorage,
  writable: true,
});

describe('QueueStorage', () => {
  beforeEach(() => {
    vi.clearAllMocks();
    vi.spyOn(console, 'error').mockImplementation(() => {});
  });

  afterEach(() => {
    vi.restoreAllMocks();
  });

  describe('loadQueue', () => {
    it('filters out expired messages', () => {
      const now = Date.now();
      const messages: QueuedMessage[] = [
        { id: '1', content: 'Fresh message', timestamp: now },
        { id: '2', content: 'Old message', timestamp: now - 25 * 60 * 60 * 1000 }, // 25 hours old
        { id: '3', content: 'Recent message', timestamp: now - 1000 },
      ];
      mockSessionStorage.getItem.mockReturnValue(JSON.stringify(messages));

      const result = QueueStorage.loadQueue();

      expect(result).toHaveLength(2);
      expect(result.map((m: QueuedMessage) => m.id)).toEqual(['1', '3']);
      // Should update storage without expired message
      expect(mockSessionStorage.setItem).toHaveBeenCalled();
    });

    it('limits queue to maximum size', () => {
      const messages: QueuedMessage[] = Array.from({ length: 60 }, (_, i) => ({
        id: `${i}`,
        content: `Message ${i}`,
        timestamp: Date.now(),
      }));
      mockSessionStorage.getItem.mockReturnValue(JSON.stringify(messages));

      const result = QueueStorage.loadQueue();

      expect(result).toHaveLength(50); // MAX_QUEUE_SIZE
      expect(result[0].id).toBe('0');
      expect(result[49].id).toBe('49');
    });
  });

  describe('saveQueue', () => {
    it('filters expired messages before saving', () => {
      const now = Date.now();
      const messages: QueuedMessage[] = [
        { id: '1', content: 'Fresh', timestamp: now },
        { id: '2', content: 'Expired', timestamp: now - 25 * 60 * 60 * 1000 },
      ];

      QueueStorage.saveQueue(messages);

      const savedData = JSON.parse(mockSessionStorage.setItem.mock.calls[0][1]);
      expect(savedData).toHaveLength(1);
      expect(savedData[0].id).toBe('1');
    });

    it('limits queue size before saving', () => {
      const messages: QueuedMessage[] = Array.from({ length: 60 }, (_, i) => ({
        id: `${i}`,
        content: `Message ${i}`,
        timestamp: Date.now(),
      }));

      QueueStorage.saveQueue(messages);

      const savedData = JSON.parse(mockSessionStorage.setItem.mock.calls[0][1]);
      expect(savedData).toHaveLength(50);
    });
  });

  describe('addMessage', () => {
    it('adds message to existing queue', () => {
      const existingMessages: QueuedMessage[] = [
        { id: '1', content: 'Existing', timestamp: Date.now() },
      ];
      mockSessionStorage.getItem.mockReturnValue(JSON.stringify(existingMessages));

      const newMessage: QueuedMessage = {
        id: '2',
        content: 'New message',
        timestamp: Date.now(),
      };

      QueueStorage.addMessage(newMessage);

      const savedData = JSON.parse(mockSessionStorage.setItem.mock.calls[0][1]);
      expect(savedData).toHaveLength(2);
      expect(savedData[1]).toEqual(newMessage);
    });

    it('creates new queue if none exists', () => {
      mockSessionStorage.getItem.mockReturnValue(null);

      const message: QueuedMessage = {
        id: '1',
        content: 'First message',
        timestamp: Date.now(),
      };

      QueueStorage.addMessage(message);

      const savedData = JSON.parse(mockSessionStorage.setItem.mock.calls[0][1]);
      expect(savedData).toHaveLength(1);
      expect(savedData[0]).toEqual(message);
    });
  });

  describe('removeMessage', () => {
    it('removes message by id', () => {
      const messages: QueuedMessage[] = [
        { id: '1', content: 'Message 1', timestamp: Date.now() },
        { id: '2', content: 'Message 2', timestamp: Date.now() },
        { id: '3', content: 'Message 3', timestamp: Date.now() },
      ];
      mockSessionStorage.getItem.mockReturnValue(JSON.stringify(messages));

      QueueStorage.removeMessage('2');

      const savedData = JSON.parse(mockSessionStorage.setItem.mock.calls[0][1]);
      expect(savedData).toHaveLength(2);
      expect(savedData.map((m: QueuedMessage) => m.id)).toEqual(['1', '3']);
    });

    it('does nothing if message not found', () => {
      const messages: QueuedMessage[] = [{ id: '1', content: 'Message 1', timestamp: Date.now() }];
      mockSessionStorage.getItem.mockReturnValue(JSON.stringify(messages));

      QueueStorage.removeMessage('999');

      const savedData = JSON.parse(mockSessionStorage.setItem.mock.calls[0][1]);
      expect(savedData).toHaveLength(1);
      expect(savedData[0].id).toBe('1');
    });

    it('clears storage when removing last message', () => {
      const messages: QueuedMessage[] = [
        { id: '1', content: 'Last message', timestamp: Date.now() },
      ];
      mockSessionStorage.getItem.mockReturnValue(JSON.stringify(messages));

      QueueStorage.removeMessage('1');

      expect(mockSessionStorage.removeItem).toHaveBeenCalledWith('goose-message-queue');
    });
  });

  describe('updateMessage', () => {
    it('updates message content and timestamp', () => {
      const originalTimestamp = Date.now() - 1000;
      const messages: QueuedMessage[] = [
        { id: '1', content: 'Old content', timestamp: originalTimestamp },
        { id: '2', content: 'Message 2', timestamp: originalTimestamp },
      ];
      mockSessionStorage.getItem.mockReturnValue(JSON.stringify(messages));

      const beforeUpdate = Date.now();
      QueueStorage.updateMessage('1', 'New content');
      const afterUpdate = Date.now();

      const savedData = JSON.parse(mockSessionStorage.setItem.mock.calls[0][1]);
      expect(savedData[0].content).toBe('New content');
      expect(savedData[0].timestamp).toBeGreaterThanOrEqual(beforeUpdate);
      expect(savedData[0].timestamp).toBeLessThanOrEqual(afterUpdate);
      expect(savedData[1]).toEqual(messages[1]); // Other message unchanged
    });
  });

  describe('reorderQueue', () => {
    it('saves reordered queue', () => {
      const reorderedMessages: QueuedMessage[] = [
        { id: '3', content: 'Message 3', timestamp: Date.now() },
        { id: '1', content: 'Message 1', timestamp: Date.now() },
        { id: '2', content: 'Message 2', timestamp: Date.now() },
      ];

      QueueStorage.reorderQueue(reorderedMessages);

      const savedData = JSON.parse(mockSessionStorage.setItem.mock.calls[0][1]);
      expect(savedData).toEqual(reorderedMessages);
    });

    it('applies same filtering rules as saveQueue', () => {
      const now = Date.now();
      const reorderedMessages: QueuedMessage[] = [
        { id: '1', content: 'Fresh', timestamp: now },
        { id: '2', content: 'Expired', timestamp: now - 25 * 60 * 60 * 1000 },
      ];

      QueueStorage.reorderQueue(reorderedMessages);

      const savedData = JSON.parse(mockSessionStorage.setItem.mock.calls[0][1]);
      expect(savedData).toHaveLength(1);
      expect(savedData[0].id).toBe('1');
    });
  });
});
