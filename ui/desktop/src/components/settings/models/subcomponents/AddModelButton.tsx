import { useState } from 'react';
import { Button } from '../../../ui/button';
import { AddModelModal } from './AddModelModal';
import type { View } from '../../../../App';
import { ArrowLeftRight } from 'lucide-react';

interface AddModelButtonProps {
  setView: (view: View) => void;
}

export const AddModelButton = ({ setView }: AddModelButtonProps) => {
  const [isAddModelModalOpen, setIsAddModelModalOpen] = useState(false);

  return (
    <>
      <Button
        className="flex items-center gap-2 justify-center"
        variant="default"
        onClick={() => setIsAddModelModalOpen(true)}
      >
        <ArrowLeftRight />
        Switch models
      </Button>
      {isAddModelModalOpen ? (
        <AddModelModal setView={setView} onClose={() => setIsAddModelModalOpen(false)} />
      ) : null}
    </>
  );
};
