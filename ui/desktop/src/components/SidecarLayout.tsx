/// <reference lib="dom" />
import React, { useState, createContext, useContext, useEffect } from 'react';
//import { X, FileDiff, SquareSplitHorizontal, BetweenHorizontalStart, Palette, RefreshCw, Square as StopIcon, ExternalLink } from 'lucide-react';
import {
  X,
  SquareSplitHorizontal,
  BetweenHorizontalStart,
  RefreshCw,
  Square as StopIcon,
  ExternalLink,
} from 'lucide-react';
import { Button } from './ui/button';
import { Tooltip, TooltipTrigger, TooltipContent } from './ui/Tooltip';
import { useWindowManager } from '../hooks/useWindowManager';
import PenpotCanvas from './PenpotCanvas';
import { SidecarContextType, SidecarView } from '../types/sidecar';

// Declare global Event and EventListener types
declare global {
  type CustomEventMap = {
    'penpot-docker-state-change': CustomEvent<{
      status: 'stopped' | 'starting' | 'running' | 'error';
    }>;
  };
}

const SidecarContext = createContext<SidecarContextType | null>(null);

export const useSidecar = () => {
  const context = useContext(SidecarContext);
  return context;
};

interface SidecarProviderProps {
  children: React.ReactNode;
  showSidecar?: boolean;
}

export function SidecarProvider({ children, showSidecar = true }: SidecarProviderProps) {
  const [activeView, setActiveView] = useState<string | null>(null);
  const [views, setViews] = useState<SidecarView[]>([]);

  const { toggleWindow } = useWindowManager({
    expandPercentage: 100,
    maxWidthForExpansion: 2200,
    transitionDuration: 300,
  });

  const showView = async (view: SidecarView) => {
    setViews((prev) => {
      const existing = prev.find((v) => v.id === view.id);
      if (existing) {
        return prev.map((v) => (v.id === view.id ? view : v));
      }
      return [...prev, view];
    });

    await toggleWindow();
    setActiveView(view.id);
  };

  const hideView = () => {
    setActiveView(null);
  };

  const showPenpotDesigner = (projectId?: string, fileId?: string, initialDesign?: string) => {
    const penpotView: SidecarView = {
      id: 'penpot',
      title: 'Penpot Designer',
      content: (
        <PenpotCanvas
          projectId={projectId}
          fileId={fileId}
          initialDesign={initialDesign}
          onDesignChange={(design) => {
            console.log('Design changed:', design);
          }}
          onExport={(format) => {
            console.log('Exporting as:', format);
          }}
        />
      ),
    };
    showView(penpotView);
  };

  const hidePenpotDesigner = () => {
    setViews((prev) => prev.filter((v) => v.id !== 'penpot'));
    if (activeView === 'penpot') {
      setActiveView(null);
    }
  };

  const showDiffViewer = (diffContent: string, fileName: string) => {
    const diffView: SidecarView = {
      id: 'diff',
      title: `Diff: ${fileName}`,
      content: (
        <div className="flex-1 overflow-auto p-4 font-mono whitespace-pre">{diffContent}</div>
      ),
    };
    showView(diffView);
  };

  const contextValue: SidecarContextType = {
    activeView,
    views,
    showView,
    hideView,
    showPenpotDesigner,
    hidePenpotDesigner,
    showDiffViewer,
  };

  if (!showSidecar) {
    return <SidecarContext.Provider value={contextValue}>{children}</SidecarContext.Provider>;
  }

  return <SidecarContext.Provider value={contextValue}>{children}</SidecarContext.Provider>;
}

