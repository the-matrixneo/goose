import React, { useEffect } from 'react';
import { useLocation } from 'react-router-dom';
import GooseMessage from './GooseMessage';
import UserMessage from './UserMessage';
import { SearchView } from './conversation/SearchView';
import { AgentHeader } from './AgentHeader';
import LayingEggLoader from './LayingEggLoader';
import LoadingGoose from './LoadingGoose';
import { SessionSummaryModal } from './context_management/SessionSummaryModal';
import {
  ChatContextManagerProvider,
  useChatContextManager,
} from './context_management/ChatContextManager';
import { ContextHandler } from './context_management/ContextHandler';
import { type View, ViewOptions } from '../App';
import { MainPanelLayout } from './Layout/MainPanelLayout';
import ChatInput from './ChatInput';
import { useChatEngine } from '../hooks/useChatEngine';
import { useRecipeManager } from '../hooks/useRecipeManager';
import { useSessionContinuation } from '../hooks/useSessionContinuation';
import { useFileDrop } from '../hooks/useFileDrop';
import { useCostTracking } from '../hooks/useCostTracking';
import { Message } from '../types/message';

export interface ChatType {
  id: string;
  title: string;
  messageHistoryIndex: number;
  messages: Message[];
}

interface BaseChatProps {
  chat: ChatType;
  setChat: (chat: ChatType) => void;
  setView: (view: View, viewOptions?: ViewOptions) => void;
  setIsGoosehintsModalOpen?: (isOpen: boolean) => void;
  enableLocalStorage?: boolean;
  onMessageStreamFinish?: () => void;
  renderHeader?: () => React.ReactNode;
  renderBeforeMessages?: () => React.ReactNode;
  renderAfterMessages?: () => React.ReactNode;
  customChatInputProps?: Record<string, unknown>;
  customMainLayoutProps?: Record<string, unknown>;
  contentClassName?: string; // Add custom class for content area
}

