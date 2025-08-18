import { useState, useEffect } from 'react';
import { WidgetData, WidgetType, DashboardState } from '../types/dashboard';
import { getApiUrl } from '../config';
import { fetchSessions } from '../sessions';

const DEFAULT_DASHBOARD_STATE: DashboardState = {
  widgets: [
    {
      id: 'greeting',
      type: WidgetType.GREETING,
      position: { x: 50, y: 50 },
      size: { width: 300, height: 120 },
      title: 'Welcome',
    },
    {
      id: 'total-sessions',
      type: WidgetType.TOTAL_SESSIONS,
      position: { x: 50, y: 200 },
      size: { width: 180, height: 120 },
      title: 'Sessions',
    },
    {
      id: 'total-tokens',
      type: WidgetType.TOTAL_TOKENS,
      position: { x: 250, y: 200 },
      size: { width: 180, height: 120 },
      title: 'Tokens',
    },
    {
      id: 'recent-chats',
      type: WidgetType.RECENT_CHATS,
      position: { x: 450, y: 50 },
      size: { width: 280, height: 270 },
      title: 'Recent Chats',
    },
  ],
  canvasSize: { width: 800, height: 600 },
};

export function useDashboard() {
  const [dashboardState, setDashboardState] = useState<DashboardState>(DEFAULT_DASHBOARD_STATE);
  const [isLoading, setIsLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);

  // Load dashboard data
  useEffect(() => {
    const loadDashboardData = async () => {
      try {
        setIsLoading(true);
        
        // Load insights data
        const insightsResponse = await fetch(getApiUrl('/sessions/insights'), {
          headers: {
            Accept: 'application/json',
            'Content-Type': 'application/json',
            'X-Secret-Key': await window.electron.getSecretKey(),
          },
        });

        let insights = null;
        if (insightsResponse.ok) {
          insights = await insightsResponse.json();
        }

        // Load recent sessions
        const recentSessions = await fetchSessions();

        // Update widget data
        setDashboardState(prev => ({
          ...prev,
          widgets: prev.widgets.map(widget => {
            switch (widget.type) {
              case WidgetType.TOTAL_SESSIONS:
                return { ...widget, data: { totalSessions: insights?.totalSessions ?? 0 } };
              case WidgetType.TOTAL_TOKENS:
                return { ...widget, data: { totalTokens: insights?.totalTokens ?? 0 } };
              case WidgetType.RECENT_CHATS:
                return { ...widget, data: { recentSessions: recentSessions.slice(0, 5) } };
              case WidgetType.GREETING:
                return { 
                  ...widget, 
                  data: { 
                    greeting: getGreeting(),
                    subtitle: 'Ready to start a new conversation?'
                  } 
                };
              default:
                return widget;
            }
          })
        }));

        setError(null);
      } catch (err) {
        console.error('Failed to load dashboard data:', err);
        setError(err instanceof Error ? err.message : 'Failed to load dashboard data');
      } finally {
        setIsLoading(false);
      }
    };

    loadDashboardData();
  }, []);

  const moveWidget = (id: string, position: { x: number; y: number }) => {
    setDashboardState(prev => ({
      ...prev,
      widgets: prev.widgets.map(widget =>
        widget.id === id ? { ...widget, position } : widget
      )
    }));
  };

  const resizeWidget = (id: string, size: { width: number; height: number }) => {
    setDashboardState(prev => ({
      ...prev,
      widgets: prev.widgets.map(widget =>
        widget.id === id ? { ...widget, size } : widget
      )
    }));
  };

  return {
    dashboardState,
    isLoading,
    error,
    moveWidget,
    resizeWidget,
  };
}

function getGreeting(): string {
  const hour = new Date().getHours();
  if (hour < 12) return 'Good morning!';
  if (hour < 18) return 'Good afternoon!';
  return 'Good evening!';
}
