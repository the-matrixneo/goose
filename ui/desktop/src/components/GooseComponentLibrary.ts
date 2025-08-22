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
 * Text - Adapts Label component styling for text content
 * Extracts patterns from Label component with semantic tokens
 */
interface TextProps {
  children: React.ReactNode;
  variant?: 'body' | 'caption' | 'small' | 'large';
  className?: string;
}

const Text: React.FC<TextProps> = ({ 
  children, 
  variant = 'body', 
  className 
}) => {
  const variantClasses = {
    body: 'text-sm leading-none', // Matches Label component
    caption: 'text-xs text-text-muted leading-none',
    small: 'text-xs leading-none',
    large: 'text-base leading-none'
  };

  return React.createElement(
    'span',
    { 
      className: cn(
        'select-none', // Matches Label component behavior
        variantClasses[variant], 
        className
      )
    },
    children
  );
};

/**
 * Heading - Heading component with Cash Sans typography
 * Extracts patterns from Greeting component with proper hierarchy
 */
interface HeadingProps {
  children: React.ReactNode;
  level?: 1 | 2 | 3 | 4 | 5 | 6;
  className?: string;
}

const Heading: React.FC<HeadingProps> = ({ 
  children, 
  level = 1, 
  className 
}) => {
  const levelClasses = {
    1: 'text-4xl font-light', // Matches Greeting component
    2: 'text-3xl font-light',
    3: 'text-2xl font-normal',
    4: 'text-xl font-normal',
    5: 'text-lg font-medium',
    6: 'text-base font-medium'
  };

  const Tag = `h${level}` as keyof JSX.IntrinsicElements;

  return React.createElement(
    Tag,
    { className: cn(levelClasses[level], className) },
    children
  );
};

/**
 * Container - Layout container without Card borders
 */
interface ContainerProps {
  children: React.ReactNode;
  padding?: 'none' | 'sm' | 'md' | 'lg';
  className?: string;
}

const Container: React.FC<ContainerProps> = ({ 
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
 * Stack - Flexbox stack layout
 */
interface StackProps {
  children: React.ReactNode;
  direction?: 'vertical' | 'horizontal';
  gap?: 'none' | 'sm' | 'md' | 'lg';
  align?: 'start' | 'center' | 'end' | 'stretch';
  justify?: 'start' | 'center' | 'end' | 'between' | 'around';
  className?: string;
}

const Stack: React.FC<StackProps> = ({ 
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
 * Grid - CSS Grid layout wrapper
 */
interface GridProps {
  children: React.ReactNode;
  columns?: number | string;
  rows?: number | string;
  gap?: 'none' | 'sm' | 'md' | 'lg';
  className?: string;
}

const Grid: React.FC<GridProps> = ({ 
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
 * Exposes Goose components with their natural React component names
 */
export const gooseComponentLibrary = {
  // Form Components (✅ Available)
  Button,
  Input,
  Textarea,
  Checkbox,
  Switch,
  Select,
  Label,
  
  // Layout Components (✅ Available)
  Card,
  Tabs,
  TabsContent,
  TabsList,
  TabsTrigger,
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
  Sheet,
  SheetContent,
  SheetDescription,
  SheetFooter,
  SheetHeader,
  SheetTitle,
  SheetTrigger,
  
  // Feedback Components (✅ Available)
  Badge,
  Tooltip,
  AlertDialog,
  AlertDialogAction,
  AlertDialogCancel,
  AlertDialogContent,
  AlertDialogDescription,
  AlertDialogFooter,
  AlertDialogHeader,
  AlertDialogTitle,
  AlertDialogTrigger,
  Skeleton,
  
  // Navigation Components (✅ Available)
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuLabel,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
  Popover,
  PopoverContent,
  PopoverTrigger,
  Separator,
  
  // Missing Primitives (🔧 Newly created)
  Text,
  Heading,
  Container,
  Stack,
  Grid,
};

export type GooseComponentLibrary = typeof gooseComponentLibrary;