function BaseChatContent({
  chat,
  setChat,
  setView,
  enableLocalStorage = false,
  onMessageStreamFinish,
  renderHeader,
  renderBeforeMessages,
  renderAfterMessages,
  customChatInputProps = {},
  customMainLayoutProps = {},
  contentClassName = "",
}: BaseChatProps) {
  const location = useLocation();

  // Get disableAnimation from location state
  const disableAnimation = location.state?.disableAnimation || false;

  const {
    summaryContent,
    summarizedThread,
    isSummaryModalOpen,
    resetMessagesWithSummary,
    closeSummaryModal,
    updateSummary,
    hasContextHandlerContent,
    getContextHandlerType,
  } = useChatContextManager();

  // Use shared chat engine
  const {
    messages,
    filteredMessages,
    ancestorMessages,
    setAncestorMessages,
    append,
    isLoading,
    error,
    setMessages,
    input: _input,
    setInput: _setInput,
    handleSubmit: engineHandleSubmit,
    onStopGoose,
    sessionTokenCount,
    sessionInputTokens,
    sessionOutputTokens,
    localInputTokens,
    localOutputTokens,
    commandHistory,
    toolCallNotifications,
    updateMessageStreamBody,
    sessionMetadata,
    isUserMessage,
  } = useChatEngine({
    chat,
    setChat,
    onMessageStreamFinish,
    enableLocalStorage,
  });

  // Use shared recipe manager
  const { recipeConfig, initialPrompt, isGeneratingRecipe } = useRecipeManager(messages);

  // Use shared session continuation
  useSessionContinuation({
    chat,
    setChat,
    summarizedThread,
    updateMessageStreamBody,
  });

  // Use shared file drop
  const { droppedFiles, handleDrop, handleDragOver } = useFileDrop();

  // Use shared cost tracking
  const { sessionCosts } = useCostTracking({
    sessionInputTokens,
    sessionOutputTokens,
    localInputTokens,
    localOutputTokens,
    sessionMetadata,
  });

  useEffect(() => {
    // Log all messages when the component first mounts
    window.electron.logInfo(
      'Initial messages when resuming session: ' + JSON.stringify(chat.messages, null, 2)
    );
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []); // Empty dependency array means this runs once on mount

  // Handle submit with summary reset support
  const handleSubmit = (e: React.FormEvent) => {
    const customEvent = e as unknown as CustomEvent;
    const combinedTextFromInput = customEvent.detail?.value || '';

    const onSummaryReset =
      summarizedThread.length > 0
        ? () => {
            resetMessagesWithSummary(
              messages,
              setMessages,
              ancestorMessages,
              setAncestorMessages,
              summaryContent
            );
          }
        : undefined;

    engineHandleSubmit(combinedTextFromInput, onSummaryReset);
  };

  return (
    <div>
      <MainPanelLayout {...customMainLayoutProps}>
        {/* Loader when generating recipe */}
        {isGeneratingRecipe && <LayingEggLoader />}

        {/* Custom header */}
        {renderHeader && renderHeader()}

        <div
          className={`flex flex-col min-w-0 flex-1 overflow-y-scroll relative ${contentClassName}`}
          onDrop={handleDrop}
          onDragOver={handleDragOver}
        >
          {/* Custom content before messages */}
          {renderBeforeMessages && renderBeforeMessages()}

          {/* Recipe agent header */}
          {recipeConfig?.title && messages.length > 0 && (
            <AgentHeader
              title={recipeConfig.title}
              profileInfo={
                recipeConfig.profile
                  ? `${recipeConfig.profile} - ${recipeConfig.mcps || 12} MCPs`
                  : undefined
              }
              onChangeProfile={() => {
                console.log('Change profile clicked');
              }}
            />
          )}

          {/* Messages */}
          {messages.length > 0 && (
            <>
              <SearchView>
                {filteredMessages.map((message, index) => {
                  const isUser = isUserMessage(message);

                  return (
                    <div
                      key={message.id || index}
                      className={`relative ${index === 0 ? 'mt-0' : 'mt-4'} ${isUser ? 'user' : 'assistant'}`}
                      data-testid="message-container"
                    >
                      {isUser ? (
                        <>
                          {hasContextHandlerContent(message) ? (
                            <ContextHandler
                              messages={messages}
                              messageId={message.id ?? message.created.toString()}
                              chatId={chat.id}
                              workingDir={window.appConfig.get('GOOSE_WORKING_DIR') as string}
                              contextType={getContextHandlerType(message)}
                            />
                          ) : (
                            <UserMessage message={message} />
                          )}
                        </>
                      ) : (
                        <>
                          {hasContextHandlerContent(message) ? (
                            <ContextHandler
                              messages={messages}
                              messageId={message.id ?? message.created.toString()}
                              chatId={chat.id}
                              workingDir={window.appConfig.get('GOOSE_WORKING_DIR') as string}
                              contextType={getContextHandlerType(message)}
                            />
                          ) : (
                            <GooseMessage
                              messageHistoryIndex={chat?.messageHistoryIndex}
                              message={message}
                              messages={messages}
                              append={append}
                              appendMessage={(newMessage) => {
                                const updatedMessages = [...messages, newMessage];
                                setMessages(updatedMessages);
                              }}
                              toolCallNotifications={toolCallNotifications}
                            />
                          )}
                        </>
                      )}
                    </div>
                  );
                })}
              </SearchView>

              {error && (
                <div className="flex flex-col items-center justify-center p-4">
                  <div className="text-red-700 dark:text-red-300 bg-red-400/50 p-3 rounded-lg mb-2">
                    {error.message || 'Honk! Goose experienced an error while responding'}
                  </div>
                  <div
                    className="px-3 py-2 mt-2 text-center whitespace-nowrap cursor-pointer text-textStandard border border-borderSubtle hover:bg-bgSubtle rounded-full inline-block transition-all duration-150"
                    onClick={async () => {
                      // Find the last user message
                      const lastUserMessage = messages.reduceRight(
                        (found, m) => found || (m.role === 'user' ? m : null),
                        null as Message | null
                      );
                      if (lastUserMessage) {
                        append(lastUserMessage);
                      }
                    }}
                  >
                    Retry Last Message
                  </div>
                </div>
              )}
              <div className="block h-8" />
            </>
          )}

          {/* Custom content after messages */}
          {renderAfterMessages && renderAfterMessages()}
        </div>

        <div
          className={`relative z-10 ${disableAnimation ? '' : 'animate-[fadein_400ms_ease-in_forwards]'}`}
        >
          <div className="px-6">{isLoading && <LoadingGoose />}</div>
          <ChatInput
            handleSubmit={handleSubmit}
            isLoading={isLoading}
            onStop={onStopGoose}
            commandHistory={commandHistory}
            initialValue={_input || initialPrompt}
            setView={setView}
            numTokens={sessionTokenCount}
            inputTokens={sessionInputTokens || localInputTokens}
            outputTokens={sessionOutputTokens || localOutputTokens}
            droppedFiles={droppedFiles}
            messages={messages}
            setMessages={setMessages}
            disableAnimation={disableAnimation}
            sessionCosts={sessionCosts}
            {...customChatInputProps}
          />
        </div>
      </MainPanelLayout>

      <SessionSummaryModal
        isOpen={isSummaryModalOpen}
        onClose={closeSummaryModal}
        onSave={(editedContent) => {
          updateSummary(editedContent);
          closeSummaryModal();
        }}
        summaryContent={summaryContent}
      />
    </div>
  );
}

export default function BaseChat(props: BaseChatProps) {
  return (
    <ChatContextManagerProvider>
      <BaseChatContent {...props} />
    </ChatContextManagerProvider>
  );
}
