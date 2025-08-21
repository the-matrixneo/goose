import React, { useState, useEffect, useCallback, useRef } from 'react';
import { Button } from '../../ui/button';
import { Card } from '../../ui/card';
import { Upload, X, RotateCcw, Loader2 } from 'lucide-react';
import { Switch } from '../../ui/switch';

interface BackgroundOption {
  id: string;
  name: string;
  type: 'gradient' | 'image' | 'solid';
  value: string;
  preview?: string;
  gradient?: string;
  color?: string;
}

const DEFAULT_BACKGROUNDS: BackgroundOption[] = [
  {
    id: 'default-gradient',
    name: 'Default Gradient',
    type: 'gradient',
    value: `
      radial-gradient(circle at 20% 80%, rgba(100, 100, 110, 0.25) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(120, 120, 125, 0.22) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(90, 95, 100, 0.18) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(0, 0, 0, 0.02) 0%, 
        rgba(0, 0, 0, 0.04) 25%, 
        rgba(0, 0, 0, 0.02) 50%, 
        rgba(0, 0, 0, 0.06) 75%, 
        rgba(0, 0, 0, 0.03) 100%
      )
    `,
    gradient: `
      radial-gradient(circle at 20% 80%, rgba(100, 100, 110, 0.25) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(120, 120, 125, 0.22) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(90, 95, 100, 0.18) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(0, 0, 0, 0.02) 0%, 
        rgba(0, 0, 0, 0.04) 25%, 
        rgba(0, 0, 0, 0.02) 50%, 
        rgba(0, 0, 0, 0.06) 75%, 
        rgba(0, 0, 0, 0.03) 100%
      )
    `,
  },
  {
    id: 'blue-gradient',
    name: 'Ocean Blue',
    type: 'gradient',
    value: `
      radial-gradient(circle at 20% 80%, rgba(59, 130, 246, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(99, 102, 241, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(147, 197, 253, 0.08) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(59, 130, 246, 0.02) 0%, 
        rgba(99, 102, 241, 0.04) 25%, 
        rgba(147, 197, 253, 0.02) 50%, 
        rgba(59, 130, 246, 0.06) 75%, 
        rgba(99, 102, 241, 0.03) 100%
      )
    `,
    gradient: `
      radial-gradient(circle at 20% 80%, rgba(59, 130, 246, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(99, 102, 241, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(147, 197, 253, 0.08) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(59, 130, 246, 0.02) 0%, 
        rgba(99, 102, 241, 0.04) 25%, 
        rgba(147, 197, 253, 0.02) 50%, 
        rgba(59, 130, 246, 0.06) 75%, 
        rgba(99, 102, 241, 0.03) 100%
      )
    `,
  },
  {
    id: 'purple-gradient',
    name: 'Purple Haze',
    type: 'gradient',
    value: `
      radial-gradient(circle at 20% 80%, rgba(147, 51, 234, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(168, 85, 247, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(196, 181, 253, 0.08) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(147, 51, 234, 0.02) 0%, 
        rgba(168, 85, 247, 0.04) 25%, 
        rgba(196, 181, 253, 0.02) 50%, 
        rgba(147, 51, 234, 0.06) 75%, 
        rgba(168, 85, 247, 0.03) 100%
      )
    `,
    gradient: `
      radial-gradient(circle at 20% 80%, rgba(147, 51, 234, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(168, 85, 247, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(196, 181, 253, 0.08) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(147, 51, 234, 0.02) 0%, 
        rgba(168, 85, 247, 0.04) 25%, 
        rgba(196, 181, 253, 0.02) 50%, 
        rgba(147, 51, 234, 0.06) 75%, 
        rgba(168, 85, 247, 0.03) 100%
      )
    `,
  },
  {
    id: 'green-gradient',
    name: 'Forest Green',
    type: 'gradient',
    value: `
      radial-gradient(circle at 20% 80%, rgba(34, 197, 94, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(22, 163, 74, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(134, 239, 172, 0.08) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(34, 197, 94, 0.02) 0%, 
        rgba(22, 163, 74, 0.04) 25%, 
        rgba(134, 239, 172, 0.02) 50%, 
        rgba(34, 197, 94, 0.06) 75%, 
        rgba(22, 163, 74, 0.03) 100%
      )
    `,
    gradient: `
      radial-gradient(circle at 20% 80%, rgba(34, 197, 94, 0.15) 0%, transparent 50%),
      radial-gradient(circle at 80% 20%, rgba(22, 163, 74, 0.12) 0%, transparent 50%),
      radial-gradient(circle at 40% 40%, rgba(134, 239, 172, 0.08) 0%, transparent 50%),
      linear-gradient(135deg, 
        rgba(34, 197, 94, 0.02) 0%, 
        rgba(22, 163, 74, 0.04) 25%, 
        rgba(134, 239, 172, 0.02) 50%, 
        rgba(34, 197, 94, 0.06) 75%, 
        rgba(22, 163, 74, 0.03) 100%
      )
    `,
  },
  {
    id: 'solid-dark',
    name: 'Dark',
    type: 'solid',
    value: 'rgba(0, 0, 0, 0.1)',
    color: 'rgba(0, 0, 0, 0.1)',
  },
  {
    id: 'solid-light',
    name: 'Light',
    type: 'solid',
    value: 'rgba(255, 255, 255, 0.05)',
    color: 'rgba(255, 255, 255, 0.05)',
  },
];

