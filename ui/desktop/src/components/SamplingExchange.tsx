import { useState } from 'react';
import { cn } from '../utils';
import { Sparkles, ChevronRight } from 'lucide-react';
import { Button } from './ui/button';
import MarkdownContent from './MarkdownContent';
import { Content } from '../types/message';

export interface SamplingExchangeData {
  extensionName: string;
  request: {
    messages: Array<{
      role: string;
      content: string | Content[];
    }>;
    systemPrompt?: string;
    maxTokens?: number;
    temperature?: number;
    model?: string;
  };
  response?: {
    content: string | Content[];
    model?: string;
    stopReason?: string;
  };
  error?: string;
  timestamp: number;
}

interface SamplingExchangeProps {
  exchange: SamplingExchangeData;
  isStreamingMessage?: boolean;
}

export default function SamplingExchange({
  exchange,
  isStreamingMessage = false,
}: SamplingExchangeProps) {
  const [isExpanded, setIsExpanded] = useState(false);

  const getContentText = (content: string | Content[]): string => {
    if (typeof content === 'string') return content;
    if (Array.isArray(content)) {
      return content
        .map((c) => {
          if (typeof c === 'string') return c;
          if (c.type === 'text' && 'text' in c) return c.text;
          return '';
        })
        .join('');
    }
    return '';
  };

  const formatMessages = (messages: Array<{ role: string; content: string | Content[] }>) => {
    return messages.map((msg, idx) => (
      <div key={idx} className="mb-2">
        <span className="font-semibold capitalize">{msg.role}:</span>{' '}
        <span className="text-textSubtle">{getContentText(msg.content)}</span>
      </div>
    ));
  };

  return (
    <div
      className={cn(
        'w-full text-sm font-sans rounded-lg overflow-hidden border-borderSubtle border bg-background-muted'
      )}
    >
      <Button
        onClick={() => setIsExpanded(!isExpanded)}
        className="group w-full flex justify-between items-center pr-2 transition-colors rounded-none"
        variant="ghost"
      >
        <span className="flex items-center gap-2 font-sans text-sm">
          <div className="relative">
            <Sparkles className={cn('w-4 h-4 text-purple-500', isStreamingMessage && 'animate-pulse')} />
          </div>
          <span>
            {exchange.extensionName} used the model
            {exchange.response && !exchange.error && ' ✓'}
            {exchange.error && ' ✗'}
            {isStreamingMessage && !exchange.response && '...'}
          </span>
        </span>
        <ChevronRight
          className={cn(
            'group-hover:opacity-100 transition-transform opacity-70',
            isExpanded && 'rotate-90'
          )}
        />
      </Button>

      {isExpanded && (
        <div className="border-t border-borderSubtle">
          {/* Request Section */}
          <div className="p-4 border-b border-borderSubtle">
            <h4 className="font-semibold text-sm mb-2 text-purple-700 dark:text-purple-300">
              Sampling Request
            </h4>
            
            {exchange.request.systemPrompt && (
              <div className="mb-3">
                <span className="font-semibold text-xs text-textSubtle">System Prompt:</span>
                <div className="mt-1 p-2 bg-background-subtle rounded text-xs">
                  {exchange.request.systemPrompt}
                </div>
              </div>
            )}

            <div className="mb-3">
              <span className="font-semibold text-xs text-textSubtle">Messages:</span>
              <div className="mt-1 p-2 bg-background-subtle rounded text-xs">
                {formatMessages(exchange.request.messages)}
              </div>
            </div>

            {(exchange.request.model || exchange.request.temperature !== undefined || exchange.request.maxTokens) && (
              <div className="flex gap-4 text-xs text-textSubtle">
                {exchange.request.model && (
                  <span>Model: {exchange.request.model}</span>
                )}
                {exchange.request.temperature !== undefined && (
                  <span>Temperature: {exchange.request.temperature}</span>
                )}
                {exchange.request.maxTokens && (
                  <span>Max Tokens: {exchange.request.maxTokens}</span>
                )}
              </div>
            )}
          </div>

          {/* Response Section */}
          {(exchange.response || exchange.error) && (
            <div className="p-4">
              <h4 className="font-semibold text-sm mb-2 text-purple-700 dark:text-purple-300">
                Model Response
              </h4>
              
              {exchange.error ? (
                <div className="p-2 bg-red-400/10 border border-red-500/20 rounded text-sm text-red-700 dark:text-red-300">
                  Error: {exchange.error}
                </div>
              ) : exchange.response ? (
                <>
                  <div className="mb-2">
                    <MarkdownContent
                      content={getContentText(exchange.response.content)}
                      className="whitespace-pre-wrap max-w-full overflow-x-auto"
                    />
                  </div>
                  
                  {(exchange.response.model || exchange.response.stopReason) && (
                    <div className="flex gap-4 text-xs text-textSubtle mt-2">
                      {exchange.response.model && (
                        <span>Model: {exchange.response.model}</span>
                      )}
                      {exchange.response.stopReason && (
                        <span>Stop Reason: {exchange.response.stopReason}</span>
                      )}
                    </div>
                  )}
                </>
              ) : null}
            </div>
          )}

          {/* Loading state */}
          {isStreamingMessage && !exchange.response && !exchange.error && (
            <div className="p-4 flex items-center gap-2">
              <span
                className="inline-block animate-spin rounded-full border-2 border-t-transparent border-purple-500"
                style={{ width: 12, height: 12 }}
                role="status"
                aria-label="Loading"
              />
              <span className="text-sm text-textSubtle">Waiting for model response...</span>
            </div>
          )}
        </div>
      )}
    </div>
  );
}
