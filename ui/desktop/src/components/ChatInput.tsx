import React, { useRef, useState, useEffect, useMemo } from 'react';
import { Button } from './ui/button';
import type { View } from '../App';
import Stop from './ui/Stop';
import { Attach, Send, Close } from './icons';
import { debounce } from 'lodash';
import { LocalMessageStorage } from '../utils/localMessageStorage';
import { Message } from '../types/message';
import { DirSwitcher } from './bottom_menu/DirSwitcher';
import BottomMenuAlertPopover from './bottom_menu/BottomMenuAlertPopover';
import ModelsBottomBar from './settings/models/bottom_bar/ModelsBottomBar';
import { BottomMenuModeSelection } from './bottom_menu/BottomMenuModeSelection';
import { ManualSummarizeButton } from './context_management/ManualSummaryButton';
import { useAlerts } from './alerts';
import { useToolCount } from './alerts/useToolCount';

interface PastedImage {
  id: string;
  dataUrl: string; // For immediate preview
  filePath?: string; // Path on filesystem after saving
  isLoading: boolean;
  error?: string;
}

// Constants for image handling
const MAX_IMAGES_PER_MESSAGE = 5;
const MAX_IMAGE_SIZE_MB = 5;

interface ChatInputProps {
  handleSubmit: (e: React.FormEvent) => void;
  isLoading?: boolean;
  onStop?: () => void;
  commandHistory?: string[]; // Current chat's message history
  initialValue?: string;
  droppedFiles?: string[];
  setView: (view: View) => void;
  numTokens?: number;
  hasMessages?: boolean;
  messages?: Message[];
  setMessages: (messages: Message[]) => void;
  disableAnimation?: boolean;
}