const STORAGE_KEY = 'dashboard-background';
const CUSTOM_IMAGE_KEY = 'dashboard-custom-image';
const DOT_OVERLAY_KEY = 'dashboard-dot-overlay';

// Maximum file size (5MB)
const MAX_FILE_SIZE = 5 * 1024 * 1024;

// File processing timeout (10 seconds)
const PROCESSING_TIMEOUT = 10000;

export default function BackgroundSelector() {
  const [selectedBackground, setSelectedBackground] = useState<string>('default-gradient');
  const [customImage, setCustomImage] = useState<string | null>(null);
  const [showDotOverlay, setShowDotOverlay] = useState<boolean>(true);
  const [isDragging, setIsDragging] = useState<boolean>(false);
  const [isProcessing, setIsProcessing] = useState<boolean>(false);
  const [error, setError] = useState<string | null>(null);
  
  // Refs for cleanup
  const fileReaderRef = useRef<FileReader | null>(null);
  const timeoutRef = useRef<NodeJS.Timeout | null>(null);

  // Load saved settings
  useEffect(() => {
    try {
      const savedBackground = localStorage.getItem(STORAGE_KEY);
      const savedCustomImage = localStorage.getItem(CUSTOM_IMAGE_KEY);
      const savedDotOverlay = localStorage.getItem(DOT_OVERLAY_KEY);

      if (savedBackground) {
        setSelectedBackground(savedBackground);
      }
      if (savedCustomImage) {
        setCustomImage(savedCustomImage);
      }
      if (savedDotOverlay !== null) {
        setShowDotOverlay(savedDotOverlay === 'true');
      }
    } catch (error) {
      console.error('Error loading background settings:', error);
      setError('Failed to load background settings');
    }
  }, []);

  // Cleanup function
  const cleanup = useCallback(() => {
    if (fileReaderRef.current) {
      fileReaderRef.current.abort();
      fileReaderRef.current = null;
    }
    if (timeoutRef.current) {
      clearTimeout(timeoutRef.current);
      timeoutRef.current = null;
    }
    setIsProcessing(false);
    setError(null);
  }, []);

  // Cleanup on unmount
  useEffect(() => {
    return cleanup;
  }, [cleanup]);

  // Safe event dispatch with error handling
  const safeDispatchEvent = useCallback((eventName: string, detail: any) => {
    try {
      // Use requestAnimationFrame to ensure UI updates don't block
      requestAnimationFrame(() => {
        try {
          window.dispatchEvent(new CustomEvent(eventName, { detail }));
        } catch (error) {
          console.error(`Error dispatching ${eventName}:`, error);
          setError(`Failed to update background. Please try again.`);
        }
      });
    } catch (error) {
      console.error(`Error in safeDispatchEvent for ${eventName}:`, error);
      setError('Background update failed. Please try again.');
    }
  }, []);

  // Save settings and trigger update with error handling
  const saveAndApplyBackground = useCallback((backgroundId: string, imageData?: string | null) => {
    try {
      setSelectedBackground(backgroundId);
      localStorage.setItem(STORAGE_KEY, backgroundId);

      if (imageData !== undefined) {
        if (imageData) {
          localStorage.setItem(CUSTOM_IMAGE_KEY, imageData);
        } else {
          localStorage.removeItem(CUSTOM_IMAGE_KEY);
        }
        setCustomImage(imageData);
      }

      // Clear any existing error
      setError(null);

      // Safely dispatch the event
      safeDispatchEvent('dashboard-background-changed', {
        backgroundId,
        customImage: imageData !== undefined ? imageData : customImage
      });

    } catch (error) {
      console.error('Error saving background:', error);
      setError('Failed to save background settings');
    }
  }, [customImage, safeDispatchEvent]);

  // Handle dot overlay toggle
  const handleDotOverlayToggle = useCallback((enabled: boolean) => {
    try {
      setShowDotOverlay(enabled);
      localStorage.setItem(DOT_OVERLAY_KEY, String(enabled));
      safeDispatchEvent('dashboard-dot-overlay-changed', { enabled });
    } catch (error) {
      console.error('Error toggling dot overlay:', error);
      setError('Failed to update dot overlay setting');
    }
  }, [safeDispatchEvent]);

  // Async file processing with timeout and error handling
  const processFileAsync = useCallback(async (file: File): Promise<string> => {
    return new Promise((resolve, reject) => {
      // Validate file type
      if (!file.type.startsWith('image/')) {
        reject(new Error('Please select a valid image file (JPG, PNG, GIF, etc.)'));
        return;
      }

      // Validate file size
      if (file.size > MAX_FILE_SIZE) {
        reject(new Error(`File too large. Please select an image smaller than ${Math.round(MAX_FILE_SIZE / 1024 / 1024)}MB`));
        return;
      }

      // Create FileReader
      const reader = new FileReader();
      fileReaderRef.current = reader;

      // Set up timeout
      timeoutRef.current = setTimeout(() => {
        reader.abort();
        reject(new Error('File processing timed out. Please try a smaller image.'));
      }, PROCESSING_TIMEOUT);

      // Handle successful read
      reader.onload = (e) => {
        try {
          cleanup();
          const result = e.target?.result as string;
          if (result) {
            resolve(result);
          } else {
            reject(new Error('Failed to read image file'));
          }
        } catch (error) {
          cleanup();
          reject(new Error('Error processing image file'));
        }
      };

      // Handle read error
      reader.onerror = () => {
        cleanup();
        reject(new Error('Failed to read image file. Please try again.'));
      };

      // Handle read abort
      reader.onabort = () => {
        cleanup();
        reject(new Error('File processing was cancelled'));
      };

      // Start reading (this is async and won't block the UI)
      try {
        reader.readAsDataURL(file);
      } catch (error) {
        cleanup();
        reject(new Error('Failed to start reading image file'));
      }
    });
  }, [cleanup]);

  // Handle file upload with async processing
  const handleFileUpload = useCallback(async (file: File) => {
    if (isProcessing) {
      return; // Prevent multiple simultaneous uploads
    }

    setIsProcessing(true);
    setError(null);

    try {
      const imageData = await processFileAsync(file);
      saveAndApplyBackground('custom-image', imageData);
    } catch (error) {
      console.error('File upload error:', error);
      setError(error instanceof Error ? error.message : 'Failed to upload image');
    } finally {
      setIsProcessing(false);
    }
  }, [isProcessing, processFileAsync, saveAndApplyBackground]);

  // Handle drag and drop
  const handleDrop = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);

    if (isProcessing) return;

    const files = Array.from(e.dataTransfer.files);
    const imageFile = files.find(file => file.type.startsWith('image/'));
    
    if (imageFile) {
      handleFileUpload(imageFile);
    } else {
      setError('Please drop an image file');
    }
  }, [isProcessing, handleFileUpload]);

  const handleDragOver = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    if (!isProcessing) {
      setIsDragging(true);
    }
  }, [isProcessing]);

  const handleDragLeave = useCallback((e: React.DragEvent) => {
    e.preventDefault();
    setIsDragging(false);
  }, []);

  const handleFileInputChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    const file = e.target.files?.[0];
    if (file) {
      handleFileUpload(file);
    }
    // Clear the input so the same file can be selected again
    e.target.value = '';
  }, [handleFileUpload]);

  const removeCustomImage = useCallback(() => {
    if (isProcessing) return;
    cleanup();
    saveAndApplyBackground('default-gradient', null);
  }, [isProcessing, cleanup, saveAndApplyBackground]);

  const resetToDefault = useCallback(() => {
    if (isProcessing) return;
    cleanup();
    saveAndApplyBackground('default-gradient', null);
    setShowDotOverlay(true);
    localStorage.setItem(DOT_OVERLAY_KEY, 'true');
    safeDispatchEvent('dashboard-dot-overlay-changed', { enabled: true });
  }, [isProcessing, cleanup, saveAndApplyBackground, safeDispatchEvent]);

  return (
    <div className="space-y-6">
      {/* Error Display */}
      {error && (
        <div className="bg-red-50 border border-red-200 rounded-lg p-3">
          <p className="text-sm text-red-800">{error}</p>
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setError(null)}
            className="mt-2 text-red-600 hover:text-red-800"
          >
            Dismiss
          </Button>
        </div>
      )}

      {/* Processing Indicator */}
      {isProcessing && (
        <div className="bg-blue-50 border border-blue-200 rounded-lg p-3 flex items-center gap-2">
          <Loader2 className="w-4 h-4 animate-spin text-blue-600" />
          <p className="text-sm text-blue-800">Processing image...</p>
          <Button
            variant="ghost"
            size="sm"
            onClick={cleanup}
            className="ml-auto text-blue-600 hover:text-blue-800"
          >
            Cancel
          </Button>
        </div>
      )}

      {/* Dot Overlay Toggle */}
      <div className="flex items-center justify-between">
        <div>
          <h3 className="text-sm font-medium text-textStandard">Dot Pattern Overlay</h3>
          <p className="text-xs text-textSubtle mt-1">
            Show subtle dot pattern over the background
          </p>
        </div>
        <Switch
          checked={showDotOverlay}
          onCheckedChange={handleDotOverlayToggle}
          variant="mono"
          disabled={isProcessing}
        />
      </div>

      {/* Preset Backgrounds */}
      <div>
        <h3 className="text-sm font-medium text-textStandard mb-3">Preset Backgrounds</h3>
        <div className="grid grid-cols-2 gap-3">
          {DEFAULT_BACKGROUNDS.map((bg) => (
            <Card
              key={bg.id}
              className={`relative cursor-pointer transition-all duration-200 overflow-hidden ${
                selectedBackground === bg.id
                  ? 'ring-2 ring-blue-500 shadow-lg'
                  : 'hover:shadow-md'
              } ${isProcessing ? 'opacity-50 cursor-not-allowed' : ''}`}
              onClick={() => !isProcessing && saveAndApplyBackground(bg.id)}
            >
              <div className="aspect-video relative">
                <div
                  className="absolute inset-0"
                  style={{
                    background: bg.type === 'gradient' ? (bg.gradient || bg.value) : (bg.color || bg.value),
                    backgroundSize: bg.type === 'gradient' ? '400% 400%' : 'auto',
                  }}
                />
                {/* Dot overlay preview */}
                {showDotOverlay && (
                  <div
                    className="absolute inset-0 opacity-20"
                    style={{
                      backgroundImage: `radial-gradient(circle, rgba(0, 0, 0, 0.4) 1px, transparent 1px)`,
                      backgroundSize: '12px 12px',
                      backgroundPosition: '6px 6px',
                    }}
                  />
                )}
                <div className="absolute inset-0 flex items-center justify-center">
                  <span className="text-xs font-medium text-textStandard bg-background-default/80 px-2 py-1 rounded">
                    {bg.name}
                  </span>
                </div>
              </div>
            </Card>
          ))}
        </div>
      </div>

      {/* Custom Image Section */}
      <div>
        <h3 className="text-sm font-medium text-textStandard mb-3">Custom Image</h3>
        
        {customImage && (
          <div className="mb-4">
            <Card
              className={`relative cursor-pointer transition-all duration-200 overflow-hidden ${
                selectedBackground === 'custom-image'
                  ? 'ring-2 ring-blue-500 shadow-lg'
                  : 'hover:shadow-md'
              } ${isProcessing ? 'opacity-50 cursor-not-allowed' : ''}`}
              onClick={() => !isProcessing && saveAndApplyBackground('custom-image')}
            >
              <div className="aspect-video relative">
                <img
                  src={customImage}
                  alt="Custom background"
                  className="absolute inset-0 w-full h-full object-cover opacity-30"
                />
                {/* Dot overlay preview */}
                {showDotOverlay && (
                  <div
                    className="absolute inset-0 opacity-20"
                    style={{
                      backgroundImage: `radial-gradient(circle, rgba(0, 0, 0, 0.4) 1px, transparent 1px)`,
                      backgroundSize: '12px 12px',
                      backgroundPosition: '6px 6px',
                    }}
                  />
                )}
                <div className="absolute inset-0 flex items-center justify-center">
                  <span className="text-xs font-medium text-textStandard bg-background-default/80 px-2 py-1 rounded">
                    Custom Image
                  </span>
                </div>
              </div>
            </Card>
            
            <div className="flex gap-2 mt-2">
              <Button
                variant="outline"
                size="sm"
                onClick={removeCustomImage}
                disabled={isProcessing}
                className="flex items-center gap-1"
              >
                <X className="w-3 h-3" />
                Remove
              </Button>
            </div>
          </div>
        )}

        {/* Upload Area */}
        <div
          className={`border-2 border-dashed rounded-lg p-6 text-center transition-colors ${
            isDragging
              ? 'border-blue-400 bg-blue-50'
              : 'border-gray-300 hover:border-gray-400'
          } ${isProcessing ? 'opacity-50 cursor-not-allowed' : 'cursor-pointer'}`}
          onDrop={handleDrop}
          onDragOver={handleDragOver}
          onDragLeave={handleDragLeave}
          onClick={() => !isProcessing && document.getElementById('background-file-input')?.click()}
        >
          <input
            id="background-file-input"
            type="file"
            accept="image/*"
            onChange={handleFileInputChange}
            className="hidden"
            disabled={isProcessing}
          />
          
          {isProcessing ? (
            <div className="flex flex-col items-center gap-2">
              <Loader2 className="w-8 h-8 animate-spin text-blue-500" />
              <p className="text-sm text-textSubtle">Processing image...</p>
            </div>
          ) : (
            <div className="flex flex-col items-center gap-2">
              <Upload className="w-8 h-8 text-textSubtle" />
              <p className="text-sm text-textStandard">
                {isDragging ? 'Drop your image here' : 'Click to upload or drag and drop'}
              </p>
              <p className="text-xs text-textSubtle">
                Supports JPG, PNG, GIF up to {Math.round(MAX_FILE_SIZE / 1024 / 1024)}MB
              </p>
            </div>
          )}
        </div>
      </div>

      {/* Reset Button */}
      <div className="pt-4 border-t">
        <Button
          variant="outline"
          onClick={resetToDefault}
          disabled={isProcessing}
          className="flex items-center gap-2"
        >
          <RotateCcw className="w-4 h-4" />
          Reset to Default
        </Button>
      </div>
    </div>
  );
}
