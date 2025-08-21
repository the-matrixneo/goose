import { useEffect, useState, useRef, useCallback } from 'react';
import { ChatType } from '../types/chat';
import { fetchSessionDetails, generateSessionId } from '../sessions';
import { View, ViewOptions } from '../App';
import { DEFAULT_CHAT_TITLE } from '../contexts/ChatContext';

type UseChatArgs = {
  setIsLoadingSession: (isLoading: boolean) => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  setPairChat?: (chat: ChatType) => void;
};

export const useChat = ({ setIsLoadingSession, setView, setPairChat }: UseChatArgs) => {
  const [chat, setChat] = useState<ChatType>({
    id: generateSessionId(),
    title: DEFAULT_CHAT_TITLE,
    messages: [],
    messageHistoryIndex: 0,
    recipeConfig: null, // Initialize with no recipe
  });

  // Refs to prevent memory leaks and race conditions
  const loadingTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  const hasInitializedRef = useRef(false);

  // Safe loading state management with timeout protection
  const safeSetLoadingState = useCallback((isLoading: boolean, timeoutMs: number = 5000) => {
    try {
      console.log('useChat: Setting loading state to:', isLoading);
      setIsLoadingSession(isLoading);
      
      if (isLoading) {
        // Clear any existing timeout
        if (loadingTimeoutRef.current) {
          clearTimeout(loadingTimeoutRef.current);
        }
        
        // Set a fail-safe timeout to prevent stuck loading states
        loadingTimeoutRef.current = setTimeout(() => {
          console.warn('useChat: Loading session timeout - forcing completion');
          setIsLoadingSession(false);
        }, timeoutMs);
      } else {
        // Clear timeout when loading completes
        if (loadingTimeoutRef.current) {
          clearTimeout(loadingTimeoutRef.current);
          loadingTimeoutRef.current = null;
        }
      }
    } catch (error) {
      console.error('useChat: Error setting loading state:', error);
      setIsLoadingSession(false);
    }
  }, [setIsLoadingSession]);

  // Cleanup function
  const cleanup = useCallback(() => {
    console.log('useChat: Cleaning up');
    if (loadingTimeoutRef.current) {
      clearTimeout(loadingTimeoutRef.current);
      loadingTimeoutRef.current = null;
    }
    safeSetLoadingState(false);
  }, [safeSetLoadingState]);

  // Check for resumeSessionId in URL parameters with robust error handling
  useEffect(() => {
    // Prevent multiple initializations
    if (hasInitializedRef.current) {
      console.log('useChat: Already initialized, skipping');
      return;
    }
    hasInitializedRef.current = true;

    const checkForResumeSession = async () => {
      try {
        console.log('useChat: Checking for resume session');
        const urlParams = new URLSearchParams(window.location.search);
        const resumeSessionId = urlParams.get('resumeSessionId');

        console.log('useChat: Resume session ID:', resumeSessionId);

        // If no resumeSessionId, just ensure loading is false and return
        if (!resumeSessionId) {
          console.log('useChat: No resume session ID, ensuring loading is false');
          safeSetLoadingState(false);
          return;
        }

        console.log('useChat: Processing resume session:', resumeSessionId);
        safeSetLoadingState(true);

        // Use Promise.race to add additional timeout protection
        const sessionPromise = fetchSessionDetails(resumeSessionId);
        const timeoutPromise = new Promise((_, reject) => {
          setTimeout(() => reject(new Error('Session fetch timeout')), 8000);
        });

        const sessionDetails = await Promise.race([sessionPromise, timeoutPromise]);

        // Validate session details
        if (!sessionDetails || typeof sessionDetails !== 'object') {
          throw new Error('Invalid session details received');
        }

        if (!sessionDetails.session_id) {
          throw new Error('Session ID missing from session details');
        }

        console.log('useChat: Session details loaded successfully:', sessionDetails.session_id);

        // Create session chat object
        const sessionChat: ChatType = {
          id: sessionDetails.session_id,
          title: sessionDetails.metadata?.description || `ID: ${sessionDetails.session_id}`,
          messages: Array.isArray(sessionDetails.messages) ? sessionDetails.messages : [],
          messageHistoryIndex: Array.isArray(sessionDetails.messages) ? sessionDetails.messages.length : 0,
          recipeConfig: null, // Sessions don't have recipes by default
        };

        // Update chat states
        setChat(sessionChat);

        // If we're setting the view to 'pair', also update the pairChat state
        if (setPairChat) {
          setPairChat(sessionChat);
        }

        // Navigate to pair view
        setView('pair');

        // Clear the resumeSessionId from URL to prevent reprocessing
        const newUrl = new URL(window.location.href);
        newUrl.searchParams.delete('resumeSessionId');
        window.history.replaceState({}, document.title, newUrl.toString());

      } catch (error) {
        console.error('useChat: Failed to resume session:', error);
        
        // Clear the resumeSessionId from URL even on error
        try {
          const newUrl = new URL(window.location.href);
          newUrl.searchParams.delete('resumeSessionId');
          window.history.replaceState({}, document.title, newUrl.toString());
        } catch (urlError) {
          console.error('useChat: Failed to clean URL:', urlError);
        }
      } finally {
        // Always clear the loading state
        console.log('useChat: Clearing loading state in finally block');
        safeSetLoadingState(false);
      }
    };

    // Use setTimeout to ensure this runs after the component is fully mounted
    const initTimeout = setTimeout(() => {
      checkForResumeSession().catch((error) => {
        console.error('useChat: Unexpected error in checkForResumeSession:', error);
        cleanup();
      });
    }, 100);

    // Cleanup on unmount
    return () => {
      clearTimeout(initTimeout);
      cleanup();
    };
  }, [setView, setPairChat, safeSetLoadingState, cleanup]);

  // Additional safety: Clear loading state on component unmount
  useEffect(() => {
    return cleanup;
  }, [cleanup]);

  return { chat, setChat };
};
