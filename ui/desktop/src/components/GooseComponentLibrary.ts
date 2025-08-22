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

// Import new primitive components
import { Text } from './ui/text';
import { Heading } from './ui/heading';
import { Container } from './ui/container';
import { Stack } from './ui/stack';
import { Grid } from './ui/grid';

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
