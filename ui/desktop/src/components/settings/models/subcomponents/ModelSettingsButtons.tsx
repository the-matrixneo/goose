import { AddModelButton } from './AddModelButton';
import { Button } from '../../../ui/button';
import { Sliders } from 'lucide-react';
import type { View } from '../../../../App';

interface ConfigureModelButtonsProps {
  setView: (view: View) => void;
}

export default function ModelSettingsButtons({ setView }: ConfigureModelButtonsProps) {
  return (
    <div className="flex gap-2 pt-4">
      <AddModelButton setView={setView} />
      <Button
        className="flex items-center gap-2 justify-center"
        variant="secondary"
        onClick={() => {
          setView('ConfigureProviders');
        }}
      >
        <Sliders className="rotate-90" />
        Configure providers
      </Button>
    </div>
  );
}
