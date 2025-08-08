import { useRef, useState, useEffect } from 'react';
import { RefreshCw, ExternalLink, ChevronLeft, ChevronRight, AlertCircle, CheckCircle } from 'lucide-react';
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
  const [connectionStatus, setConnectionStatus] = useState<'idle' | 'connecting' | 'retrying' | 'connected' | 'failed'>('idle');
  const [canGoBack, setCanGoBack] = useState(false);
  const [canGoForward, setCanGoForward] = useState(false);
  const [hasLoadedOnce, setHasLoadedOnce] = useState(false);
  const [retryCount, setRetryCount] = useState(0);
  const iframeRef = useRef<HTMLIFrameElement>(null);
  const retryTimeoutRef = useRef<NodeJS.Timeout | null>(null);
  const contentCheckTimeoutRef = useRef<NodeJS.Timeout | null>(null);

  useEffect(() => {
    if (onUrlChange) {
      onUrlChange(url);
    }
  }, [url, onUrlChange]);

  // Cleanup timeouts on unmount
  useEffect(() => {
    return () => {
      if (retryTimeoutRef.current) {
        clearTimeout(retryTimeoutRef.current);
      }
      if (contentCheckTimeoutRef.current) {
        clearTimeout(contentCheckTimeoutRef.current);
      }
    };
  }, []);

  const handleUrlSubmit = (newUrl: string) => {
    const formattedUrl = formatUrl(newUrl);

    if (!isValidLocalhostUrl(formattedUrl)) {
      setError('Only localhost URLs are allowed (e.g., http://localhost:3000)');
      setConnectionStatus('failed');
      return;
    }

    setError(null);
    setUrl(formattedUrl);
    setInputUrl(formattedUrl);
    setIsLoading(true);
    setConnectionStatus('connecting');
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
      setConnectionStatus('connecting');
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

  const checkIframeContent = () => {
    try {
      const iframe = iframeRef.current;
      if (!iframe || !iframe.contentDocument) {
        return false;
      }

      const doc = iframe.contentDocument;
      const title = doc.title;
      const body = doc.body;
      
      // Check for common error page indicators
      const isErrorPage = (
        title.toLowerCase().includes('error') ||
        title.toLowerCase().includes('not found') ||
        title.toLowerCase().includes('404') ||
        title.toLowerCase().includes('500') ||
        title.toLowerCase().includes('connection') ||
        title.toLowerCase().includes('refused') ||
        title.toLowerCase().includes('unavailable')
      );

      // Check if body is essentially empty or contains error messages
      const bodyText = body?.textContent?.trim().toLowerCase() || '';
      const hasErrorContent = (
        bodyText.includes('cannot get') ||
        bodyText.includes('not found') ||
        bodyText.includes('error') ||
        bodyText.includes('connection refused') ||
        bodyText.includes('econnrefused') ||
        bodyText.includes('site can\'t be reached') ||
        bodyText.includes('server is not responding')
      );

      // Check if the page is essentially empty (very little content)
      const isEmpty = bodyText.length < 50 && !body?.querySelector('script, style, img, video, canvas');

      return !isErrorPage && !hasErrorContent && !isEmpty;
    } catch (e) {
      // If we can't access the iframe content (CORS), assume it loaded successfully
      // since the onLoad event fired
      return true;
    }
  };

  const handleIframeLoad = () => {
    setIsLoading(false);

    // Clear any pending retry timeout
    if (retryTimeoutRef.current) {
      clearTimeout(retryTimeoutRef.current);
      retryTimeoutRef.current = null;
    }

    // Give the iframe a moment to fully load its content before checking
    contentCheckTimeoutRef.current = setTimeout(() => {
      const hasValidContent = checkIframeContent();
      
      if (hasValidContent) {
        setError(null);
        setConnectionStatus('connected');
        setHasLoadedOnce(true);
        setRetryCount(0); // Reset retry count on successful load
      } else {
        // Treat empty/error content as a connection failure
        console.log('LocalhostViewer: Detected empty or error content, treating as connection failure');
        handleIframeError();
        return;
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
    }, 500); // Wait 500ms for content to fully load
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
      setConnectionStatus('retrying');
      setError(`Retrying... (${retryCount + 1}/${maxRetries})`);
      
      // Set a timeout to retry
      retryTimeoutRef.current = setTimeout(() => {
        if (iframeRef.current) {
          setIsLoading(true);
          setConnectionStatus('connecting');
          // Force reload by updating the src
          iframeRef.current.src = `${url}?retry=${Date.now()}`;
        }
      }, retryDelay);
    } else {
      // Show final error message
      setConnectionStatus('failed');
      const errorMessage = retryCount >= maxRetries 
        ? `No server running on port`
        : `Connection failed`;
      
      setError(errorMessage);
    }
  };

  // Helper function to render connection status indicator
  const renderConnectionStatus = () => {
    switch (connectionStatus) {
      case 'connecting':
        return (
          <div className="flex items-center text-blue-600 text-xs">
            <RefreshCw className="h-3 w-3 animate-spin mr-1" />
            <span>Connecting...</span>
          </div>
        );
      case 'retrying':
        return (
          <div className="flex items-center text-yellow-600 text-xs">
            <RefreshCw className="h-3 w-3 animate-spin mr-1" />
            <span>{error}</span>
          </div>
        );
      case 'connected':
        return (
          <div className="flex items-center text-green-600 text-xs">
            <CheckCircle className="h-3 w-3 mr-1" />
            <span>Connected</span>
          </div>
        );
      case 'failed':
        return (
          <div className="flex items-center text-red-600 text-xs">
            <AlertCircle className="h-3 w-3 mr-1" />
            <span>{error}</span>
          </div>
        );
      default:
        return null;
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

        {/* URL Input with Status */}
        <div className="flex-1 flex items-center gap-2">
          <div className="flex-1 flex flex-col">
            <div className="flex items-center">
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
            {/* Connection Status - appears below the input */}
            <div className="mt-1 min-h-[16px]">
              {renderConnectionStatus()}
            </div>
          </div>
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
