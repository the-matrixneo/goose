import React, { useEffect, useState, useCallback, useRef } from 'react';
import { Search, ChevronDown, Folder, Loader2 } from 'lucide-react';
import { fetchSessions, type Session } from '../sessions';
import { ScrollArea } from './ui/scroll-area';
import { Input } from './ui/input';
import {
  Sidebar,
  SidebarContent,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuItem,
  SidebarGroup,
  SidebarGroupLabel,
  SidebarGroupContent,
  SidebarMenuButton,
} from './ui/sidebar';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from './ui/collapsible';
import { cn } from '../utils';
import { useTextAnimator } from '../hooks/use-text-animator';
import { Button } from './ui/button';
import { ChatSmart } from './icons';
import { Separator } from './ui/separator';

interface SidebarProps {
  onSelectSession: (sessionId: string) => void;
  refreshTrigger?: number;
  children?: React.ReactNode;
}

interface GroupedSessions {
  today: Session[];
  yesterday: Session[];
  older: { [key: string]: Session[] };
}

// Sessions Section Component
const SessionsSection: React.FC<{
  onSelectSession: (sessionId: string) => void;
  refreshTrigger?: number;
}> = ({ onSelectSession, refreshTrigger }) => {
  const [sessions, setSessions] = useState<Session[]>([]);
  const [searchTerm, setSearchTerm] = useState('');
  const [groupedSessions, setGroupedSessions] = useState<GroupedSessions>({
    today: [],
    yesterday: [],
    older: {},
  });
  const [sessionsWithDescriptions, setSessionsWithDescriptions] = useState<Set<string>>(new Set());

  const refreshTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Debounced refresh function
  const debouncedRefresh = useCallback(() => {
    console.log('SessionsSection: Debounced refresh triggered');
    // Clear any existing timeout
    if (refreshTimeoutRef.current) {
      clearTimeout(refreshTimeoutRef.current);
    }

    // Set new timeout - reduced to 200ms for faster response
    refreshTimeoutRef.current = setTimeout(() => {
      console.log('SessionsSection: Executing debounced refresh');
      loadSessions();
      refreshTimeoutRef.current = null;
    }, 200);
  }, []);

  // Cleanup timeout on unmount
  useEffect(() => {
    return () => {
      if (refreshTimeoutRef.current) {
        clearTimeout(refreshTimeoutRef.current);
      }
    };
  }, []);

  useEffect(() => {
    console.log('SessionsSection: Initial load');
    loadSessions();
  }, []);

  // Add effect to refresh sessions when refreshTrigger changes
  useEffect(() => {
    if (refreshTrigger) {
      console.log('SessionsSection: Refresh trigger changed, triggering refresh');
      debouncedRefresh();
    }
  }, [refreshTrigger, debouncedRefresh]);

  // Add effect to listen for session creation events
  useEffect(() => {
    const handleSessionCreated = () => {
      console.log('SessionsSection: Session created event received');
      debouncedRefresh();
    };

    const handleMessageStreamFinish = () => {
      console.log('SessionsSection: Message stream finished event received');
      // Always refresh when message stream finishes
      debouncedRefresh();
    };

    // Listen for custom events that indicate a session was created
    window.addEventListener('session-created', handleSessionCreated);

    // Also listen for message stream finish events
    window.addEventListener('message-stream-finished', handleMessageStreamFinish);

    return () => {
      window.removeEventListener('session-created', handleSessionCreated);
      window.removeEventListener('message-stream-finished', handleMessageStreamFinish);
    };
  }, [debouncedRefresh]);

  useEffect(() => {
    if (searchTerm) {
      const filtered = sessions.filter((session) =>
        (session.metadata.description || session.id)
          .toLowerCase()
          .includes(searchTerm.toLowerCase())
      );
      groupSessions(filtered);
    } else {
      groupSessions(sessions);
    }
  }, [searchTerm, sessions]);

  const groupSessions = (sessionsToGroup: Session[]) => {
    const now = new Date();
    const today = new Date(now.getFullYear(), now.getMonth(), now.getDate());
    const yesterday = new Date(today);
    yesterday.setDate(yesterday.getDate() - 1);

    const grouped: GroupedSessions = {
      today: [],
      yesterday: [],
      older: {},
    };

    sessionsToGroup.forEach((session) => {
      const sessionDate = new Date(session.modified);
      const sessionDateOnly = new Date(
        sessionDate.getFullYear(),
        sessionDate.getMonth(),
        sessionDate.getDate()
      );

      if (sessionDateOnly.getTime() === today.getTime()) {
        grouped.today.push(session);
      } else if (sessionDateOnly.getTime() === yesterday.getTime()) {
        grouped.yesterday.push(session);
      } else {
        const dateKey = sessionDateOnly.toISOString().split('T')[0];
        if (!grouped.older[dateKey]) {
          grouped.older[dateKey] = [];
        }
        grouped.older[dateKey].push(session);
      }
    });

    // Sort older sessions by date (newest first)
    const sortedOlder: { [key: string]: Session[] } = {};
    Object.keys(grouped.older)
      .sort()
      .reverse()
      .forEach((key) => {
        sortedOlder[key] = grouped.older[key];
      });

    grouped.older = sortedOlder;
    setGroupedSessions(grouped);
  };

  const loadSessions = async () => {
    try {
      const sessions = await fetchSessions();
      setSessions(sessions);
      groupSessions(sessions);
    } catch (err) {
      console.error('Failed to load sessions:', err);
      setSessions([]);
      setGroupedSessions({ today: [], yesterday: [], older: {} });
    }
  };

  // Component for individual session items with loading and animation states
  const SessionItem = ({ session }: { session: Session }) => {
    const hasDescription =
      session.metadata.description && session.metadata.description.trim() !== '';
    const isNewSession = session.id.match(/^\d{8}_\d{6}$/);
    const messageCount = session.metadata.message_count || 0;
    // Show loading for new sessions with few messages and no description
    // Only show loading for sessions created in the last 5 minutes
    const sessionDate = new Date(session.modified);
    const fiveMinutesAgo = new Date(Date.now() - 5 * 60 * 1000);
    const isRecentSession = sessionDate > fiveMinutesAgo;
    const shouldShowLoading =
      !hasDescription && isNewSession && messageCount <= 2 && isRecentSession;
    const [isAnimating, setIsAnimating] = useState(false);

    // Use text animator only for sessions that need animation
    const descriptionRef = useTextAnimator({
      text: isAnimating ? session.metadata.description : '',
    });

    // Track when description becomes available and trigger animation
    useEffect(() => {
      if (hasDescription && !sessionsWithDescriptions.has(session.id)) {
        setSessionsWithDescriptions((prev) => new Set(prev).add(session.id));

        // Only animate for new sessions that were showing loading
        if (shouldShowLoading) {
          setIsAnimating(true);
        }
      }
    }, [hasDescription, session.id, sessionsWithDescriptions, shouldShowLoading]);

    const handleClick = () => {
      console.log('SessionItem: Clicked on session:', session.id);
      onSelectSession(session.id);
    };

    return (
      <SidebarMenuItem key={session.id}>
        <SidebarMenuButton
          onClick={handleClick}
          className="cursor-pointer w-56 transition-all duration-150 hover:bg-background-medium rounded-xl text-text-muted hover:text-text-default h-fit flex items-start"
        >
          <div className="flex flex-col">
            <div className="text-sm w-48 truncate mb-1 px-1 text-ellipsis text-text-default flex items-center gap-2">
              {shouldShowLoading ? (
                <div className="flex items-center gap-2 animate-in fade-in duration-200">
                  <Loader2 className="size-3 animate-spin text-text-default" />
                  <span className="text-text-default animate-pulse">Generating description...</span>
                </div>
              ) : (
                <span
                  ref={isAnimating ? descriptionRef : undefined}
                  className={isAnimating ? 'animate-in fade-in duration-300' : ''}
                >
                  {hasDescription ? session.metadata.description : `Session ${session.id}`}
                </span>
              )}
            </div>
            <div className="text-xs w-48 truncate px-1 flex items-center gap-2 text-ellipsis">
              <Folder className="size-4" />
              {session.metadata.working_dir}
            </div>
          </div>
        </SidebarMenuButton>
      </SidebarMenuItem>
    );
  };

  const renderSessionGroup = (sessions: Session[], title: string, index: number) => {
    if (sessions.length === 0) return null;

    const isFirstTwoGroups = index < 2;

    return (
      <Collapsible defaultOpen={isFirstTwoGroups} className="group/collapsible">
        <SidebarGroup>
          <CollapsibleTrigger className="w-full">
            <SidebarGroupLabel className="flex cursor-pointer items-center justify-between text-text-default hover:text-text-default h-12 pl-3">
              <div className="flex min-w-0 items-center">
                <span className="opacity-100 transition-all duration-200 text-xs">{title}</span>
              </div>
              <ChevronDown className="size-4 text-text-muted flex-shrink-0 opacity-100 transition-all duration-200 group-data-[state=open]/collapsible:rotate-180" />
            </SidebarGroupLabel>
          </CollapsibleTrigger>
          <CollapsibleContent className="data-[state=open]:animate-collapsible-down data-[state=closed]:animate-collapsible-up overflow-hidden transition-all duration-200">
            <SidebarGroupContent>
              <SidebarMenu className="mb-2">
                {sessions.map((session) => (
                  <SessionItem key={session.id} session={session} />
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </CollapsibleContent>
        </SidebarGroup>
      </Collapsible>
    );
  };

  return (
    <Collapsible defaultOpen={false} className="group/collapsible rounded-xl">
      <SidebarGroup className="px-1">
        <CollapsibleTrigger className="w-full">
          <SidebarGroupLabel className="flex cursor-pointer items-center justify-between text-text-default px-4">
            <div className="flex min-w-0 items-center">
              <span className="opacity-100 transition-all duration-200 text-sm">Past sessions</span>
            </div>
            <ChevronDown className="size-4 text-text-muted flex-shrink-0 opacity-100 transition-all duration-200 group-data-[state=open]/collapsible:rotate-180" />
          </SidebarGroupLabel>
        </CollapsibleTrigger>
        <CollapsibleContent className="data-[state=open]:animate-collapsible-down data-[state=closed]:animate-collapsible-up overflow-hidden transition-all duration-200">
          <SidebarGroupContent>
            {/* Search Input */}
            <div className="p-1 pb-2">
              <div className="relative flex flex-row items-center gap-2">
                <Search className="absolute top-2.5 left-2.5 size-4 text-muted-foreground" />
                <Input
                  type="search"
                  placeholder="Search sessions..."
                  className="pl-8"
                  value={searchTerm}
                  onChange={(e: React.ChangeEvent<HTMLInputElement>) =>
                    setSearchTerm(e.target.value)
                  }
                />
              </div>
            </div>

            {/* Sessions Groups */}
            {(() => {
              let groupIndex = 0;
              const groups = [
                { sessions: groupedSessions.today, title: 'Today' },
                { sessions: groupedSessions.yesterday, title: 'Yesterday' },
                ...Object.entries(groupedSessions.older).map(([date, sessions]) => ({
                  sessions,
                  title: new Date(date).toLocaleDateString('en-US', {
                    weekday: 'long',
                    year: 'numeric',
                    month: 'long',
                    day: 'numeric',
                  }),
                })),
              ];

              return groups.map(({ sessions, title }) => {
                if (sessions.length === 0) return null;
                const currentIndex = groupIndex++;
                return renderSessionGroup(sessions, title, currentIndex);
              });
            })()}
          </SidebarGroupContent>
        </CollapsibleContent>
      </SidebarGroup>
    </Collapsible>
  );
};

// Main Sidebar Component
const AppSidebar: React.FC<SidebarProps> = ({ onSelectSession, refreshTrigger, children }) => {
  const [isVisible, setIsVisible] = useState(false);

  useEffect(() => {
    // Trigger animation after a small delay
    const timer = setTimeout(() => {
      setIsVisible(true);
    }, 100);
    return () => clearTimeout(timer);
  }, []);

  return (
    <Sidebar
      collapsible="offcanvas"
      variant="inset"
      side="left"
      className={cn(
        'transition-all duration-500 ease-out',
        isVisible ? 'translate-x-0 opacity-100' : '-translate-x-full opacity-0'
      )}
    >
      <SidebarHeader>
        <div className="p-1 pt-12 opacity-100 transition-all duration-200">
          {/* Header content can be customized here */}
        </div>
      </SidebarHeader>
      <SidebarContent>
        <ScrollArea className="h-full">
          {/* Action Buttons */}
          <div className="px-1 py-0">
            <Button
              onClick={() => {
                window.electron.createChatWindow(
                  undefined,
                  window.appConfig.get('GOOSE_WORKING_DIR') as string | undefined
                );
              }}
              className="w-full justify-start rounded-lg h-fit hover:shadow-default hover:bg-background-default transition-all duration-200"
              variant="ghost"
            >
              <div className="flex flex-col gap-1 items-start">
                <div className="flex gap-2 items-center text-text-default">
                  <ChatSmart className="w-4 h-4" />
                  New session
                </div>
                <span className="text-xs font-regular text-text-muted">
                  In the current directory
                </span>
              </div>
            </Button>

            <Button
              onClick={() => {
                window.electron.directoryChooser();
              }}
              className="w-full justify-start rounded-lg h-fit hover:shadow-default hover:bg-background-default transition-all duration-200"
              variant="ghost"
            >
              <div className="flex flex-col gap-1 items-start">
                <div className="flex gap-2 items-center text-text-default">
                  <Folder className="w-4 h-4" />
                  Open directory
                </div>
                <span className="text-xs font-regular text-text-muted">Select a directory</span>
              </div>
            </Button>
          </div>

          <div className="px-4 my-4">
            <Separator />
          </div>

          {/* Sessions Section */}
          <SessionsSection onSelectSession={onSelectSession} refreshTrigger={refreshTrigger} />

          {/* Additional sidebar items can be added here */}
          {children}
        </ScrollArea>
      </SidebarContent>
    </Sidebar>
  );
};

// Export the main component and the sessions section for flexibility
export { AppSidebar as default, SessionsSection };
