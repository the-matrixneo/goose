/**
 * Goose Component Library for MCP-UI Remote DOM
 * 
 * Maps existing Goose UI components to MCP-UI Remote DOM component names.
 * This allows MCP servers to use Goose's design system components in Remote DOM scripts.
 */

// Import existing Goose UI components
import { Button } from './ui/button';
import { Input } from './ui/input';
import { Textarea } from './ui/textarea';
import { Checkbox } from './ui/checkbox';
import { Switch } from './ui/switch';
import { Select } from './ui/Select';
import { Card } from './ui/card';
import { Tabs, TabsContent, TabsList, TabsTrigger } from './ui/tabs';
import { Dialog, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle, DialogTrigger } from './ui/dialog';
import { Sheet, SheetContent, SheetDescription, SheetFooter, SheetHeader, SheetTitle, SheetTrigger } from './ui/sheet';
import { Badge } from './ui/badge';
import { Tooltip } from './ui/Tooltip';
import { AlertDialog, AlertDialogAction, AlertDialogCancel, AlertDialogContent, AlertDialogDescription, AlertDialogFooter, AlertDialogHeader, AlertDialogTitle, AlertDialogTrigger } from './ui/alert-dialog';
import { Skeleton } from './ui/skeleton';
import { DropdownMenu, DropdownMenuContent, DropdownMenuItem, DropdownMenuLabel, DropdownMenuSeparator, DropdownMenuTrigger } from './ui/dropdown-menu';
import { Popover, PopoverContent, PopoverTrigger } from './ui/popover';
import { Separator } from './ui/separator';
import { Label } from './ui/label';

// Missing primitives that need to be created
import React from 'react';
import { cn } from '../utils';

/**
 * RemoteDOMText - Adapts Label component styling for text content
 */
interface RemoteDOMTextProps {
  children: React.ReactNode;
  variant?: 'body' | 'caption' | 'small' | 'large';
  className?: string;
}

const RemoteDOMText: React.FC<RemoteDOMTextProps> = ({ 
  children, 
  variant = 'body', 
  className 
}) => {
  const variantClasses = {
    body: 'text-sm',
    caption: 'text-xs text-text-muted',
    small: 'text-xs',
    large: 'text-base'
  };

  return (
    <span className={cn(variantClasses[variant], className)}>
      {children}
    </span>
  );
};

/**
 * RemoteDOMHeading - Heading component with Cash Sans typography
 */
interface RemoteDOMHeadingProps {
  children: React.ReactNode;
  level?: 1 | 2 | 3 | 4 | 5 | 6;
  className?: string;
}

const RemoteDOMHeading: React.FC<RemoteDOMHeadingProps> = ({ 
  children, 
  level = 1, 
  className 
}) => {
  const levelClasses = {
    1: 'text-2xl font-bold',
    2: 'text-xl font-semibold',
    3: 'text-lg font-semibold',
    4: 'text-base font-semibold',
    5: 'text-sm font-semibold',
    6: 'text-xs font-semibold'
  };

  const Tag = `h${level}` as keyof JSX.IntrinsicElements;

  return React.createElement(
    Tag,
    { className: cn(levelClasses[level], className) },
    children
  );
};

/**
 * RemoteDOMContainer - Layout container without Card borders
 */
interface RemoteDOMContainerProps {
  children: React.ReactNode;
  padding?: 'none' | 'sm' | 'md' | 'lg';
  className?: string;
}

const RemoteDOMContainer: React.FC<RemoteDOMContainerProps> = ({ 
  children, 
  padding = 'md', 
  className 
}) => {
  const paddingClasses = {
    none: '',
    sm: 'p-2',
    md: 'p-4',
    lg: 'p-6'
  };

  return (
    <div className={cn(paddingClasses[padding], className)}>
      {children}
    </div>
  );
};

/**
 * RemoteDOMStack - Flexbox stack layout
 */
interface RemoteDOMStackProps {
  children: React.ReactNode;
  direction?: 'vertical' | 'horizontal';
  gap?: 'none' | 'sm' | 'md' | 'lg';
  align?: 'start' | 'center' | 'end' | 'stretch';
  justify?: 'start' | 'center' | 'end' | 'between' | 'around';
  className?: string;
}

