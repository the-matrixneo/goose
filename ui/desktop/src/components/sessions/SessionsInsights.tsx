import { useEffect, useState } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../ui/card';
import { FolderOpen } from 'lucide-react';
import { getApiUrl, getSecretKey } from '../../config';
import { useTextAnimator } from '../../hooks/use-text-animator';
import { Greeting } from '../common/Greeting';
import { ActivityHeatmap } from '../common/ActivityHeatmap';

interface SessionInsights {
  totalSessions: number;
  mostActiveDirs: [string, number][];
  avgSessionDuration: number;
  totalTokens: number;
}

export function SessionInsights() {
  const [insights, setInsights] = useState<SessionInsights | null>(null);
  const [error, setError] = useState<string | null>(null);

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

    loadInsights();
  }, []);

  if (!insights) {
    return <></>;
  }

  return (
    <>
      <Greeting />

      <div className="grid grid-cols-4 gap-4">
        {/* Total Sessions Card */}
        <Card className="w-full p-3 h-32 animate-in fade-in slide-in-from-right-8 duration-500">
          <CardContent className="flex flex-col justify-end h-full px-0">
            <div className="flex flex-col justify-end">
              <p className="text-4xl font-mono font-light flex items-end">
                {insights?.totalSessions}
              </p>
              <CardDescription>Total sessions</CardDescription>
            </div>
          </CardContent>
        </Card>

        {/* Average Duration Card */}
        <Card className="w-full p-3 h-32 animate-in fade-in slide-in-from-right-8 duration-500">
          <CardContent className="flex flex-col justify-end h-full px-0">
            <div className="flex flex-col justify-end">
              <p className="text-4xl font-mono font-light flex items-end">
                {insights?.avgSessionDuration?.toFixed(1)}m
              </p>
              <CardDescription>Avg. chat length</CardDescription>
            </div>
          </CardContent>
        </Card>

        {/* Total Tokens Card */}
        <Card className="w-full p-3 h-32 col-span-2 animate-in fade-in slide-in-from-right-8 duration-500">
          <CardContent className="flex flex-col justify-end h-full px-0">
            <div className="flex flex-col justify-end">
              <p className="text-4xl font-mono font-light flex items-end">
                {insights?.totalTokens ? `${(insights.totalTokens / 1000000).toFixed(2)}M` : ''}
              </p>
              <CardDescription>Total tokens</CardDescription>
            </div>
          </CardContent>
        </Card>

        {/* Activity Heatmap Card */}
        <Card className="w-full col-span-4 animate-in fade-in slide-in-from-right-8 duration-500">
          <CardHeader>
            <CardTitle className="text-lg">Activity</CardTitle>
          </CardHeader>
          <CardContent>
            <ActivityHeatmap />
          </CardContent>
        </Card>

        {/* Most Active Directories Card */}
        {/* <Card className="w-full p-3 col-span-4 animate-in fade-in slide-in-from-right-8 duration-500">
          <CardContent className="px-0">
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
    </>
  );
}
