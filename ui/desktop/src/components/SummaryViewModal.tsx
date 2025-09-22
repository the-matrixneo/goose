import React from 'react';
import { X, FileText } from 'lucide-react';
import { Message, SummarizationRequestedContent } from '../types/message';
import MarkdownContent from './MarkdownContent';
import { formatMessageTimestamp } from '../utils/timeUtils';

interface SummaryViewModalProps {
  isOpen: boolean;
  onClose: () => void;
  messages: Message[];
}

export const SummaryViewModal: React.FC<SummaryViewModalProps> = ({
  isOpen,
  onClose,
  messages,
}) => {
  if (!isOpen) return null;

  // Find the most recent summary message
  // It should be agent visible but not user visible (the actual summary)
  // OR user visible but not agent visible (the compaction marker)
  const findLatestSummary = (): {
    message: Message;
    content: SummarizationRequestedContent;
  } | null => {
    // First, try to find the actual summary (agent visible but not user visible)
    for (let i = messages.length - 1; i >= 0; i--) {
      const msg = messages[i];
      if (msg.metadata?.agentVisible === true && msg.metadata?.userVisible === false) {
        // Check if it contains text that looks like a summary
        const textContent = msg.content.find((c) => c.type === 'text');
        if (textContent && 'text' in textContent && textContent.text.includes('summary')) {
          return {
            message: msg,
            content: {
              type: 'summarizationRequested',
              msg: textContent.text,
            },
          };
        }
      }
    }

    // Fallback: Look for summarization requested content in any message
    for (let i = messages.length - 1; i >= 0; i--) {
      const msg = messages[i];
      const summaryContent = msg.content.find((c) => c.type === 'summarizationRequested');
      if (summaryContent && 'msg' in summaryContent) {
        return {
          message: msg,
          content: summaryContent as SummarizationRequestedContent,
        };
      }
    }

    return null;
  };

  const summaryData = findLatestSummary();

  return (
    <div className="fixed inset-0 z-[300] flex items-center justify-center bg-black/50 animate-fadeIn">
      <div className="bg-background-default border border-borderSubtle rounded-lg w-[90vw] max-w-4xl max-h-[80vh] flex flex-col animate-slideUp">
        {/* Header */}
        <div className="flex items-center justify-between p-6 border-b border-borderSubtle">
          <div className="flex items-center gap-3">
            <FileText className="w-5 h-5 text-textStandard" />
            <h2 className="text-xl font-medium text-textProminent">Latest Conversation Summary</h2>
          </div>
          <button
            onClick={onClose}
            className="p-2 hover:bg-bgSubtle rounded-lg transition-colors"
            aria-label="Close modal"
          >
            <X className="w-5 h-5 text-textSubtle" />
          </button>
        </div>

        {/* Content */}
        <div className="flex-1 overflow-y-auto p-6">
          {summaryData ? (
            <div className="space-y-4">
              <div className="flex items-center gap-2 text-sm text-textSubtle">
                <span>Created:</span>
                <span className="font-mono">
                  {formatMessageTimestamp(summaryData.message.created)}
                </span>
              </div>

              <div className="prose prose-sm dark:prose-invert max-w-none">
                <MarkdownContent content={summaryData.content.msg} />
              </div>
            </div>
          ) : (
            <div className="flex flex-col items-center justify-center py-12 text-textSubtle">
              <FileText className="w-12 h-12 mb-4 opacity-50" />
              <p className="text-lg mb-2">No Summary Available</p>
              <p className="text-sm text-center max-w-md">
                Summaries are automatically created when the conversation context window reaches its
                limit and needs to be compacted.
              </p>
            </div>
          )}
        </div>

        {/* Footer */}
        <div className="flex justify-end gap-3 p-6 border-t border-borderSubtle">
          <button
            onClick={onClose}
            className="px-4 py-2 bg-bgSubtle text-textStandard rounded-lg hover:bg-bgSecondary transition-colors"
          >
            Close
          </button>
        </div>
      </div>
    </div>
  );
};

export default SummaryViewModal;
