import React from 'react';
import { cn } from '../../utils';

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

export { Text };
export type { TextProps };
