import { ReactNode } from 'react';

export interface SidecarView {
  id: string;
  title: string;
  content: ReactNode;
}

export interface SidecarContextType {
  activeView: string | null;
  views: SidecarView[];
  showView: (view: SidecarView) => void;
  hideView: () => void;
  showPenpotDesigner: (projectId?: string, fileId?: string, initialDesign?: string) => void;
  hidePenpotDesigner: () => void;
  showDiffViewer: (diffContent: string, fileName: string) => void;
}
