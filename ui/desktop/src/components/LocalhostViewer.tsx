import { useRef, useState, useEffect } from 'react';
import { RefreshCw, ExternalLink, ChevronLeft, ChevronRight } from 'lucide-react';
import { Button } from './ui/button';
import { Tooltip, TooltipTrigger, TooltipContent } from './ui/Tooltip';

interface LocalhostViewerProps {
  initialUrl?: string;
  onUrlChange?: (url: string) => void;
}

function isValidLocalhostUrl(url: string): boolean {
  try {
    const parsedUrl = new URL(url);
    return (
      parsedUrl.protocol === 'http:' &&
      (parsedUrl.hostname === 'localhost' || parsedUrl.hostname === '127.0.0.1')
    );
  } catch {
    return false;
  }
}

function formatUrl(input: string): string {
  // If it's just a port number, prepend localhost
  if (/^\d+$/.test(input.trim())) {
    return `http://localhost:${input.trim()}`;
  }

  // If it starts with localhost: or 127.0.0.1: without protocol
  if (/^(localhost|127\.0\.0\.1):\d+/.test(input.trim())) {
    return `http://${input.trim()}`;
  }

  return input.trim();
}

export function LocalhostViewer({
  initialUrl = 'http://localhost:3000',
  onUrlChange,
}: LocalhostViewerProps) {
  // Initialize from localStorage or use initialUrl
  const [url, setUrl] = useState(() => {
    if (typeof window !== 'undefined') {
      return localStorage.getItem('goose-sidecar-url') || initialUrl;
    }
    return initialUrl;
  });
  
  const [inputUrl, setInputUrl] = useState(() => {
    if (typeof window !== 'undefined') {
      return localStorage.getItem('goose-sidecar-url') || initialUrl;
    }
    return initialUrl;
  });
  
  const [isLoading, setIsLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);
  const [canGoBack, setCanGoBack] = useState(false);
  const [canGoForward, setCanGoForward] = useState(false);
  const [hasLoadedOnce, setHasLoadedOnce] = useState(false);
  const [retryCount, setRetryCount] = useState(0);
  const iframeRef = useRef<HTMLIFrameElement>(null);
  const retryTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    if (onUrlChange) {
      onUrlChange(url);
    }
  }, [url, onUrlChange]);

  // Cleanup timeout on unmount
  useEffect(() => {
    return () => {
      if (retryTimeoutRef.current) {
        clearTimeout(retryTimeoutRef.current);
      }
    };
  }, []);

  const handleUrlSubmit = (newUrl: string) => {
    const formattedUrl = formatUrl(newUrl);

    if (!isValidLocalhostUrl(formattedUrl)) {
      setError('Only localhost URLs are allowed (e.g., http://localhost:3000)');
      return;
    }

    setError(null);
    setUrl(formattedUrl);
    setInputUrl(formattedUrl);
    setIsLoading(true);
    setRetryCount(0); // Reset retry count for new URL
    
    // Save to localStorage
    if (typeof window !== 'undefined') {
      localStorage.setItem('goose-sidecar-url', formattedUrl);
    }
  };

  const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
    if (e.key === 'Enter') {
      handleUrlSubmit(inputUrl);
    }
  };

  const handleRefresh = () => {
    if (iframeRef.current) {
      setIsLoading(true);
      setError(null);
      iframeRef.current.src = iframeRef.current.src;
    }
  };

  const handleOpenInBrowser = () => {
    window.open(url, '_blank');
  };

  const handleGoBack = () => {
    if (iframeRef.current?.contentWindow) {
      try {
        iframeRef.current.contentWindow.history.back();
      } catch (e) {
        console.warn('Cannot access iframe history:', e);
      }
    }
  };

  const handleGoForward = () => {
    if (iframeRef.current?.contentWindow) {
      try {
        iframeRef.current.contentWindow.history.forward();
      } catch (e) {
        console.warn('Cannot access iframe history:', e);
      }
    }
  };

  const handleIframeLoad = () => {
    setIsLoading(false);
    setError(null);
    setHasLoadedOnce(true);
    setRetryCount(0); // Reset retry count on successful load

    // Clear any pending retry timeout
    if (retryTimeoutRef.current) {
      clearTimeout(retryTimeoutRef.current);
      retryTimeoutRef.current = null;
    }

    // Try to update navigation state (may not work due to CORS)
    try {
      if (iframeRef.current?.contentWindow) {
        setCanGoBack(iframeRef.current.contentWindow.history.length > 1);
        setCanGoForward(false); // Reset forward state
      }
    } catch (e) {
      // Ignore CORS errors when trying to access iframe history
    }
  };

  const handleIframeError = () => {
    setIsLoading(false);
    
    // If this is the first load attempt and we haven't loaded successfully before
    // and we haven't exceeded our retry limit, automatically retry
    const maxRetries = 3;
    const retryDelay = 2000; // 2 seconds
    
    if (!hasLoadedOnce && retryCount < maxRetries) {
      console.log(`LocalhostViewer: Auto-retrying connection (attempt ${retryCount + 1}/${maxRetries})`);
      
      setRetryCount(prev => prev + 1);
      setError(`Connecting to ${url}... (attempt ${retryCount + 1}/${maxRetries})`);
      
      // Set a timeout to retry
      retryTimeoutRef.current = setTimeout(() => {
        if (iframeRef.current) {
          setIsLoading(true);
          // Force reload by updating the src
          iframeRef.current.src = `${url}?retry=${Date.now()}`;
        }
      }, retryDelay);
    } else {
      // Show final error message
      const errorMessage = retryCount >= maxRetries 
        ? `Failed to connect to ${url} after ${maxRetries} attempts. Make sure the server is running.`
        : `Failed to load ${url}. Make sure the server is running.`;
      
      setError(errorMessage);
    }
  };

  return (
    <div className="h-full flex flex-col bg-background-default">
      {/* URL Bar and Controls */}
      <div className="flex items-center gap-2 p-3 border-b border-borderSubtle bg-background-muted">
        {/* Navigation buttons */}
        <div className="flex items-center gap-1">
          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant="ghost"
                size="sm"
                onClick={handleGoBack}
                disabled={!canGoBack}
                className="p-1 h-8 w-8"
              >
                <ChevronLeft size={14} />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Go Back</TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger asChild>
              <Button
                variant="ghost"
                size="sm"
                onClick={handleGoForward}
                disabled={!canGoForward}
                className="p-1 h-8 w-8"
              >
                <ChevronRight size={14} />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Go Forward</TooltipContent>
          </Tooltip>

          <Tooltip>
            <TooltipTrigger asChild>
              <Button variant="ghost" size="sm" onClick={handleRefresh} className="p-1 h-8 w-8">
                <RefreshCw size={14} className={isLoading ? 'animate-spin' : ''} />
              </Button>
            </TooltipTrigger>
            <TooltipContent>Refresh</TooltipContent>
          </Tooltip>
        </div>

        {/* URL Input */}
        <div className="flex-1 flex items-center">
          <input
            type="text"
            value={inputUrl}
            onChange={(e) => setInputUrl(e.target.value)}
            onKeyPress={handleKeyPress}
            placeholder="http://localhost:3000"
            className="flex-1 px-3 py-1 text-sm border border-borderSubtle rounded-md bg-background-default text-textStandard focus:outline-none focus:ring-2 focus:ring-borderProminent focus:border-transparent"
          />
          <Button
            variant="ghost"
            size="sm"
            onClick={() => handleUrlSubmit(inputUrl)}
            className="ml-2 px-3 py-1 text-xs"
          >
            Go
          </Button>
        </div>

        {/* External link button */}
        <Tooltip>
          <TooltipTrigger asChild>
            <Button variant="ghost" size="sm" onClick={handleOpenInBrowser} className="p-1 h-8 w-8">
              <ExternalLink size={14} />
            </Button>
          </TooltipTrigger>
          <TooltipContent>Open in Browser</TooltipContent>
        </Tooltip>
      </div>

      {/* Error Display */}
      {error && (
        <div className={`p-3 border-b ${
          error.includes('attempt') || error.includes('Connecting')
            ? 'bg-yellow-50 dark:bg-yellow-900/20 border-yellow-200 dark:border-yellow-800'
            : 'bg-red-50 dark:bg-red-900/20 border-red-200 dark:border-red-800'
        }`}>
          <p className={`text-sm ${
            error.includes('attempt') || error.includes('Connecting')
              ? 'text-yellow-800 dark:text-yellow-200'
              : 'text-red-800 dark:text-red-200'
          }`}>
            {error}
            {error.includes('attempt') && (
              <span className="ml-2">
                <RefreshCw className="inline h-3 w-3 animate-spin" />
              </span>
            )}
          </p>
        </div>
      )}

      {/* Iframe Content */}
      <div className="flex-1 relative overflow-hidden">
        <iframe
          ref={iframeRef}
          src={url}
          className="w-full h-full border-0"
          title="Localhost Viewer"
          onLoad={handleIframeLoad}
          onError={handleIframeError}
          sandbox="allow-same-origin allow-scripts allow-forms allow-popups allow-popups-to-escape-sandbox allow-presentation allow-top-navigation-by-user-activation"
        />

        {/* Loading overlay */}
        {isLoading && (
          <div className="absolute inset-0 bg-background-default/80 flex items-center justify-center">
            <div className="text-center">
              <RefreshCw className="h-6 w-6 animate-spin mx-auto mb-2 text-primary" />
              <p className="text-textSubtle text-sm">
                Loading {url}...
                {retryCount > 0 && (
                  <span className="block text-xs mt-1">
                    Attempt {retryCount + 1}/3
                  </span>
                )}
              </p>
            </div>
          </div>
        )}
      </div>
    </div>
  );
}

export default LocalhostViewer;
