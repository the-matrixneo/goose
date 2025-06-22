import React, { useEffect, useState } from 'react';
import { Search, ChevronDown, Folder } from 'lucide-react';
import { fetchSessions, type Session } from '../../sessions';
import { ScrollArea } from '../ui/scroll-area';
import { Input } from '../ui/input';
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
} from '../ui/sidebar';
import { Collapsible, CollapsibleContent, CollapsibleTrigger } from '../ui/collapsible';
import { cn } from '../../utils';

interface SessionsSidebarProps {
  onSelectSession: (sessionId: string) => void;
  currentSession?: {
    id: string;
    title: string;
    messageHistoryIndex: number;
    messages: any[];
  };
}

interface GroupedSessions {
  today: Session[];
  yesterday: Session[];
  older: { [key: string]: Session[] };
}

const SessionsSidebar: React.FC<SessionsSidebarProps> = ({ onSelectSession, currentSession }) => {
  const [sessions, setSessions] = useState<Session[]>([]);
  // @ts-expect-error
  const [isLoading, setIsLoading] = useState(true);
  // @ts-expect-error
  const [error, setError] = useState<string | null>(null);
  const [searchTerm, setSearchTerm] = useState('');
  // @ts-expect-error
  const [filteredSessions, setFilteredSessions] = useState<Session[]>([]);
  // @ts-expect-error
  const [isTooltipOpen, setIsTooltipOpen] = useState(false);
  const [isVisible, setIsVisible] = useState(false);
  const [groupedSessions, setGroupedSessions] = useState<GroupedSessions>({
    today: [],
    yesterday: [],
    older: {},
  });

  useEffect(() => {
    loadSessions();
    // Trigger animation after a small delay
    const timer = setTimeout(() => {
      setIsVisible(true);
    }, 100);
    return () => clearTimeout(timer);
  }, []);

  useEffect(() => {
    if (searchTerm) {
      const filtered = sessions.filter((session) =>
        (session.metadata.description || session.id)
          .toLowerCase()
          .includes(searchTerm.toLowerCase())
      );
      setFilteredSessions(filtered);
      groupSessions(filtered);
    } else {
      setFilteredSessions(sessions);
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
    setIsLoading(true);
    setError(null);
    try {
      const sessions = await fetchSessions();
      setSessions(sessions);
      setFilteredSessions(sessions);
      groupSessions(sessions);
    } catch (err) {
      console.error('Failed to load sessions:', err);
      setError('Failed to load sessions. Please try again later.');
      setSessions([]);
      setFilteredSessions([]);
      setGroupedSessions({ today: [], yesterday: [], older: {} });
    } finally {
      setIsLoading(false);
    }
  };

  const renderSessionGroup = (sessions: Session[], title: string, index: number) => {
    if (sessions.length === 0) return null;

    const isFirstTwoGroups = index < 2;

    return (
      <Collapsible defaultOpen={isFirstTwoGroups} className="group/collapsible">
        <SidebarGroup>
          <CollapsibleTrigger className="w-full">
            <SidebarGroupLabel className="flex cursor-pointer items-center justify-between text-text-muted hover:text-text-default h-12">
              <div className="flex min-w-0 items-center">
                <span className="opacity-100 transition-all duration-200 text-xs">{title}</span>
              </div>
              <ChevronDown className="size-4 text-text-muted flex-shrink-0 opacity-100 transition-all duration-200 group-data-[state=open]/collapsible:rotate-180" />
            </SidebarGroupLabel>
          </CollapsibleTrigger>
          <CollapsibleContent className="data-[state=open]:animate-collapsible-down data-[state=closed]:animate-collapsible-up overflow-hidden transition-all duration-200">
            <SidebarGroupContent>
              <SidebarMenu className="mb-2 ml-2 pl-2 border-l border-border-default">
                {sessions.map((session) => (
                  <SidebarMenuItem key={session.id}>
                    <SidebarMenuButton
                      asChild
                      onClick={() => onSelectSession(session.id)}
                      className=" cursor-pointer w-52 transition-all duration-150 hover:bg-background-muted rounded-2xl text-text-muted hover:text-text-default h-fit flex items-start"
                    >
                      <div className="flex flex-col">
                        <div className="text-sm w-48 px-2 truncate -mb-1 text-ellipsis">
                          {session.metadata.description || session.id}
                        </div>
                        <div className="text-xs w-48 px-2 truncate flex items-center gap-2 text-ellipsis">
                          <Folder className="size-4" />
                          {session.metadata.working_dir}
                        </div>
                      </div>
                    </SidebarMenuButton>
                  </SidebarMenuItem>
                ))}
              </SidebarMenu>
            </SidebarGroupContent>
          </CollapsibleContent>
        </SidebarGroup>
      </Collapsible>
    );
  };

  const renderCurrentSession = () => {
    if (!currentSession || currentSession.messages.length === 0) return null;

    return (
      <SidebarGroup>
        <SidebarGroupLabel className="flex items-center h-12">
          <span className="opacity-100 transition-all duration-200 text-xs text-text-muted">
            Current Session
          </span>
        </SidebarGroupLabel>
        <SidebarGroupContent>
          <SidebarMenu className="mb-2 ml-2 pl-2 border-l border-border-default">
            <SidebarMenuItem>
              <SidebarMenuButton
                asChild
                className="cursor-pointer w-52 transition-all duration-150 bg-background-muted rounded-2xl text-text-default h-fit flex items-start"
              >
                <div className="flex flex-col">
                  <div className="text-sm w-48 px-2 truncate -mb-1 text-ellipsis">
                    {currentSession.title || currentSession.id}
                  </div>
                  <div className="text-xs w-48 px-2 truncate flex items-center gap-2 text-ellipsis">
                    <Folder className="size-4" />
                    {window.appConfig.get('GOOSE_WORKING_DIR') as string}
                  </div>
                </div>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    );
  };

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
          <div className="relative flex flex-row items-center gap-2">
            <Search className="absolute top-2.5 left-2.5 size-4 text-muted-foreground" />
            <Input
              type="search"
              placeholder="Search sessions..."
              className="pl-8"
              value={searchTerm}
              onChange={(e) => setSearchTerm(e.target.value)}
            />
          </div>

          {/* <div className="mt-4">
            <div className="text-xs text-text-muted mb-2">Current</div>
            <TooltipProvider>
              <Tooltip open={isTooltipOpen} onOpenChange={setIsTooltipOpen}>
                <TooltipTrigger asChild>
                  <Button
                    className="w-full justify-start"
                    variant="ghost"
                    size="sm"
                    onClick={async () => {
                      window.electron.directoryChooser();
                    }}
                  >
                    <Folder className="mr-1 size-4" />
                    <div className="max-w-[200px] truncate [direction:rtl]">
                      {String(window.appConfig.get('GOOSE_WORKING_DIR'))}
                    </div>
                  </Button>
                </TooltipTrigger>
                <TooltipContent className="max-w-96 overflow-auto scrollbar-thin" side="right">
                  {window.appConfig.get('GOOSE_WORKING_DIR') as string}
                </TooltipContent>
              </Tooltip>
            </TooltipProvider>
          </div> */}
        </div>
      </SidebarHeader>
      <SidebarContent>
        <ScrollArea className="h-full">
          {renderCurrentSession()}
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
        </ScrollArea>
      </SidebarContent>
    </Sidebar>
  );
};

export default SessionsSidebar;
