import React, { useState, useEffect, useCallback } from 'react';
import SessionListView from './SessionListView';
import SessionHistoryView from './SessionHistoryView';
import { useLocation } from 'react-router-dom';
import { getSession, Session } from '../../api';

const SessionsView: React.FC = () => {
  const [selectedSession, setSelectedSession] = useState<Session | null>(null);
  const [showSessionHistory, setShowSessionHistory] = useState(false);
  const [isLoadingSession, setIsLoadingSession] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [initialSessionId, setInitialSessionId] = useState<string | null>(null);
  const location = useLocation();

  const loadSessionDetails = async (sessionId: string) => {
    setIsLoadingSession(true);
    setError(null);
    setShowSessionHistory(true);
    try {
      const response = await getSession<true>({
        path: { session_id: sessionId },
        throwOnError: true,
      });
      setSelectedSession(response.data);
    } catch (err) {
      console.error(`Failed to load session details for ${sessionId}:`, err);
      setError('Failed to load session details. Please try again later.');
      // Keep the selected session null if there's an error
      setSelectedSession(null);
      setShowSessionHistory(false);
    } finally {
      setIsLoadingSession(false);
      setInitialSessionId(null);
    }
  };

  const handleSelectSession = useCallback(async (sessionId: string) => {
    await loadSessionDetails(sessionId);
  }, []);

  // Check if a session ID was passed in the location state (from SessionsInsights)
  useEffect(() => {
    const state = location.state as { selectedSessionId?: string } | null;
    if (state?.selectedSessionId) {
      // Set immediate loading state to prevent flash of session list
      setIsLoadingSession(true);
      setInitialSessionId(state.selectedSessionId);
      handleSelectSession(state.selectedSessionId);
      // Clear the state to prevent reloading on navigation
      window.history.replaceState({}, document.title);
    }
  }, [location.state, handleSelectSession]);

  const handleBackToSessions = () => {
    setShowSessionHistory(false);
    setError(null);
  };

  const handleRetryLoadSession = () => {
    if (selectedSession) {
      loadSessionDetails(selectedSession.id);
    }
  };

  // If we're loading an initial session or have a selected showSessionHistory, show the session history view
  // Otherwise, show the sessions list view
  return (showSessionHistory && selectedSession) || (isLoadingSession && initialSessionId) ? (
    <SessionHistoryView
      session={
        selectedSession || {
          id: initialSessionId || '',
          conversation: [],
          description: 'Loading...',
          working_dir: '',
          message_count: 0,
          total_tokens: 0,
          created_at: '',
          updated_at: '',
          extension_data: {},
        }
      }
      isLoading={isLoadingSession}
      error={error}
      onBack={handleBackToSessions}
      onRetry={handleRetryLoadSession}
    />
  ) : (
    <SessionListView
      onSelectSession={handleSelectSession}
      selectedSessionId={selectedSession?.id ?? null}
    />
  );
};

export default SessionsView;
