import { Message } from '../../types/message';
import type { View, ViewOptions } from '../../App';

export default function BottomMenu({
  setView,
  numTokens = 0,
  messages = [],
  isLoading = false,
  setMessages,
}: {
  setView: (view: View, viewOptions?: ViewOptions) => void;
  numTokens?: number;
  messages?: Message[];
  isLoading?: boolean;
  setMessages: (messages: Message[]) => void;
}) {
  // Suppress unused parameter warnings - these are kept for API compatibility
  // but the functionality has been moved to HeaderToolbar
  void setView;
  void numTokens;
  void messages;
  void isLoading;
  void setMessages;

  return (
    <div className="flex w-full justify-center items-center pb-2 transition-colors relative text-xs align-middle animate-in fade-in slide-in-from-right-8 duration-500">
      {/* Bottom menu is now simplified - status and summarize moved to HeaderToolbar */}
    </div>
  );
}
