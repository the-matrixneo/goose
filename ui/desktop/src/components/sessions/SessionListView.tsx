import React, { useCallback, useEffect, useState, useRef } from 'react';
import { Session, fetchSessions, updateSessionMetadata } from '../../sessions';
import { ScrollArea } from '../ui/scroll-area';
import { MainPanelLayout } from '../Layout/MainPanelLayout';
import { Button } from '../ui/button';
import { Input } from '../ui/input';
import { Calendar, Folder, MessageSquareText, MoreHorizontal, AlertCircle, Target, Search, X } from 'lucide-react';
import { formatMessageTimestamp } from '../../utils/timeUtils';
import { StaggeredSessionItem } from './StaggeredSessionItem';

interface DateGroup {
  label: string;
  sessions: Session[];
}

interface SearchViewProps {
  onSearch: (query: string) => void;
}

const SearchView: React.FC<SearchViewProps> = ({ onSearch }) => {
  const [query, setQuery] = useState('');
  const inputRef = useRef<HTMLInputElement>(null);

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    onSearch(query);
  };

  const handleClear = () => {
    setQuery('');
    onSearch('');
    inputRef.current?.focus();
  };

  return (
    <form onSubmit={handleSubmit} className="flex items-center gap-2 mb-6">
      <div className="relative flex-1">
        <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-text-muted" />
        <Input
          ref={inputRef}
          type="text"
          placeholder="Search sessions..."
          className="pl-10 pr-10 py-2 h-10 bg-background-muted/30 border-borderSubtle"
          value={query}
          onChange={(e) => setQuery(e.target.value)}
        />
        {query && (
          <button
            type="button"
            onClick={handleClear}
            className="absolute right-3 top-1/2 transform -translate-y-1/2 text-text-muted hover:text-text-standard"
          >
            <X className="h-4 w-4" />
          </button>
        )}
      </div>
      <Button type="submit" variant="default" size="sm">
        Search
      </Button>
    </form>
  );
};

