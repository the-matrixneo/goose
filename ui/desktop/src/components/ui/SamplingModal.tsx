import { BaseModal } from './BaseModal';
import { Button } from './button';
import MarkdownContent from '../MarkdownContent';
import { SamplingConfirmationRequest } from '../../api/types.gen';

interface SamplingModalProps {
  isOpen: boolean;
  request: SamplingConfirmationRequest | null;
  onApprove: (response?: string) => void;
  onDeny: () => void;
}

export function SamplingModal({
  isOpen,
  request,
  onApprove,
  onDeny,
}: SamplingModalProps) {
  if (!request) return null;

  return (
    <BaseModal
      isOpen={isOpen}
      title="MCP Sampling Request"
      actions={
        <div className="flex justify-end gap-2 p-4 border-t border-gray-200 dark:border-gray-700">
          <Button variant="outline" onClick={onDeny}>
            Deny
          </Button>
          <Button onClick={() => onApprove()}>
            Approve
          </Button>
        </div>
      }
    >
      <div className="space-y-4">
        <div className="text-sm text-gray-600 dark:text-gray-400">
          Extension: <strong className="text-gray-900 dark:text-gray-100">{request.extension_name}</strong>
        </div>
        
        <div className="bg-gray-50 dark:bg-gray-900 rounded-lg p-4 max-h-[400px] overflow-y-auto">
          <h4 className="font-medium mb-3 text-gray-900 dark:text-gray-100">Request Details:</h4>
          <div className="space-y-3">
            {request.messages.map((msg, idx) => (
              <div key={idx} className="border-l-2 border-gray-300 dark:border-gray-600 pl-3">
                <div className="font-semibold text-sm text-gray-700 dark:text-gray-300 mb-1">
                  {msg.role.charAt(0).toUpperCase() + msg.role.slice(1)}:
                </div>
                <div className="text-sm">
                  <MarkdownContent content={msg.content} />
                </div>
              </div>
            ))}
          </div>
        </div>

        {request.system_prompt && (
          <div className="text-sm bg-blue-50 dark:bg-blue-900/20 p-3 rounded-lg">
            <strong className="text-blue-900 dark:text-blue-300">System Prompt:</strong>
            <div className="mt-1 text-gray-700 dark:text-gray-300">{request.system_prompt}</div>
          </div>
        )}

        {request.model_preferences?.hints && request.model_preferences.hints.length > 0 && (
          <div className="text-sm">
            <strong className="text-gray-700 dark:text-gray-300">Model Preferences:</strong>{' '}
            <span className="text-gray-600 dark:text-gray-400">
              {request.model_preferences.hints.join(', ')}
            </span>
          </div>
        )}
      </div>
    </BaseModal>
  );
}
