import React, { useState } from 'react';
import { Message, SummarizationRequestedContent } from '../../types/message';
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
  const summaryText = compactionContent?.summary || null;

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
