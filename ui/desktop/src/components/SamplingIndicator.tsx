import React from 'react';
import { cn } from '../utils';
import { Sparkles } from 'lucide-react';

export interface SamplingIndicatorProps {
  isActive: boolean;
  extensionName?: string;
  className?: string;
}

/**
 * Visual indicator that shows when an MCP server is using the model connection
 * to generate responses through sampling.
 */
export const SamplingIndicator: React.FC<SamplingIndicatorProps> = ({
  isActive,
  extensionName,
  className,
}) => {
  if (!isActive) return null;

  return (
    <div
      className={cn(
        'flex items-center gap-2 px-3 py-1.5 rounded-md',
        'bg-purple-500/10 border border-purple-500/20',
        'animate-in fade-in slide-in-from-top-1 duration-300',
        className
      )}
    >
      <Sparkles className="w-4 h-4 text-purple-500 animate-pulse" />
      <span className="text-sm text-purple-700 dark:text-purple-300">
        {extensionName ? (
          <>
            <span className="font-medium">{extensionName}</span> is using the model
          </>
        ) : (
          'MCP server is using the model'
        )}
      </span>
    </div>
  );
};

/**
 * Inline sampling badge that can be shown next to tool names
 */
export const SamplingBadge: React.FC<{ className?: string }> = ({ className }) => {
  return (
    <span
      className={cn(
        'inline-flex items-center gap-1 px-1.5 py-0.5 rounded text-xs',
        'bg-purple-500/10 text-purple-700 dark:text-purple-300',
        'border border-purple-500/20',
        className
      )}
      title="This extension is using the model to generate a response"
    >
      <Sparkles className="w-3 h-3" />
      <span>sampling</span>
    </span>
  );
};