const RemoteDOMStack: React.FC<RemoteDOMStackProps> = ({ 
  children, 
  direction = 'vertical', 
  gap = 'md',
  align = 'stretch',
  justify = 'start',
  className 
}) => {
  const directionClasses = {
    vertical: 'flex-col',
    horizontal: 'flex-row'
  };

  const gapClasses = {
    none: '',
    sm: 'gap-2',
    md: 'gap-4',
    lg: 'gap-6'
  };

  const alignClasses = {
    start: 'items-start',
    center: 'items-center',
    end: 'items-end',
    stretch: 'items-stretch'
  };

  const justifyClasses = {
    start: 'justify-start',
    center: 'justify-center',
    end: 'justify-end',
    between: 'justify-between',
    around: 'justify-around'
  };

  return (
    <div className={cn(
      'flex',
      directionClasses[direction],
      gapClasses[gap],
      alignClasses[align],
      justifyClasses[justify],
      className
    )}>
      {children}
    </div>
  );
};

/**
 * RemoteDOMGrid - CSS Grid layout wrapper
 */
interface RemoteDOMGridProps {
  children: React.ReactNode;
  columns?: number | string;
  rows?: number | string;
  gap?: 'none' | 'sm' | 'md' | 'lg';
  className?: string;
}

const RemoteDOMGrid: React.FC<RemoteDOMGridProps> = ({ 
  children, 
  columns = 'auto',
  rows = 'auto',
  gap = 'md',
  className 
}) => {
  const gapClasses = {
    none: '',
    sm: 'gap-2',
    md: 'gap-4',
    lg: 'gap-6'
  };

  const gridStyle = {
    gridTemplateColumns: typeof columns === 'number' ? `repeat(${columns}, 1fr)` : columns,
    gridTemplateRows: typeof rows === 'number' ? `repeat(${rows}, 1fr)` : rows,
  };

  return (
    <div 
      className={cn('grid', gapClasses[gap], className)}
      style={gridStyle}
    >
      {children}
    </div>
  );
};

/**
 * Component Library Export
 * Maps MCP-UI component names to Goose components
 */
export const gooseComponentLibrary = {
  // Form Components (✅ Available)
  'ui-button': Button,
  'ui-input': Input,
  'ui-textarea': Textarea,
  'ui-checkbox': Checkbox,
  'ui-switch': Switch,
  'ui-select': Select,
  'ui-label': Label,
  
  // Layout Components (✅ Available)
  'ui-card': Card,
  'ui-tabs': Tabs,
  'ui-tabs-content': TabsContent,
  'ui-tabs-list': TabsList,
  'ui-tabs-trigger': TabsTrigger,
  'ui-dialog': Dialog,
  'ui-dialog-content': DialogContent,
  'ui-dialog-description': DialogDescription,
  'ui-dialog-footer': DialogFooter,
  'ui-dialog-header': DialogHeader,
  'ui-dialog-title': DialogTitle,
  'ui-dialog-trigger': DialogTrigger,
  'ui-sheet': Sheet,
  'ui-sheet-content': SheetContent,
  'ui-sheet-description': SheetDescription,
  'ui-sheet-footer': SheetFooter,
  'ui-sheet-header': SheetHeader,
  'ui-sheet-title': SheetTitle,
  'ui-sheet-trigger': SheetTrigger,
  
  // Feedback Components (✅ Available)
  'ui-badge': Badge,
  'ui-tooltip': Tooltip,
  'ui-alert-dialog': AlertDialog,
  'ui-alert-dialog-action': AlertDialogAction,
  'ui-alert-dialog-cancel': AlertDialogCancel,
  'ui-alert-dialog-content': AlertDialogContent,
  'ui-alert-dialog-description': AlertDialogDescription,
  'ui-alert-dialog-footer': AlertDialogFooter,
  'ui-alert-dialog-header': AlertDialogHeader,
  'ui-alert-dialog-title': AlertDialogTitle,
  'ui-alert-dialog-trigger': AlertDialogTrigger,
  'ui-skeleton': Skeleton,
  
  // Navigation Components (✅ Available)
  'ui-dropdown-menu': DropdownMenu,
  'ui-dropdown-menu-content': DropdownMenuContent,
  'ui-dropdown-menu-item': DropdownMenuItem,
  'ui-dropdown-menu-label': DropdownMenuLabel,
  'ui-dropdown-menu-separator': DropdownMenuSeparator,
  'ui-dropdown-menu-trigger': DropdownMenuTrigger,
  'ui-popover': Popover,
  'ui-popover-content': PopoverContent,
  'ui-popover-trigger': PopoverTrigger,
  'ui-separator': Separator,
  
  // Missing Primitives (🔧 Newly created)
  'ui-text': RemoteDOMText,
  'ui-heading': RemoteDOMHeading,
  'ui-container': RemoteDOMContainer,
  'ui-stack': RemoteDOMStack,
  'ui-grid': RemoteDOMGrid,
};

export type GooseComponentLibrary = typeof gooseComponentLibrary;
