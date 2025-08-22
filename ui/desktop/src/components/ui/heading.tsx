import React from 'react';
import { cn } from '../../utils';

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

export { Heading };
export type { HeadingProps };
