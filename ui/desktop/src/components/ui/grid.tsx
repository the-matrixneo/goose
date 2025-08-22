import React from 'react';
import { cn } from '../../utils';

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

export { Grid };
export type { GridProps };
