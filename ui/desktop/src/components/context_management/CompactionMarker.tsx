import React, { useState } from 'react';
import { Message, SummarizationRequestedContent, getTextContent } from '../../types/message';
import { Search } from 'lucide-react';
import { Button } from '../ui/button';
import SummaryViewModal from '../SummaryViewModal';

interface CompactionMarkerProps {
  message: Message;
}

export const CompactionMarker: React.FC<CompactionMarkerProps> = ({ message }) => {
  const [showSummaryModal, setShowSummaryModal] = useState(false);

  const compactionContent = message.content.find(
    (content) => content.type === 'summarizationRequested'
  ) as SummarizationRequestedContent | undefined;

  const markerText = compactionContent?.msg || 'Conversation compacted';

  // Extract the summary from the text content if it exists
  const textContent = getTextContent(message);
  const summaryMatch = textContent.match(/__SUMMARY__:\s*([\s\S]*)/);
  const summaryText = summaryMatch ? summaryMatch[1].trim() : null;

  return (
    <div className="flex items-center justify-between py-2">
      <div className="text-xs text-gray-400 text-left">{markerText}</div>
      {summaryText && (
        <Button
          onClick={() => setShowSummaryModal(true)}
          size="sm"
          variant="ghost"
          className="ml-2 text-xs"
        >
          <Search className="w-3 h-3 mr-1" />
          View Summary
        </Button>
      )}
      {showSummaryModal && summaryText && (
        <SummaryViewModal
          isOpen={showSummaryModal}
          onClose={() => setShowSummaryModal(false)}
          summaryText={summaryText}
        />
      )}
    </div>
  );
};
