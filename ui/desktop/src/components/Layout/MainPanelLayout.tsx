import React, { useCallback, useRef, useState, useEffect } from 'react';
import SidecarPanel from '../Sidecar/SidecarPanel';
import { SidecarProvider, useSidecar } from '../Sidecar/SidecarContext';

// Layout constants
const CHAT_MIN_WIDTH = 600;
const SIDECAR_MIN_WIDTH = 320;
const SIDECAR_DEFAULT_WIDTH = 400;

interface LayoutState {
  mode: 'steady' | 'animating';
  fixedChatWidth?: number;
  transitionPhase?: 'preparing' | 'expanding-window' | 'revealing-sidecar' | 'complete';
}

export const MainPanelLayout: React.FC<{
  children: React.ReactNode;
  removeTopPadding?: boolean;
  backgroundColor?: string;
}> = ({ children, removeTopPadding = false, backgroundColor = 'bg-background-default' }) => {
  const [layoutState, setLayoutState] = useState<LayoutState>({ mode: 'steady' });
  const [sidecarPixelWidth, setSidecarPixelWidth] = useState(SIDECAR_DEFAULT_WIDTH);
  const chatRef = useRef<HTMLDivElement>(null!);

  return (
    <SidecarProvider>
      <MainPanelContent
        backgroundColor={backgroundColor}
        removeTopPadding={removeTopPadding}
        layoutState={layoutState}
        setLayoutState={setLayoutState}
        sidecarPixelWidth={sidecarPixelWidth}
        setSidecarPixelWidth={setSidecarPixelWidth}
        chatRef={chatRef}
      >
        {children}
      </MainPanelContent>
    </SidecarProvider>
  );
};

// Separate component to use the sidecar context
const MainPanelContent: React.FC<{
  children: React.ReactNode;
  backgroundColor: string;
  removeTopPadding: boolean;
  layoutState: LayoutState;
  setLayoutState: React.Dispatch<React.SetStateAction<LayoutState>>;
  sidecarPixelWidth: number;
  setSidecarPixelWidth: React.Dispatch<React.SetStateAction<number>>;
  chatRef: React.RefObject<HTMLDivElement>;
}> = ({
  children,
  backgroundColor,
  removeTopPadding,
  layoutState,
  setLayoutState,
  sidecarPixelWidth,
  setSidecarPixelWidth,
  chatRef,
}) => {
  const { isOpen: sidecarOpen, close } = useSidecar();
  const prevSidecarOpen = useRef(sidecarOpen);

  // Update window minimum size based on open components
  useEffect(() => {
    const updateWindowMinimum = () => {
      // Calculate minimum window width needed
      let minWindowWidth = CHAT_MIN_WIDTH; // Base chat minimum

      if (sidecarOpen) {
        minWindowWidth += SIDECAR_MIN_WIDTH; // Add sidecar minimum
      }

      // TODO: Add sidebar width if we need to account for it
      // For now, sidebar is handled by the mobile breakpoint (930px)

      // Ensure we don't go below the app's absolute minimum
      minWindowWidth = Math.max(minWindowWidth, 750);

      // Set the window minimum size
      if (window.electron && window.electron.setWindowMinimumSize) {
        window.electron.setWindowMinimumSize(minWindowWidth, 600);
      }
    };

    updateWindowMinimum();
  }, [sidecarOpen]);

  // Handle sidecar opening animation
  useEffect(() => {
    // Only trigger animation on actual state change
    if (sidecarOpen !== prevSidecarOpen.current) {
      if (sidecarOpen) {
        // Opening animation
        const currentChatWidth = chatRef.current?.offsetWidth || CHAT_MIN_WIDTH;

        setLayoutState({
          mode: 'animating',
          fixedChatWidth: currentChatWidth,
          transitionPhase: 'preparing',
        });

        // Notify main process to disable sidebar auto-show
        window.electron.setSidecarTransitioning?.(true);

        // Small delay to ensure layout state is applied
        setTimeout(() => {
          setLayoutState((prev) => ({ ...prev, transitionPhase: 'expanding-window' }));

          // Wait for window resize animation
          setTimeout(() => {
            setLayoutState((prev) => ({ ...prev, transitionPhase: 'revealing-sidecar' }));

            // After sidecar reveal animation, return to steady state
            setTimeout(() => {
              setLayoutState({ mode: 'steady' });
              window.electron.setSidecarTransitioning?.(false);
            }, 200); // Sidecar animation duration
          }, 150); // Window resize duration
        }, 50);
      } else {
        // Closing - reset immediately
        setLayoutState({ mode: 'steady' });
        window.electron.setSidecarTransitioning?.(false);
      }

      // Update previous state
      prevSidecarOpen.current = sidecarOpen;
    }
  }, [sidecarOpen, chatRef, setLayoutState]);

  // Handle manual window resizing
  useEffect(() => {
    if (layoutState.mode === 'animating') return;

    const handleResize = () => {
      if (!sidecarOpen) return;

      const container = document.querySelector('#main-panel-layout-container') as HTMLDivElement;
      if (!container) return;

      const totalWidth = container.offsetWidth;
      const availableWidth = totalWidth;

      // Ensure minimums are respected
      if (availableWidth < CHAT_MIN_WIDTH + SIDECAR_MIN_WIDTH) {
        // Window too small, might need to hide sidecar
        return;
      }

      // Distribute extra space proportionally (70/30 default)
      const minRequired = CHAT_MIN_WIDTH + SIDECAR_MIN_WIDTH;
      const extraSpace = Math.max(0, availableWidth - minRequired);
      const sidecarExtra = extraSpace * 0.3;

      setSidecarPixelWidth(SIDECAR_MIN_WIDTH + sidecarExtra);
    };

    const resizeObserver = new ResizeObserver(handleResize);
    const container = document.querySelector('#main-panel-layout-container');
    if (container) {
      resizeObserver.observe(container);
    }

    return () => resizeObserver.disconnect();
  }, [sidecarOpen, layoutState.mode, setSidecarPixelWidth]);

  // Determine chat container style based on layout state
  const chatStyle: React.CSSProperties =
    layoutState.mode === 'animating'
      ? {
          width: `${layoutState.fixedChatWidth}px`,
          flexGrow: 0,
          flexShrink: 0,
          transition: 'none',
        }
      : {
          flex: '1 1 auto',
          minWidth: `${CHAT_MIN_WIDTH}px`,
        };

  // Determine sidecar style
  const sidecarStyle: React.CSSProperties = {
    width: `${sidecarPixelWidth}px`,
    flexShrink: 0,
    transition:
      layoutState.mode === 'animating' && layoutState.transitionPhase === 'revealing-sidecar'
        ? 'transform 200ms ease-out, opacity 200ms ease-out'
        : 'none',
    transform:
      layoutState.transitionPhase === 'expanding-window' ? 'translateX(100%)' : 'translateX(0)',
    opacity: layoutState.transitionPhase === 'expanding-window' ? 0 : 1,
  };

  return (
    <div className="h-dvh">
      <div
        id="main-panel-layout-container"
        className={`flex ${backgroundColor} flex-1 min-w-0 h-full min-h-0`}
      >
        <div
          ref={chatRef}
          className={`flex flex-col ${removeTopPadding ? '' : 'pt-[32px]'}`}
          style={chatStyle}
        >
          {children}
        </div>
        {sidecarOpen && (
          <>
            <ResizableSeparator
              isOpen={sidecarOpen}
              layoutState={layoutState}
              setSidecarPixelWidth={setSidecarPixelWidth}
              close={close}
            />
            <div
              style={sidecarStyle}
              className={`pointer-events-auto ${
                layoutState.transitionPhase === 'expanding-window' ? 'pointer-events-none' : ''
              }`}
            >
              <SidecarPanel />
            </div>
          </>
        )}
      </div>
    </div>
  );
};