export function Sidecar({ className = '' }: { className?: string }) {
  const sidecar = useSidecar();
  const [viewMode, setViewMode] = useState<'split' | 'unified'>('unified');

  // Docker state for Penpot - we'll need to access this from the PenpotCanvas component
  const [dockerState, setDockerState] = useState<{
    status: 'stopped' | 'starting' | 'running' | 'error';
  }>({ status: 'stopped' });

  // Update the diff viewer when view mode changes
  useEffect(() => {
    if (sidecar) {
      const { activeView, views } = sidecar;
      const currentView = views.find((v) => v.id === activeView);
      const isDiffViewer = currentView?.id === 'diff';

      if (
        isDiffViewer &&
        (
          window as unknown as {
            diffViewerControls?: {
              viewMode: string;
              setViewMode: (mode: 'split' | 'unified') => void;
            };
          }
        ).diffViewerControls
      ) {
        (
          window as unknown as {
            diffViewerControls?: {
              viewMode: string;
              setViewMode: (mode: 'split' | 'unified') => void;
            };
          }
        ).diffViewerControls!.setViewMode(viewMode);
      }
    }
  }, [viewMode, sidecar]);

  // Listen for Docker state changes from PenpotCanvas
  useEffect(() => {
    type DockerStateEvent = CustomEvent<{ status: 'stopped' | 'starting' | 'running' | 'error' }>;

    const handleDockerStateChange = ((e: Event) => {
      const customEvent = e as DockerStateEvent;
      setDockerState(customEvent.detail);
    }) satisfies EventListener;

    window.addEventListener('penpot-docker-state-change', handleDockerStateChange);
    return () => {
      window.removeEventListener('penpot-docker-state-change', handleDockerStateChange);
    };
  }, []);

  // Docker control functions
  const handleRefreshPenpot = () => {
    window.dispatchEvent(new CustomEvent('penpot-refresh-canvas'));
  };

  const handleStopPenpot = () => {
    window.dispatchEvent(new CustomEvent('penpot-stop-container'));
  };

  // const handleStartPenpot = () => {
  //   window.dispatchEvent(new CustomEvent('penpot-start-container'));
  // };

  const handleOpenInBrowser = () => {
    window.dispatchEvent(new CustomEvent('penpot-open-browser'));
  };

  if (!sidecar) return null;

  const { activeView, views, hideView } = sidecar;
  const currentView = views.find((v) => v.id === activeView);
  const isVisible = activeView && currentView;

  if (!isVisible) return null;

  // Check if current view is diff viewer or penpot
  const isDiffViewer = currentView.id === 'diff';
  const isPenpotDesigner = currentView.id === 'penpot';

  return (
    <div
      className={`bg-background-default overflow-hidden rounded-2xl flex flex-col m-5 h-full ${className}`}
      style={{ height: 'calc(100% - 40px)' }}
    >
      {currentView && (
        <>
          <div className="flex items-center justify-between p-4 border-b border-borderSubtle">
            <div className="flex items-center space-x-3">
              <div
                className={`w-3 h-3 rounded-full ${
                  dockerState.status === 'running'
                    ? 'bg-green-500'
                    : dockerState.status === 'starting'
                      ? 'bg-yellow-500'
                      : dockerState.status === 'error'
                        ? 'bg-red-500'
                        : 'bg-gray-500'
                }`}
              />
              <span className="text-textStandard font-medium">{currentView.title}</span>
            </div>
            <div className="flex items-center space-x-2">
              {/* Docker Status Indicator - Only show for Penpot */}
              {isPenpotDesigner && (
                <div className="flex items-center space-x-2">
                  <div
                    className={`w-2 h-2 rounded-full ${
                      dockerState.status === 'running'
                        ? 'bg-green-500'
                        : dockerState.status === 'starting'
                          ? 'bg-yellow-500'
                          : dockerState.status === 'error'
                            ? 'bg-red-500'
                            : 'bg-gray-500'
                    }`}
                  />

                  {/* Docker Control Buttons */}
                  {dockerState.status === 'running' && (
                    <>
                      <Tooltip>
                        <TooltipTrigger asChild>
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={handleRefreshPenpot}
                            className="p-1 h-8 w-8"
                          >
                            <RefreshCw size={14} />
                          </Button>
                        </TooltipTrigger>
                        <TooltipContent side="bottom" sideOffset={8}>
                          Refresh Canvas
                        </TooltipContent>
                      </Tooltip>

                      <Tooltip>
                        <TooltipTrigger asChild>
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={handleStopPenpot}
                            className="p-1 h-8 w-8"
                          >
                            <StopIcon size={14} />
                          </Button>
                        </TooltipTrigger>
                        <TooltipContent side="bottom" sideOffset={8}>
                          Stop Container
                        </TooltipContent>
                      </Tooltip>

                      <Tooltip>
                        <TooltipTrigger asChild>
                          <Button
                            variant="ghost"
                            size="sm"
                            onClick={handleOpenInBrowser}
                            className="p-1 h-8 w-8"
                          >
                            <ExternalLink size={14} />
                          </Button>
                        </TooltipTrigger>
                        <TooltipContent side="bottom" sideOffset={8}>
                          Open in Browser
                        </TooltipContent>
                      </Tooltip>
                    </>
                  )}
                </div>
              )}

              {/* View Mode Toggle - Only show for diff viewer */}
              {isDiffViewer && (
                <div className="flex items-center space-x-1 bg-background-muted rounded-lg p-1">
                  <Tooltip>
                    <TooltipTrigger asChild>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => setViewMode('unified')}
                        className={`px-2 py-1 cursor-pointer focus:outline-none focus:ring-2 focus:ring-borderProminent focus:ring-offset-1 ${
                          viewMode === 'unified'
                            ? 'bg-background-default text-textStandard hover:bg-background-default dark:hover:bg-background-default'
                            : 'text-textSubtle'
                        }`}
                      >
                        <BetweenHorizontalStart size={14} />
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent side="bottom" sideOffset={8}>
                      Unified View
                    </TooltipContent>
                  </Tooltip>

                  <Tooltip>
                    <TooltipTrigger asChild>
                      <Button
                        variant="ghost"
                        size="sm"
                        onClick={() => setViewMode('split')}
                        className={`px-2 py-1 cursor-pointer focus:outline-none focus:ring-2 focus:ring-borderProminent focus:ring-offset-1  ${
                          viewMode === 'split'
                            ? 'bg-background-default text-textStandard hover:bg-background-default dark:hover:bg-background-default'
                            : 'text-textSubtle'
                        }`}
                      >
                        <SquareSplitHorizontal size={14} />
                      </Button>
                    </TooltipTrigger>
                    <TooltipContent side="bottom" sideOffset={8}>
                      Split View
                    </TooltipContent>
                  </Tooltip>
                </div>
              )}

              <Tooltip>
                <TooltipTrigger asChild>
                  <Button
                    variant="ghost"
                    size="sm"
                    onClick={hideView}
                    className="text-textSubtle hover:text-textStandard"
                  >
                    <X size={16} />
                  </Button>
                </TooltipTrigger>
                <TooltipContent side="bottom">Close</TooltipContent>
              </Tooltip>
            </div>
          </div>

          <div className="flex-1 overflow-hidden">{currentView.content}</div>
        </>
      )}
    </div>
  );
}
