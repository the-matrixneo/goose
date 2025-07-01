import { useEffect, useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../ui/card';
import {
  FolderOpen,
  Calendar,
  MessageSquareText,
  Target,
  ChevronRight,
  Folder,
} from 'lucide-react';
import { getApiUrl, getSecretKey } from '../../config';
import { useTextAnimator } from '../../hooks/use-text-animator';
import { Greeting } from '../common/Greeting';
import { ActivityHeatmap } from '../common/ActivityHeatmap';
import { fetchSessions, type Session } from '../../sessions';
import { fetchProjects, type ProjectMetadata } from '../../projects';
import { useNavigate } from 'react-router-dom';
import { formatMessageTimestamp } from '../../utils/timeUtils';
import { Button } from '../ui/button';
import { ChatSmart } from '../icons/';

interface SessionInsights {
  totalSessions: number;
  mostActiveDirs: [string, number][];
  avgSessionDuration: number;
  totalTokens: number;
}

export function SessionInsights() {
  const [insights, setInsights] = useState<SessionInsights | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [recentSessions, setRecentSessions] = useState<Session[]>([]);
  const [recentProjects, setRecentProjects] = useState<ProjectMetadata[]>([]);
  const navigate = useNavigate();

  // Add text animator effects for each number
  const totalSessionsRef = useTextAnimator({ text: insights?.totalSessions.toString() ?? '' });
  const avgDurationRef = useTextAnimator({
    text: insights?.avgSessionDuration ? insights.avgSessionDuration.toFixed(1) : '',
  });
  const totalTokensRef = useTextAnimator({
    text: insights?.totalTokens ? (insights.totalTokens / 1000000).toFixed(2) : '',
  });

  useEffect(() => {
    const loadInsights = async () => {
      try {
        const response = await fetch(getApiUrl('/sessions/insights'), {
          headers: {
            Accept: 'application/json',
            'Content-Type': 'application/json',
            'X-Secret-Key': getSecretKey(),
          },
        });

        if (!response.ok) {
          const errorText = await response.text();
          throw new Error(`Failed to fetch insights: ${response.status} ${errorText}`);
        }

        const data = await response.json();
        setInsights(data);
      } catch (error) {
        setError(error instanceof Error ? error.message : 'Failed to load insights');
      }
    };

    const loadRecentSessions = async () => {
      try {
        const sessions = await fetchSessions();
        setRecentSessions(sessions.slice(0, 3));
      } catch (error) {
        console.error('Failed to load recent sessions:', error);
      }
    };

    const loadRecentProjects = async () => {
      try {
        const projects = await fetchProjects();
        setRecentProjects(projects.slice(0, 3));
      } catch (error) {
        console.error('Failed to load recent projects:', error);
      }
    };

    loadInsights();
    loadRecentSessions();
    loadRecentProjects();
  }, []);

  const handleSessionClick = (sessionId: string) => {
    navigate('/sessions', {
      state: { selectedSessionId: sessionId },
      replace: true,
    });
  };

  const navigateToSessionHistory = () => {
    navigate('/sessions');
  };

  const navigateToProjects = () => {
    navigate('/projects');
  };

  const handleProjectClick = (projectId: string) => {
    navigate('/projects', {
      state: { selectedProjectId: projectId },
      replace: true,
    });
  };

  // Format date to show only the date part (without time)
  const formatDateOnly = (dateStr: string) => {
    const date = new Date(dateStr);
    return date
      .toLocaleDateString('en-US', { month: '2-digit', day: '2-digit', year: 'numeric' })
      .replace(/\//g, '/');
  };

  if (!insights) {
    return <></>;
  }

  return (
    <>
      <Greeting />

      <div className="grid gap-4">
        {/* Top row with three equal columns */}
        <div className="grid grid-cols-3 gap-4">
          {/* Total Sessions Card */}
          <Card className="w-full p-3 px-4 animate-in fade-in slide-in-from-right-8 duration-500">
            <CardContent className="flex flex-col justify-end h-full px-0">
              <div className="flex flex-col justify-end">
                <p className="text-4xl font-mono font-light flex items-end">
                  {insights?.totalSessions}
                </p>
                <span className="text-xs text-text-muted">Total sessions</span>
              </div>
            </CardContent>
          </Card>

          {/* Average Duration Card */}
          <Card className="w-full p-3 px-4 animate-in fade-in slide-in-from-right-8 duration-500">
            <CardContent className="flex flex-col justify-end h-full px-0">
              <div className="flex flex-col justify-end">
                <p className="text-4xl font-mono font-light flex items-end">
                  {insights?.avgSessionDuration?.toFixed(1)}m
                </p>
                <span className="text-xs text-text-muted">Avg. chat length</span>
              </div>
            </CardContent>
          </Card>

          {/* Total Tokens Card */}
          <Card className="w-full p-3 px-4 animate-in fade-in slide-in-from-right-8 duration-500">
            <CardContent className="flex flex-col justify-end h-full px-0">
              <div className="flex flex-col justify-end">
                <p className="text-4xl font-mono font-light flex items-end">
                  {insights?.totalTokens ? `${(insights.totalTokens / 1000000).toFixed(2)}M` : ''}
                </p>
                <span className="text-xs text-text-muted">Total tokens</span>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* Bottom row with two equal columns */}
        <div className="grid grid-cols-2 gap-4">
          {/* Recent Projects Card */}
          <Card className="w-full p-3 px-2 animate-in fade-in slide-in-from-right-8 duration-500">
            <CardContent className="px-0">
              <div className="flex justify-between items-center mb-4">
                <CardDescription className="mb-0 px-2">
                  <span className="text-lg text-text-default">Recent projects</span>
                </CardDescription>
                <Button
                  variant="ghost"
                  size="sm"
                  className="text-xs text-text-muted flex items-center gap-1 !px-0 hover:bg-transparent hover:underline hover:text-text-default"
                  onClick={navigateToProjects}
                >
                  See all
                </Button>
              </div>
              <div className="space-y-1">
                {recentProjects.length > 0 ? (
                  recentProjects.map((project) => (
                    <div
                      key={project.id}
                      className="flex items-center justify-between text-sm py-1 px-2 rounded-md hover:bg-background-muted cursor-pointer transition-colors"
                      onClick={() => handleProjectClick(project.id)}
                      role="button"
                      tabIndex={0}
                      onKeyDown={(e) => {
                        if (e.key === 'Enter' || e.key === ' ') {
                          handleProjectClick(project.id);
                        }
                      }}
                    >
                      <div className="flex items-center space-x-2">
                        <Folder className="h-4 w-4 text-text-muted" />
                        <span className="truncate max-w-[200px]">{project.name}</span>
                      </div>
                      <span className="text-text-muted font-mono font-light">
                        {formatDateOnly(project.updatedAt)}
                      </span>
                    </div>
                  ))
                ) : (
                  <div className="text-text-muted text-sm py-2 px-2">No recent projects found.</div>
                )}
              </div>
            </CardContent>
          </Card>

          {/* Recent Chats Card */}
          <Card className="w-full p-3 px-2 animate-in fade-in slide-in-from-right-8 duration-500">
            <CardContent className="px-0">
              <div className="flex justify-between items-center mb-4">
                <CardDescription className="mb-0 px-2">
                  <span className="text-lg text-text-default">Recent chats</span>
                </CardDescription>
                <Button
                  variant="ghost"
                  size="sm"
                  className="text-xs text-text-muted flex items-center gap-1 !px-0 hover:bg-transparent hover:underline hover:text-text-default"
                  onClick={navigateToSessionHistory}
                >
                  See all
                </Button>
              </div>
              <div className="space-y-1">
                {recentSessions.length > 0 ? (
                  recentSessions.map((session) => (
                    <div
                      key={session.id}
                      className="flex items-center justify-between text-sm py-1 px-2 rounded-md hover:bg-background-muted cursor-pointer transition-colors"
                      onClick={() => handleSessionClick(session.id)}
                      role="button"
                      tabIndex={0}
                      onKeyDown={(e) => {
                        if (e.key === 'Enter' || e.key === ' ') {
                          handleSessionClick(session.id);
                        }
                      }}
                    >
                      <div className="flex items-center space-x-2">
                        <ChatSmart className="h-4 w-4 text-text-muted" />
                        <span className="truncate max-w-[200px]">
                          {session.metadata.description || session.id}
                        </span>
                      </div>
                      <span className="text-text-muted font-mono font-light">
                        {formatDateOnly(session.modified)}
                      </span>
                    </div>
                  ))
                ) : (
                  <div className="text-text-muted text-sm py-2">No recent chat sessions found.</div>
                )}
              </div>
            </CardContent>
          </Card>
        </div>
      </div>
    </>
  );
}