// Update ResizableSeparator to be a standalone component
const ResizableSeparator: React.FC<{
  isOpen: boolean;
  layoutState: LayoutState;
  setSidecarPixelWidth: React.Dispatch<React.SetStateAction<number>>;
  close: () => void;
}> = ({ isOpen, layoutState, setSidecarPixelWidth, close }) => {
  const dragging = useRef(false);

  const onMouseDown = useCallback(
    (e: React.MouseEvent) => {
      if (!isOpen || layoutState.mode === 'animating') return;
      dragging.current = true;
      document.body.style.cursor = 'col-resize';
      e.preventDefault();
      e.stopPropagation();
    },
    [isOpen, layoutState.mode]
  );

  const onMouseMove = useCallback(
    (e: MouseEvent) => {
      if (!dragging.current) return;
      const container = document.querySelector('#main-panel-layout-container') as HTMLDivElement;
      if (!container) return;
      const rect = container.getBoundingClientRect();
      const distanceFromRight = Math.max(0, Math.min(rect.width, rect.right - e.clientX));

      // Calculate chat width to ensure minimum
      const chatWidth = rect.width - distanceFromRight;

      if (chatWidth < CHAT_MIN_WIDTH) {
        // Don't allow chat to go below minimum
        setSidecarPixelWidth(rect.width - CHAT_MIN_WIDTH);
        return;
      }

      if (distanceFromRight < SIDECAR_MIN_WIDTH) {
        // Auto-collapse if below minimum
        close();
        dragging.current = false;
        document.body.style.cursor = '';
        return;
      }

      setSidecarPixelWidth(distanceFromRight);
    },
    [close, setSidecarPixelWidth]
  );

  const onMouseUp = useCallback(() => {
    if (!dragging.current) return;
    dragging.current = false;
    document.body.style.cursor = '';
  }, []);

  useEffect(() => {
    window.addEventListener('mousemove', onMouseMove);
    window.addEventListener('mouseup', onMouseUp);
    return () => {
      window.removeEventListener('mousemove', onMouseMove);
      window.removeEventListener('mouseup', onMouseUp);
    };
  }, [onMouseMove, onMouseUp]);

  return (
    <div
      className={`relative ${isOpen && layoutState.mode === 'steady' ? 'opacity-100' : 'opacity-0 pointer-events-none'}`}
    >
      <div className="absolute inset-y-0 -left-[3px] w-[6px]" />
      <div
        role="separator"
        aria-orientation="vertical"
        className={`w-[6px] cursor-col-resize bg-transparent hover:bg-borderSubtle/60 active:bg-borderSubtle transition-colors`}
        onMouseDown={onMouseDown}
        title="Resize side panel"
      />
    </div>
  );
};
