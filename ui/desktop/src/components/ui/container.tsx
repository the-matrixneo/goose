import React from 'react';
import { cn } from '../../utils';

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

export { Container };
export type { ContainerProps };