export default function ChatInput({
  handleSubmit,
  isLoading = false,
  onStop,
  commandHistory = [],
  initialValue = '',
  droppedFiles = [],
  setView,
  numTokens,
  hasMessages,
  messages = [],
  setMessages,
  disableAnimation = false,
}: ChatInputProps) {
  const [_value, setValue] = useState(initialValue);
  const [displayValue, setDisplayValue] = useState(initialValue); // For immediate visual feedback
  const [isFocused, setIsFocused] = useState(false);
  const [pastedImages, setPastedImages] = useState<PastedImage[]>([]);
  const { alerts, addAlert, clearAlerts } = useAlerts();
  const dropdownRef = useRef<HTMLDivElement>(null);
  const toolCount = useToolCount();

  // Update internal value when initialValue changes
  useEffect(() => {
    setValue(initialValue);
    setDisplayValue(initialValue);

    // Use a functional update to get the current pastedImages
    // and perform cleanup. This avoids needing pastedImages in the deps.
    setPastedImages((currentPastedImages) => {
      currentPastedImages.forEach((img) => {
        if (img.filePath) {
          window.electron.deleteTempFile(img.filePath);
        }
      });
      return []; // Return a new empty array
    });

    // Reset history index when input is cleared
    setHistoryIndex(-1);
    setIsInGlobalHistory(false);
  }, [initialValue]); // Keep only initialValue as a dependency

  // State to track if the IME is composing (i.e., in the middle of Japanese IME input)
  const [isComposing, setIsComposing] = useState(false);
  const [historyIndex, setHistoryIndex] = useState(-1);
  const [savedInput, setSavedInput] = useState('');
  const [isInGlobalHistory, setIsInGlobalHistory] = useState(false);
  const textAreaRef = useRef<HTMLTextAreaElement>(null);
  const [processedFilePaths, setProcessedFilePaths] = useState<string[]>([]);

  const handleRemovePastedImage = (idToRemove: string) => {
    const imageToRemove = pastedImages.find((img) => img.id === idToRemove);
    if (imageToRemove?.filePath) {
      window.electron.deleteTempFile(imageToRemove.filePath);
    }
    setPastedImages((currentImages) => currentImages.filter((img) => img.id !== idToRemove));
  };

  const handleRetryImageSave = async (imageId: string) => {
    const imageToRetry = pastedImages.find((img) => img.id === imageId);
    if (!imageToRetry || !imageToRetry.dataUrl) return;

    // Set the image to loading state
    setPastedImages((prev) =>
      prev.map((img) => (img.id === imageId ? { ...img, isLoading: true, error: undefined } : img))
    );

    try {
      const result = await window.electron.saveDataUrlToTemp(imageToRetry.dataUrl, imageId);
      setPastedImages((prev) =>
        prev.map((img) =>
          img.id === result.id
            ? { ...img, filePath: result.filePath, error: result.error, isLoading: false }
            : img
        )
      );
    } catch (err) {
      console.error('Error retrying image save:', err);
      setPastedImages((prev) =>
        prev.map((img) =>
          img.id === imageId
            ? { ...img, error: 'Failed to save image via Electron.', isLoading: false }
            : img
        )
      );
    }
  };

  useEffect(() => {
    if (textAreaRef.current) {
      textAreaRef.current.focus();
    }
  }, []);

  const minHeight = '1rem';
  const maxHeight = 10 * 24;

  // If we have dropped files, add them to the input and update our state.
  useEffect(() => {
    if (processedFilePaths !== droppedFiles && droppedFiles.length > 0) {
      // Append file paths that aren't in displayValue.
      const currentText = displayValue || '';
      const joinedPaths = currentText.trim()
        ? `${currentText.trim()} ${droppedFiles.filter((path) => !currentText.includes(path)).join(' ')}`
        : droppedFiles.join(' ');

      setDisplayValue(joinedPaths);
      setValue(joinedPaths);
      textAreaRef.current?.focus();
      setProcessedFilePaths(droppedFiles);
    }
  }, [droppedFiles, processedFilePaths, displayValue]);

  // Debounced function to update actual value
  const debouncedSetValue = useMemo(
    () =>
      debounce((value: string) => {
        setValue(value);
      }, 150),
    [setValue]
  );

  // Debounced autosize function
  const debouncedAutosize = useMemo(
    () =>
      debounce((element: HTMLTextAreaElement) => {
        element.style.height = '0px'; // Reset height
        const scrollHeight = element.scrollHeight;
        element.style.height = Math.min(scrollHeight, maxHeight) + 'px';
      }, 150),
    [maxHeight]
  );

  useEffect(() => {
    if (textAreaRef.current) {
      debouncedAutosize(textAreaRef.current);
    }
  }, [debouncedAutosize, displayValue]);

  const handleChange = (evt: React.ChangeEvent<HTMLTextAreaElement>) => {
    const val = evt.target.value;
    setDisplayValue(val); // Update display immediately
    debouncedSetValue(val); // Debounce the actual state update
  };

  const handlePaste = async (evt: React.ClipboardEvent<HTMLTextAreaElement>) => {
    const files = Array.from(evt.clipboardData.files || []);
    const imageFiles = files.filter((file) => file.type.startsWith('image/'));

    if (imageFiles.length === 0) return;

    // Check if adding these images would exceed the limit
    if (pastedImages.length + imageFiles.length > MAX_IMAGES_PER_MESSAGE) {
      // Show error message to user
      setPastedImages((prev) => [
        ...prev,
        {
          id: `error-${Date.now()}`,
          dataUrl: '',
          isLoading: false,
          error: `Cannot paste ${imageFiles.length} image(s). Maximum ${MAX_IMAGES_PER_MESSAGE} images per message allowed.`,
        },
      ]);

      // Remove the error message after 3 seconds
      setTimeout(() => {
        setPastedImages((prev) => prev.filter((img) => !img.id.startsWith('error-')));
      }, 3000);

      return;
    }

    evt.preventDefault();

    for (const file of imageFiles) {
      // Check individual file size before processing
      if (file.size > MAX_IMAGE_SIZE_MB * 1024 * 1024) {
        const errorId = `error-${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;
        setPastedImages((prev) => [
          ...prev,
          {
            id: errorId,
            dataUrl: '',
            isLoading: false,
            error: `Image too large (${Math.round(file.size / (1024 * 1024))}MB). Maximum ${MAX_IMAGE_SIZE_MB}MB allowed.`,
          },
        ]);

        // Remove the error message after 3 seconds
        setTimeout(() => {
          setPastedImages((prev) => prev.filter((img) => img.id !== errorId));
        }, 3000);

        continue;
      }

      const reader = new FileReader();
      reader.onload = async (e) => {
        const dataUrl = e.target?.result as string;
        if (dataUrl) {
          const imageId = `img-${Date.now()}-${Math.random().toString(36).substring(2, 9)}`;
          setPastedImages((prev) => [...prev, { id: imageId, dataUrl, isLoading: true }]);

          try {
            const result = await window.electron.saveDataUrlToTemp(dataUrl, imageId);
            setPastedImages((prev) =>
              prev.map((img) =>
                img.id === result.id
                  ? { ...img, filePath: result.filePath, error: result.error, isLoading: false }
                  : img
              )
            );
          } catch (err) {
            console.error('Error saving pasted image:', err);
            setPastedImages((prev) =>
              prev.map((img) =>
                img.id === imageId
                  ? { ...img, error: 'Failed to save image via Electron.', isLoading: false }
                  : img
              )
            );
          }
        }
      };
      reader.readAsDataURL(file);
    }
  };

  // Cleanup debounced functions on unmount
  useEffect(() => {
    return () => {
      debouncedSetValue.cancel?.();
      debouncedAutosize.cancel?.();
    };
  }, [debouncedSetValue, debouncedAutosize]);

  // Handlers for composition events, which are crucial for proper IME behavior
  const handleCompositionStart = () => {
    setIsComposing(true);
  };

  const handleCompositionEnd = () => {
    setIsComposing(false);
  };

  const handleHistoryNavigation = (evt: React.KeyboardEvent<HTMLTextAreaElement>) => {
    const isUp = evt.key === 'ArrowUp';
    const isDown = evt.key === 'ArrowDown';

    // Only handle up/down keys with Cmd/Ctrl modifier
    if ((!isUp && !isDown) || !(evt.metaKey || evt.ctrlKey) || evt.altKey || evt.shiftKey) {
      return;
    }

    evt.preventDefault();

    // Get global history once to avoid multiple calls
    const globalHistory = LocalMessageStorage.getRecentMessages() || [];

    // Save current input if we're just starting to navigate history
    if (historyIndex === -1) {
      setSavedInput(displayValue || '');
      setIsInGlobalHistory(commandHistory.length === 0);
    }

    // Determine which history we're using
    const currentHistory = isInGlobalHistory ? globalHistory : commandHistory;
    let newIndex = historyIndex;
    let newValue = '';

    // Handle navigation
    if (isUp) {
      // Moving up through history
      if (newIndex < currentHistory.length - 1) {
        // Still have items in current history
        newIndex = historyIndex + 1;
        newValue = currentHistory[newIndex];
      } else if (!isInGlobalHistory && globalHistory.length > 0) {
        // Switch to global history
        setIsInGlobalHistory(true);
        newIndex = 0;
        newValue = globalHistory[newIndex];
      }
    } else {
      // Moving down through history
      if (newIndex > 0) {
        // Still have items in current history
        newIndex = historyIndex - 1;
        newValue = currentHistory[newIndex];
      } else if (isInGlobalHistory && commandHistory.length > 0) {
        // Switch to chat history
        setIsInGlobalHistory(false);
        newIndex = commandHistory.length - 1;
        newValue = commandHistory[newIndex];
      } else {
        // Return to original input
        newIndex = -1;
        newValue = savedInput;
      }
    }

    // Update display if we have a new value
    if (newIndex !== historyIndex) {
      setHistoryIndex(newIndex);
      if (newIndex === -1) {
        setDisplayValue(savedInput || '');
        setValue(savedInput || '');
      } else {
        setDisplayValue(newValue || '');
        setValue(newValue || '');
      }
    }
  };

  const performSubmit = () => {
    const validPastedImageFilesPaths = pastedImages
      .filter((img) => img.filePath && !img.error && !img.isLoading)
      .map((img) => img.filePath as string);

    let textToSend = displayValue.trim();

    if (validPastedImageFilesPaths.length > 0) {
      const pathsString = validPastedImageFilesPaths.join(' ');
      textToSend = textToSend ? `${textToSend} ${pathsString}` : pathsString;
    }

    if (textToSend) {
      if (displayValue.trim()) {
        LocalMessageStorage.addMessage(displayValue);
      } else if (validPastedImageFilesPaths.length > 0) {
        LocalMessageStorage.addMessage(validPastedImageFilesPaths.join(' '));
      }

      handleSubmit(
        new CustomEvent('submit', { detail: { value: textToSend } }) as unknown as React.FormEvent
      );

      setDisplayValue('');
      setValue('');
      setPastedImages([]);
      setHistoryIndex(-1);
      setSavedInput('');
      setIsInGlobalHistory(false);
    }
  };

  const handleKeyDown = (evt: React.KeyboardEvent<HTMLTextAreaElement>) => {
    // Handle history navigation first
    handleHistoryNavigation(evt);

    if (evt.key === 'Enter') {
      // should not trigger submit on Enter if it's composing (IME input in progress) or shift/alt(option) is pressed
      if (evt.shiftKey || isComposing) {
        // Allow line break for Shift+Enter, or during IME composition
        return;
      }

      if (evt.altKey) {
        const newValue = displayValue + '\n';
        setDisplayValue(newValue);
        setValue(newValue);
        return;
      }

      evt.preventDefault();
      const canSubmit =
        !isLoading &&
        (displayValue.trim() ||
          pastedImages.some((img) => img.filePath && !img.error && !img.isLoading));
      if (canSubmit) {
        performSubmit();
      }
    }
  };

  const onFormSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    const canSubmit =
      !isLoading &&
      (displayValue.trim() ||
        pastedImages.some((img) => img.filePath && !img.error && !img.isLoading));
    if (canSubmit) {
      performSubmit();
    }
  };

  const handleFileSelect = async () => {
    const path = await window.electron.selectFileOrDirectory();
    if (path) {
      const newValue = displayValue.trim() ? `${displayValue.trim()} ${path}` : path;
      setDisplayValue(newValue);
      setValue(newValue);
      textAreaRef.current?.focus();
    }
  };

  const hasSubmittableContent =
    displayValue.trim() || pastedImages.some((img) => img.filePath && !img.error && !img.isLoading);
  const isAnyImageLoading = pastedImages.some((img) => img.isLoading);

  return (
    <div
      className={`flex flex-col relative h-auto rounded-lg border ml-4 mr-6 mb-6 transition-colors ${
        disableAnimation ? '' : 'animate-in fade-in slide-in-from-right-8 duration-500'
      } z-10`}
    >
      {/* DirSwitcher at the top */}
      <div className="p-2 pb-0">
        <DirSwitcher hasMessages={messages.length > 0} />
      </div>
      <form onSubmit={onFormSubmit} className="flex flex-col">
        <textarea
          data-testid="chat-input"
          autoFocus
          id="dynamic-textarea"
          placeholder="What would you like to pair on? ⌘↑/⌘↓"
          value={displayValue}
          onChange={handleChange}
          onCompositionStart={handleCompositionStart}
          onCompositionEnd={handleCompositionEnd}
          onKeyDown={handleKeyDown}
          onPaste={handlePaste}
          onFocus={() => setIsFocused(true)}
          onBlur={() => setIsFocused(false)}
          ref={textAreaRef}
          rows={1}
          style={{
            minHeight: `${minHeight}px`,
            maxHeight: `${maxHeight}px`,
            overflowY: 'auto',
          }}
          className="w-full outline-none border-none focus:ring-0 bg-transparent px-3 pt-3 pb-1.5 text-sm resize-none text-textStandard placeholder:text-textPlaceholder"
        />

        {pastedImages.length > 0 && (
          <div className="flex flex-wrap gap-2 p-2 border-t border-borderSubtle">
            {pastedImages.map((img) => (
              <div key={img.id} className="relative group w-20 h-20">
                {img.dataUrl && (
                  <img
                    src={img.dataUrl}
                    alt={`Pasted image ${img.id}`}
                    className={`w-full h-full object-cover rounded border ${img.error ? 'border-red-500' : 'border-borderStandard'}`}
                  />
                )}
                {img.isLoading && (
                  <div className="absolute inset-0 flex items-center justify-center bg-black bg-opacity-50 rounded">
                    <div className="animate-spin rounded-full h-6 w-6 border-t-2 border-b-2 border-white"></div>
                  </div>
                )}
                {img.error && !img.isLoading && (
                  <div className="absolute inset-0 flex flex-col items-center justify-center bg-black bg-opacity-75 rounded p-1 text-center">
                    <p className="text-red-400 text-[10px] leading-tight break-all mb-1">
                      {img.error.substring(0, 50)}
                    </p>
                    {img.dataUrl && (
                      <Button
                        type="button"
                        onClick={() => handleRetryImageSave(img.id)}
                        title="Retry saving image"
                        variant="outline"
                        size="xs"
                      >
                        Retry
                      </Button>
                    )}
                  </div>
                )}
                {!img.isLoading && (
                  <Button
                    type="button"
                    shape="round"
                    onClick={() => handleRemovePastedImage(img.id)}
                    className="absolute -top-1 -right-1 opacity-0 group-hover:opacity-100 focus:opacity-100 transition-opacity z-10"
                    aria-label="Remove image"
                    variant="outline"
                    size="xs"
                  >
                    <Close />
                  </Button>
                )}
              </div>
            ))}
          </div>
        )}

        {/* Actions and model/mode/alerts row below input */}
        <div className="flex flex-row items-center gap-1 p-2">
          {/* Send/Attach/Stop actions */}
          <Button
            type="button"
            size="xs"
            variant="outline"
            className="text-text-muted"
            onClick={handleFileSelect}
          >
            <Attach />
          </Button>
          {isLoading ? (
            <Button
              type="button"
              onClick={onStop}
              size="xs"
              variant="outline"
              className="text-text-muted"
            >
              <Stop />
            </Button>
          ) : (
            <Button type="submit" size="xs" variant="outline" className="text-text-muted">
              <Send />
            </Button>
          )}

          {/* Model selector, mode selector, alerts, summarize button */}
          <div className="flex flex-row items-center ml-2">
            <ModelsBottomBar dropdownRef={dropdownRef} setView={setView} />
            <div className="w-px h-4 bg-border-default mx-2"></div>
            <BottomMenuModeSelection setView={setView} />
            <BottomMenuAlertPopover alerts={alerts} />
            {messages.length > 0 && (
              <ManualSummarizeButton
                messages={messages}
                isLoading={isLoading}
                setMessages={setMessages}
              />
            )}
          </div>
        </div>
      </form>
    </div>
  );
}