export default function SessionListView({
  onSessionSelect,
}: {
  onSessionSelect?: (sessionId: string) => void;
}) {
  const [sessions, setSessions] = useState<Session[]>([]);
  const [dateGroups, setDateGroups] = useState<DateGroup[]>([]);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [editingSession, setEditingSession] = useState<Session | null>(null);
  const [showEditModal, setShowEditModal] = useState(false);
  const [newDescription, setNewDescription] = useState('');
  const [searchResults, setSearchResults] = useState<Session[] | null>(null);
  const containerRef = useRef<HTMLDivElement>(null);

  // Load sessions on mount
  useEffect(() => {
    loadSessions();
  }, []);

  const loadSessions = async () => {
    setIsLoading(true);
    setError(null);

    try {
      // Use the fetchSessions function from sessions.ts instead of window.electron.getSessions
      const result = await fetchSessions();
      setSessions(result);
      groupSessionsByDate(result);
    } catch (err) {
      console.error('Failed to load sessions:', err);
      setError('Failed to load sessions. Please try again.');
    } finally {
      // Set loading to false after a short delay to ensure smooth transition
      setTimeout(() => {
        setIsLoading(false);
      }, 300);
    }
  };

  const groupSessionsByDate = (sessionsToGroup: Session[]) => {
    const today = new Date();
    today.setHours(0, 0, 0, 0);

    const yesterday = new Date(today);
    yesterday.setDate(yesterday.getDate() - 1);

    const lastWeek = new Date(today);
    lastWeek.setDate(lastWeek.getDate() - 7);

    const lastMonth = new Date(today);
    lastMonth.setMonth(lastMonth.getMonth() - 1);

    const groups: DateGroup[] = [
      { label: 'Today', sessions: [] },
      { label: 'Yesterday', sessions: [] },
      { label: 'This Week', sessions: [] },
      { label: 'This Month', sessions: [] },
      { label: 'Earlier', sessions: [] },
    ];

    sessionsToGroup.forEach((session) => {
      const sessionDate = new Date(session.modified);
      sessionDate.setHours(0, 0, 0, 0);

      if (sessionDate.getTime() === today.getTime()) {
        groups[0].sessions.push(session);
      } else if (sessionDate.getTime() === yesterday.getTime()) {
        groups[1].sessions.push(session);
      } else if (sessionDate > lastWeek) {
        groups[2].sessions.push(session);
      } else if (sessionDate > lastMonth) {
        groups[3].sessions.push(session);
      } else {
        groups[4].sessions.push(session);
      }
    });

    // Filter out empty groups
    const filteredGroups = groups.filter((group) => group.sessions.length > 0);
    setDateGroups(filteredGroups);
  };

  const handleSearch = (query: string) => {
    if (!query.trim()) {
      setSearchResults(null);
      groupSessionsByDate(sessions);
      return;
    }

    const lowerQuery = query.toLowerCase();
    const results = sessions.filter((session) => {
      const description = (session.metadata.description || session.id).toLowerCase();
      const workingDir = (session.metadata.working_dir || '').toLowerCase();
      
      return description.includes(lowerQuery) || workingDir.includes(lowerQuery);
    });

    setSearchResults(results);
    groupSessionsByDate(results);
  };

  const handleSaveDescription = useCallback(async () => {
    if (!editingSession) return;

    const sessionId = editingSession.id;
    try {
      // Use the updateSessionMetadata function from sessions.ts
      await updateSessionMetadata(sessionId, newDescription);
      setShowEditModal(false);
      setEditingSession(null);

      // Update session in state
      setSessions((prevSessions) =>
        prevSessions.map((s) =>
          s.id === sessionId ? { ...s, metadata: { ...s.metadata, description: newDescription } } : s
        )
      );
    } catch (error) {
      console.error('Failed to update session description:', error);
    }
  }, [editingSession, newDescription]);

  useEffect(() => {
    if (editingSession) {
      setNewDescription(editingSession.metadata.description || '');
    }
  }, [editingSession]);

  const handleCancelEdit = useCallback(() => {
    setShowEditModal(false);
    setEditingSession(null);
  }, []);

  const updateSessionDescription = useCallback((sessionId: string, newDescription: string) => {
    setSessions((prevSessions) =>
      prevSessions.map((s) =>
        s.id === sessionId ? { ...s, metadata: { ...s.metadata, description: newDescription } } : s
      )
    );
  }, []);

  const handleEditSession = useCallback((session: Session) => {
    setEditingSession(session);
    setShowEditModal(true);
  }, []);

  const renderContent = () => {
    if (error) {
      return (
        <div className="flex flex-col items-center justify-center h-full text-text-muted">
          <AlertCircle className="h-12 w-12 text-red-500 mb-4" />
          <p className="text-lg mb-2">Error Loading Sessions</p>
          <p className="text-sm text-center mb-4">{error}</p>
          <Button onClick={loadSessions} variant="default">
            Try Again
          </Button>
        </div>
      );
    }

    if (isLoading) {
      // Show a minimal loading state instead of skeletons
      return (
        <div className="flex flex-col items-center justify-center h-full text-text-muted">
          <div className="animate-pulse flex space-x-2 items-center">
            <div className="h-2 w-2 bg-text-muted rounded-full"></div>
            <div className="h-2 w-2 bg-text-muted rounded-full animation-delay-200"></div>
            <div className="h-2 w-2 bg-text-muted rounded-full animation-delay-400"></div>
          </div>
          <p className="mt-4 text-sm">Loading sessions...</p>
        </div>
      );
    }

    if (sessions.length === 0) {
      return (
        <div className="flex flex-col justify-center h-full text-text-muted">
          <MessageSquareText className="h-12 w-12 mb-4" />
          <p className="text-lg mb-2">No chat sessions found</p>
          <p className="text-sm">Your chat history will appear here</p>
        </div>
      );
    }

    if (dateGroups.length === 0 && searchResults !== null) {
      return (
        <div className="flex flex-col items-center justify-center h-full text-text-muted mt-4">
          <MessageSquareText className="h-12 w-12 mb-4" />
          <p className="text-lg mb-2">No matching sessions found</p>
          <p className="text-sm">Try adjusting your search terms</p>
        </div>
      );
    }

    // For regular rendering in grid layout with staggered animation
    return (
      <div className="space-y-8">
        {dateGroups.map((group, groupIndex) => (
          <div key={group.label} className="space-y-4">
            <div className="sticky top-0 z-10 bg-background-default/80 backdrop-blur-md py-2">
              <h2 className="text-text-muted">{group.label}</h2>
            </div>
            <div className="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 2xl:grid-cols-5 gap-4">
              {group.sessions.map((session, index) => (
                <StaggeredSessionItem 
                  key={session.id} 
                  session={session} 
                  onEditClick={handleEditSession}
                  index={index}
                  groupIndex={groupIndex} 
                />
              ))}
            </div>
          </div>
        ))}
      </div>
    );
  };

  // Custom main layout props to override background completely
  const customMainLayoutProps = {
    backgroundColor: 'transparent', // Force transparent background with inline style
  };

  return (
    <>
      <MainPanelLayout {...customMainLayoutProps}>
        <div className="flex-1 flex flex-col min-h-0">
          <div className="bg-background-default/80 backdrop-blur-md px-8 pb-8 pt-16">
            <div className="flex flex-col page-transition">
              <div className="flex justify-between items-center mb-1">
                <h1 className="text-4xl font-light">Chat history</h1>
              </div>
              <p className="text-sm text-text-muted mb-4">
                View and search your past conversations with Goose.
              </p>
            </div>
          </div>

          <div className="flex-1 min-h-0 relative px-8">
            <ScrollArea className="h-full" data-search-scroll-area>
              <div ref={containerRef} className="h-full relative">
                <SearchView onSearch={handleSearch} />
                {renderContent()}
              </div>
            </ScrollArea>
          </div>
        </div>
      </MainPanelLayout>

      {/* Edit Session Modal */}
      {showEditModal && editingSession && (
        <div className="fixed inset-0 z-[300] flex items-center justify-center bg-black/50">
          <div className="bg-background-default border border-borderSubtle rounded-lg p-6 w-96 max-w-[90vw]">
            <h3 className="text-lg font-medium text-text-standard mb-4">Edit Session Description</h3>
            <Input
              value={newDescription}
              onChange={(e) => setNewDescription(e.target.value)}
              placeholder="Enter a description for this session"
              className="mb-6"
              autoFocus
            />
            <div className="flex justify-end space-x-2">
              <Button variant="outline" onClick={handleCancelEdit}>
                Cancel
              </Button>
              <Button onClick={handleSaveDescription}>Save</Button>
            </div>
          </div>
        </div>
      )}
    </>
  );
}
