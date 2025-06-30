import { useEffect, useState, useCallback } from 'react';
import { all_goose_modes, ModeSelectionItem } from './ModeSelectionItem';
import { View, ViewOptions } from '../../../App';
import { useConfig } from '../../ConfigContext';

interface ModeSectionProps {
  setView: (view: View, viewOptions?: ViewOptions) => void;
}

export const ModeSection = ({ setView }: ModeSectionProps) => {
  const [currentMode, setCurrentMode] = useState('auto');
  const { read, upsert } = useConfig();

  const handleModeChange = async (newMode: string) => {
    try {
      await upsert('GOOSE_MODE', newMode, false);
      setCurrentMode(newMode);
    } catch (error) {
      console.error('Error updating goose mode:', error);
      throw new Error(`Failed to store new goose mode: ${newMode}`);
    }
  };

  const fetchCurrentMode = useCallback(async () => {
    try {
      const mode = (await read('GOOSE_MODE', false)) as string;
      if (mode) {
        setCurrentMode(mode);
      }
    } catch (error) {
      console.error('Error fetching current mode:', error);
    }
  }, [read]);

  useEffect(() => {
    fetchCurrentMode();
  }, [fetchCurrentMode]);

  return (
    <div className="space-y-1">
      {all_goose_modes.map((mode) => (
        <ModeSelectionItem
          key={mode.key}
          mode={mode}
          currentMode={currentMode}
          showDescription={true}
          isApproveModeConfigure={false}
          parentView="settings"
          setView={setView}
          handleModeChange={handleModeChange}
        />
      ))}
    </div>
  );
};
