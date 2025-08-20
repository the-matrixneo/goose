import * as React from 'react';
import * as TooltipPrimitive from '@radix-ui/react-tooltip';

import { cn } from '../../utils';

// Create a dummy tooltip provider that doesn't actually provide tooltips
function TooltipProvider({
  children,
  ...props
}: React.ComponentProps<typeof TooltipPrimitive.Provider>) {
  return <>{children}</>;
}

// Create a dummy tooltip that just renders its children
function Tooltip({ children, ...props }: React.ComponentProps<typeof TooltipPrimitive.Root>) {
  // Extract the trigger from children and render only that
  const trigger = React.Children.toArray(children).find(
    (child) => React.isValidElement(child) && child.type === TooltipTrigger
  );
  
  return <>{trigger}</>;
}

// Create a trigger that just renders its children
function TooltipTrigger({ children, ...props }: React.ComponentProps<typeof TooltipPrimitive.Trigger>) {
  return <>{children}</>;
}

// Create a content component that doesn't render anything
function TooltipContent({ children, ...props }: React.ComponentProps<typeof TooltipPrimitive.Content>) {
  return null;
}

export { Tooltip, TooltipTrigger, TooltipContent, TooltipProvider };
