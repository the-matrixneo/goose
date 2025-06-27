import { useEffect, useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../ui/card';
import { FolderOpen } from 'lucide-react';
import { getApiUrl, getSecretKey } from '../../config';
import { useTextAnimator } from '../../hooks/use-text-animator';
import GooseLogo from '../GooseLogo';
import GoosIN from '../games/GoosIN';

interface SessionInsights {
  totalSessions: number;
  mostActiveDirs: [string, number][];
  avgSessionDuration: number;
  totalTokens: number;
}

// Greeting component
function GreetingCard() {
  const [selectedGreeting, setSelectedGreeting] = useState<{
    prefix: string;
  } | null>(null);

  // Select a random greeting on component mount
  useEffect(() => {
    const prefixes = ['Hello.', 'Welcome.', 'Greetings.', 'Welcome back.', 'Hello there.'];
    const randomPrefixIndex = Math.floor(Math.random() * prefixes.length);

    setSelectedGreeting({
      prefix: prefixes[randomPrefixIndex],
    });
  }, []);

  const greeting = selectedGreeting || { prefix: 'Hello.', message: ' How can I help you today?' };

  return (
    <Card className="col-span-2 border-none animate-in fade-in slide-in-from-right-8 duration-300 bg-background-accent rounded-2xl max-w-[600px] relative">
      <CardContent className="flex flex-col justify-end items-start h-full pt-8 pb-0">
        <div className="absolute top-4 left-4">
          <GooseLogo size="small" className="text-white w-6 h-6" />
        </div>
        <div className="pt-16">
          <h1 className="text-text-on-accent text-4xl font-light">
            <span>{greeting.prefix}</span>
          </h1>
        </div>
        {/* <p className="text-text-on-accent font-light text-lg">{greeting.message}</p> */}
      </CardContent>
    </Card>
  );
}

export function SessionInsights() {
  const [insights, setInsights] = useState<SessionInsights | null>(null);
  const [error, setError] = useState<string | null>(null);
  const [currentTime, setCurrentTime] = useState<string>('');

  // Add text animator effects for each number
  const totalSessionsRef = useTextAnimator({ text: insights?.totalSessions.toString() ?? '' });
  const avgDurationRef = useTextAnimator({
    text: insights?.avgSessionDuration ? insights.avgSessionDuration.toFixed(1) : '',
  });
  const totalTokensRef = useTextAnimator({
    text: insights?.totalTokens ? (insights.totalTokens / 1000000).toFixed(2) : '',
  });

  // Update time every minute
  useEffect(() => {
    const updateTime = () => {
      const now = new Date();
      const time = now.toLocaleTimeString('en-US', {
        hour: 'numeric',
        minute: '2-digit',
        hour12: true,
      });
      setCurrentTime(time);
    };

    updateTime(); // Set initial time
    const interval = setInterval(updateTime, 60000); // Update every minute

    return () => clearInterval(interval);
  }, []);

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

    loadInsights();
  }, []);

  if (!insights) {
    return <></>;
  }

  return (
    <div className="flex items-center min-h-[60vh]">
      <div className="w-full">
        <div className="grid grid-cols-4 gap-4 mb-4">
          <GreetingCard />
        </div>

        <div className="flex flex-wrap gap-4 mb-4">
          {/* Total Sessions Card */}
          <Card className="w-full sm:w-auto animate-in fade-in slide-in-from-right-8 duration-500 rounded-2xl min-w-[200px] max-w-[350px]">
            <CardContent className="flex flex-col justify-end items-start h-full pt-4">
              <div className="flex flex-col justify-end items-start">
                <p className="text-2xl font-mono font-light flex items-end" ref={totalSessionsRef}>
                  {insights?.totalSessions}
                </p>
                <CardDescription>Total sessions</CardDescription>
              </div>
            </CardContent>
          </Card>

          {/* Average Duration Card */}
          <Card className="w-full sm:w-auto animate-in fade-in slide-in-from-right-8 duration-500 rounded-2xl min-w-[200px] max-w-[350px]">
            <CardContent className="flex flex-col justify-end items-start h-full pt-4">
              <div className="flex flex-col justify-end items-start">
                <p className="text-2xl font-mono font-light flex items-end">
                  <span ref={avgDurationRef}>{insights?.avgSessionDuration?.toFixed(1)}</span>
                  <span className="text-base">m</span>
                </p>
                <CardDescription>Avg. duration</CardDescription>
              </div>
            </CardContent>
          </Card>

          {/* Total Tokens Card */}
          <Card className="w-full sm:w-auto animate-in fade-in slide-in-from-right-8 duration-500 rounded-2xl min-w-[200px] max-w-[350px]">
            <CardContent className="flex flex-col justify-end items-start h-full pt-4">
              <div className="flex flex-col justify-end items-start">
                <p className="text-2xl font-mono font-light flex items-end" ref={totalTokensRef}>
                  <span>{insights?.totalTokens ? (insights.totalTokens / 1000000).toFixed(2) : ''}</span>
                  <span className="text-base">M</span>
                </p>
                <CardDescription>Total tokens</CardDescription>
              </div>
            </CardContent>
          </Card>

          {/* Current Time Card */}
          <Card className="w-full sm:w-auto animate-in fade-in slide-in-from-right-8 duration-500 rounded-2xl min-w-[200px] max-w-[350px]">
            <CardContent className="flex flex-col justify-end items-start h-full pt-4">
              <div className="flex flex-col justify-end items-start">
                <p className="text-2xl font-mono font-light flex items-end">
                  <span>{currentTime.replace(/ (AM|PM)$/, '')}</span>
                  <span className="text-base">{currentTime.match(/ (AM|PM)$/)?.[1] || ''}</span>
                </p>
                <CardDescription>Current time</CardDescription>
              </div>
            </CardContent>
          </Card>
        </div>

        {/* GOOSIN Game */}
        <div className="flex justify-center">
          <GoosIN />
        </div>

        {/* Most Active Directories Card */}
        {/* <Card className="w-full col-span-4 animate-in fade-in slide-in-from-right-8 duration-500 rounded-2xl">
          <CardContent>
            <CardDescription className="mb-4">
              <span className="text-lg text-text-default">Active directories</span>
            </CardDescription>
            <div className="space-y-2 ">
              {insights.mostActiveDirs.map(([dir, count], index) => (
                <div key={index} className="flex items-center justify-between text-sm">
                  <div className="flex items-center space-x-2">
                    <FolderOpen className="h-4 w-4 text-text-muted" />
                    <span className="truncate max-w-[400px] rtl">{dir}</span>
                  </div>
                  <span className="text-text-default font-mono font-light">{count} sessions</span>
                </div>
              ))}
            </div>
          </CardContent>
        </Card> */}
      </div>
    </div>
  );
}
