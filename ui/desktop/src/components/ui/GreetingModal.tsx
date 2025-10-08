import React from 'react';
import { BaseModal } from './BaseModal';

export function GreetingModal({
  isOpen,
  greeting,
  onDismiss,
}: {
  isOpen: boolean;
  greeting: string;
  onDismiss: () => void;
}) {
  return (
    <BaseModal
      isOpen={isOpen}
      title="Greeting from Goose"
      actions={
        <div className="flex justify-end gap-2 p-4 border-t border-gray-200 dark:border-gray-700">
          <button
            onClick={onDismiss}
            className="px-4 py-2 bg-blue-600 text-white rounded-lg hover:bg-blue-700 transition-colors"
          >
            Dismiss
          </button>
        </div>
      }
    >
      <div className="text-gray-700 dark:text-gray-300 text-lg text-center py-4">
        {greeting}
      </div>
    </BaseModal>
  );
}
