import { useState, useEffect } from 'react';
import { WidgetType, DashboardState } from '../types/dashboard';
import { getApiUrl } from '../config';
import { fetchSessions } from '../sessions';

const DASHBOARD_STORAGE_KEY = 'goose-dashboard-layout';

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
      size: { width: 280, height: 180 },
      title: 'Recent Chats',
    },
  ],
  canvasSize: { width: 800, height: 600 },
};

// Load saved dashboard layout from localStorage
function loadDashboardLayout(): Partial<DashboardState> | null {
  try {
    const saved = localStorage.getItem(DASHBOARD_STORAGE_KEY);
    if (!saved) return null;
    
    const parsed = JSON.parse(saved);
    
    // Validate the structure
    if (!parsed.widgets || !Array.isArray(parsed.widgets)) {
      return null;
    }
    
    return parsed;
  } catch (error) {
    console.error('Failed to load dashboard layout:', error);
    return null;
  }
}

// Save dashboard layout to localStorage
function saveDashboardLayout(dashboardState: DashboardState) {
  try {
    // Only save layout-related data (positions, sizes), not dynamic data
    const layoutData = {
      widgets: dashboardState.widgets.map(widget => ({
        id: widget.id,
        type: widget.type,
        position: widget.position,
        size: widget.size,
        title: widget.title,
        // Don't save dynamic data like totalSessions, recentSessions, etc.
      })),
      canvasSize: dashboardState.canvasSize,
    };
    
    localStorage.setItem(DASHBOARD_STORAGE_KEY, JSON.stringify(layoutData));
  } catch (error) {
    console.error('Failed to save dashboard layout:', error);
  }
}

// Merge saved layout with default state
function mergeDashboardState(defaultState: DashboardState, savedLayout: Partial<DashboardState>): DashboardState {
  if (!savedLayout.widgets) return defaultState;
  
  // Create a map of saved widgets by ID for quick lookup
  const savedWidgetMap = new Map(
    savedLayout.widgets.map(widget => [widget.id, widget])
  );
  
  // Merge saved positions/sizes with default widgets
  const mergedWidgets = defaultState.widgets.map(defaultWidget => {
    const savedWidget = savedWidgetMap.get(defaultWidget.id);
    if (savedWidget) {
      return {
        ...defaultWidget,
        position: savedWidget.position,
        size: savedWidget.size,
        // Keep title from saved if available, otherwise use default
        title: savedWidget.title || defaultWidget.title,
      };
    }
    return defaultWidget;
  });
  
  return {
    ...defaultState,
    widgets: mergedWidgets,
    canvasSize: savedLayout.canvasSize || defaultState.canvasSize,
  };
}

export function useDashboard() {
  // Initialize with merged state (default + saved layout)
  const [dashboardState, setDashboardState] = useState<DashboardState>(() => {
    const savedLayout = loadDashboardLayout();
    return savedLayout ? mergeDashboardState(DEFAULT_DASHBOARD_STATE, savedLayout) : DEFAULT_DASHBOARD_STATE;
  });
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
    setDashboardState(prev => {
      const newState = {
        ...prev,
        widgets: prev.widgets.map(widget =>
          widget.id === id ? { ...widget, position } : widget
        )
      };
      
      // Save layout to localStorage whenever a widget is moved
      saveDashboardLayout(newState);
      
      return newState;
    });
  };

  const resizeWidget = (id: string, size: { width: number; height: number }) => {
    setDashboardState(prev => {
      const newState = {
        ...prev,
        widgets: prev.widgets.map(widget =>
          widget.id === id ? { ...widget, size } : widget
        )
      };
      
      // Save layout to localStorage whenever a widget is resized
      saveDashboardLayout(newState);
      
      return newState;
    });
  };

  // Function to reset dashboard to default layout
  const resetDashboardLayout = () => {
    try {
      localStorage.removeItem(DASHBOARD_STORAGE_KEY);
      setDashboardState(DEFAULT_DASHBOARD_STATE);
    } catch (error) {
      console.error('Failed to reset dashboard layout:', error);
    }
  };

  return {
    dashboardState,
    isLoading,
    error,
    moveWidget,
    resizeWidget,
    resetDashboardLayout,
  };
}

function getGreeting(): string {
  const hour = new Date().getHours();
  if (hour < 12) return 'Good morning!';
  if (hour < 18) return 'Good afternoon!';
  return 'Good evening!';
}
