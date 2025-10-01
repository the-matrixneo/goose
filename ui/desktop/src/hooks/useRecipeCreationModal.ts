import { useEffect, useState, useRef } from 'react';

/**
 * Hook for managing recipe creation modal state and global event handling
 * Extracted from useRecipeManager to separate UI concerns from business logic
 */
export function useRecipeCreationModal(sessionId: string) {
  const [isCreateRecipeModalOpen, setIsCreateRecipeModalOpen] = useState(false);
  const isCreatingRecipeRef = useRef(false);

  // Global event listener for recipe creation triggered from other parts of the app
  useEffect(() => {
    const handleMakeAgent = async () => {
      if (window.isCreatingRecipe) {
        return;
      }

      if (isCreatingRecipeRef.current) {
        return;
      }

      setIsCreateRecipeModalOpen(true);
    };

    window.addEventListener('make-agent-from-chat', handleMakeAgent);

    return () => {
      window.removeEventListener('make-agent-from-chat', handleMakeAgent);
    };
  }, [sessionId]);

  return {
    isCreateRecipeModalOpen,
    setIsCreateRecipeModalOpen,
    isCreatingRecipeRef,
  };
}
