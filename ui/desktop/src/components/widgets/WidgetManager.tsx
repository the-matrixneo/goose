import React, { useState, useEffect } from 'react';
import QueuedTasksWidget from './QueuedTasksWidget';
import BitChatWidget from './BitChatWidget';

interface Position {
  x: number;
  y: number;
}

interface WidgetSettings {
  queuedTasksWidget: {
    enabled: boolean;
    position: Position;
  };
  bitChatWidget: {
    enabled: boolean;
    position: Position;
  };
}

const DEFAULT_SETTINGS: WidgetSettings = {
  queuedTasksWidget: {
    enabled: true,
    position: { x: 220, y: 20 },
  },
  bitChatWidget: {
    enabled: true,
    position: { x: 420, y: 20 },
  },
};

export const WidgetManager: React.FC = () => {
  const [settings, setSettings] = useState<WidgetSettings>(DEFAULT_SETTINGS);

  // Load settings from localStorage
  useEffect(() => {
    try {
      const saved = localStorage.getItem('goose-widget-settings');
      if (saved) {
        const parsed = JSON.parse(saved);
        setSettings({ ...DEFAULT_SETTINGS, ...parsed });
      }
    } catch (error) {
      console.warn('Failed to load widget settings:', error);
    }
  }, []);

  // Save settings to localStorage
  const saveSettings = (newSettings: WidgetSettings) => {
    setSettings(newSettings);
    try {
      localStorage.setItem('goose-widget-settings', JSON.stringify(newSettings));
    } catch (error) {
      console.warn('Failed to save widget settings:', error);
    }
  };

  const handleQueuedTasksPositionChange = (position: Position) => {
    saveSettings({
      ...settings,
      queuedTasksWidget: {
        ...settings.queuedTasksWidget,
        position,
      },
    });
  };

  const handleBitChatPositionChange = (position: Position) => {
    saveSettings({
      ...settings,
      bitChatWidget: {
        ...settings.bitChatWidget,
        position,
      },
    });
  };

  return (
    <>
      {settings.queuedTasksWidget.enabled && (
        <QueuedTasksWidget
          initialPosition={settings.queuedTasksWidget.position}
          onPositionChange={handleQueuedTasksPositionChange}
        />
      )}

      {settings.bitChatWidget.enabled && (
        <BitChatWidget
          initialPosition={settings.bitChatWidget.position}
          onPositionChange={handleBitChatPositionChange}
        />
      )}
    </>
  );
};

export default WidgetManager;
