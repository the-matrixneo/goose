import React, { useState, useEffect } from 'react';
import BitChatWidget from './BitChatWidget';

interface Position {
  x: number;
  y: number;
}

interface WidgetSettings {
  bitChatWidget: {
    enabled: boolean;
    position: Position;
  };
}

const DEFAULT_SETTINGS: WidgetSettings = {
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
