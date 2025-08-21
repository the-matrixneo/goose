import React, { useState, useEffect } from 'react';
import ClockWidget from './ClockWidget';
import QueuedTasksWidget from './QueuedTasksWidget';

interface Position {
  x: number;
  y: number;
}

interface WidgetSettings {
  clockWidget: {
    enabled: boolean;
    position: Position;
  };
  queuedTasksWidget: {
    enabled: boolean;
    position: Position;
  };
}

const DEFAULT_SETTINGS: WidgetSettings = {
  clockWidget: {
    enabled: true,
    position: { x: 20, y: 20 },
  },
  queuedTasksWidget: {
    enabled: true,
    position: { x: 220, y: 20 },
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

  const handleClockPositionChange = (position: Position) => {
    saveSettings({
      ...settings,
      clockWidget: {
        ...settings.clockWidget,
        position,
      },
    });
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

  return (
    <>
      {settings.clockWidget.enabled && (
        <ClockWidget
          initialPosition={settings.clockWidget.position}
          onPositionChange={handleClockPositionChange}
        />
      )}
      
      {settings.queuedTasksWidget.enabled && (
        <QueuedTasksWidget
          initialPosition={settings.queuedTasksWidget.position}
          onPositionChange={handleQueuedTasksPositionChange}
        />
      )}
    </>
  );
};

export default WidgetManager;
