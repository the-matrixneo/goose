import React, { createContext, useCallback, useContext, useMemo, useState } from 'react';
import type { ResourceContent } from '../../types/message';

export type SidecarContent =
  | { kind: 'none' }
  | { kind: 'mcp-ui'; resource: ResourceContent; appendPromptToChat?: (value: string) => void };

type SidecarContextValue = {
  isOpen: boolean;
  content: SidecarContent;
  open: () => void;
  openWithMCPUI: (payload: {
    resource: ResourceContent;
    appendPromptToChat?: (value: string) => void;
  }) => void;
  toggleMCPUI: (payload: {
    resource: ResourceContent;
    appendPromptToChat?: (value: string) => void;
  }) => void;
  close: () => void;
};

const SidecarContext = createContext<SidecarContextValue | null>(null);

export function useSidecar() {
  const ctx = useContext(SidecarContext);
  if (!ctx) throw new Error('useSidecar must be used within SidecarProvider');
  return ctx;
}

export function SidecarProvider({ children }: { children: React.ReactNode }) {
  const [isOpen, setIsOpen] = useState(false);
  const [content, setContent] = useState<SidecarContent>({ kind: 'none' });

  const close = useCallback(() => {
    setIsOpen(false);
    if (window?.electron && 'setSidecarOpen' in window.electron) {
      // notify main process to restore window size
      // @ts-expect-error exposed in preload
      window.electron.setSidecarOpen(false);
    }
  }, []);

  const open = useCallback(() => {
    setIsOpen(true);
    if (window?.electron && 'setSidecarOpen' in window.electron) {
      // notify main process to enlarge window
      // @ts-expect-error exposed in preload
      window.electron.setSidecarOpen(true);
    }
  }, []);

  const openWithMCPUI = useCallback(
    (payload: { resource: ResourceContent; appendPromptToChat?: (value: string) => void }) => {
      setContent({
        kind: 'mcp-ui',
        resource: payload.resource,
        appendPromptToChat: payload.appendPromptToChat,
      });
      // Only resize window if sidecar wasn't already open
      if (!isOpen && window?.electron && 'setSidecarOpen' in window.electron) {
        console.log('Resizing window for sidecar open');
        // notify main process to enlarge window
        // @ts-expect-error exposed in preload
        window.electron.setSidecarOpen(true);
      }
      setIsOpen(true);
    },
    [isOpen]
  );

  const toggleMCPUI = useCallback(
    (payload: { resource: ResourceContent; appendPromptToChat?: (value: string) => void }) => {
      const currentUri = content.kind === 'mcp-ui' ? content.resource.resource.uri : undefined;
      const nextUri = payload.resource.resource.uri;
      if (isOpen && currentUri && nextUri && currentUri === nextUri) {
        close();
        return;
      }
      openWithMCPUI(payload);
    },
    [content, isOpen, close, openWithMCPUI]
  );

  const value = useMemo<SidecarContextValue>(
    () => ({ isOpen, content, open, openWithMCPUI, toggleMCPUI, close }),
    [isOpen, content, open, openWithMCPUI, toggleMCPUI, close]
  );

  return <SidecarContext.Provider value={value}>{children}</SidecarContext.Provider>;
}
