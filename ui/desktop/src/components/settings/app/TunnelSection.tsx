import { useState, useEffect } from 'react';
import { Card, CardContent, CardDescription, CardHeader, CardTitle } from '../../ui/card';
import { Button } from '../../ui/button';
import { QrCode, Globe, Copy, Check, Loader2, X } from 'lucide-react';
import { Dialog, DialogContent, DialogHeader, DialogTitle } from '../../ui/dialog';

interface TunnelConnectionInfo {
  url: string;
  secret: string;
  appUrl: string;
  qrCodeDataUrl: string;
}

interface TunnelStatus {
  isRunning: boolean;
  config: {
    port: number;
    secret: string;
    url?: string;
    qrCodePath?: string;
    qrCodeDataUrl?: string;
  } | null;
}

export default function TunnelSection() {
  const [tunnelStatus, setTunnelStatus] = useState<TunnelStatus>({
    isRunning: false,
    config: null,
  });
  const [isStarting, setIsStarting] = useState(false);
  const [isStopping, setIsStopping] = useState(false);
  const [tunnelInfo, setTunnelInfo] = useState<TunnelConnectionInfo | null>(null);
  const [showQRModal, setShowQRModal] = useState(false);
  const [copiedField, setCopiedField] = useState<string | null>(null);
  const [error, setError] = useState<string | null>(null);

  useEffect(() => {
    checkTunnelStatus();
  }, []);

  const checkTunnelStatus = async () => {
    try {
      const status = await window.electron.tunnelStatus();
      setTunnelStatus(status);
      if (status.isRunning && status.config) {
        setTunnelInfo({
          url: status.config.url || '',
          secret: status.config.secret,
          appUrl: '', // We'll reconstruct this if needed
          qrCodeDataUrl: status.config.qrCodeDataUrl || '',
        });
      }
    } catch (error) {
      console.error('Failed to check tunnel status:', error);
    }
  };

  const startTunnel = async () => {
    setIsStarting(true);
    setError(null);
    try {
      const info = await window.electron.tunnelStart();
      setTunnelInfo(info);
      setTunnelStatus({
        isRunning: true,
        config: {
          port: 0, // Port is now managed internally
          secret: info.secret,
          url: info.url,
          qrCodeDataUrl: info.qrCodeDataUrl,
        },
      });
      setShowQRModal(true);
    } catch (error) {
      console.error('Failed to start tunnel:', error);
      setError(error instanceof Error ? error.message : 'Failed to start tunnel');
    } finally {
      setIsStarting(false);
    }
  };

  const stopTunnel = async () => {
    setIsStopping(true);
    setError(null);
    try {
      await window.electron.tunnelStop();
      setTunnelInfo(null);
      setTunnelStatus({ isRunning: false, config: null });
    } catch (error) {
      console.error('Failed to stop tunnel:', error);
      setError(error instanceof Error ? error.message : 'Failed to stop tunnel');
    } finally {
      setIsStopping(false);
    }
  };

  const copyToClipboard = async (text: string, field: string) => {
    try {
      await navigator.clipboard.writeText(text);
      setCopiedField(field);
      setTimeout(() => setCopiedField(null), 2000);
    } catch (error) {
      console.error('Failed to copy to clipboard:', error);
    }
  };

  return (
    <>
      <Card className="rounded-lg">
        <CardHeader className="pb-0">
          <CardTitle>Mobile Tunnel</CardTitle>
          <CardDescription>
            Create a secure tunnel to access Goose from your iOS device
          </CardDescription>
        </CardHeader>
        <CardContent className="pt-4 space-y-4 px-4">
          {error && (
            <div className="bg-destructive/10 text-destructive text-sm p-3 rounded-md flex items-start gap-2">
              <X className="h-4 w-4 mt-0.5" />
              <span>{error}</span>
            </div>
          )}

          <div className="space-y-4">
            {!tunnelStatus.isRunning ? (
              <div className="space-y-4">
                <p className="text-xs text-text-muted">
                  Start a tunnel to expose your local Goose instance to the internet securely. You
                  can then scan a QR code with your iOS device to connect.
                </p>
                <Button onClick={startTunnel} disabled={isStarting} className="w-full sm:w-auto">
                  {isStarting ? (
                    <>
                      <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                      Starting Tunnel...
                    </>
                  ) : (
                    <>
                      <Globe className="mr-2 h-4 w-4" />
                      Start Tunnel
                    </>
                  )}
                </Button>
              </div>
            ) : (
              <div className="space-y-4">
                <div className="bg-success/10 text-success text-sm p-3 rounded-md">
                  Tunnel is running
                </div>

                {tunnelInfo && (
                  <div className="space-y-3">
                    <div className="space-y-2">
                      <div className="flex items-center justify-between">
                        <span className="text-xs text-text-muted">Connection URL:</span>
                        <div className="flex items-center gap-2">
                          <code className="text-xs bg-background-secondary px-2 py-1 rounded">
                            {tunnelInfo.url}
                          </code>
                          <Button
                            size="sm"
                            variant="ghost"
                            onClick={() => copyToClipboard(tunnelInfo.url, 'url')}
                          >
                            {copiedField === 'url' ? (
                              <Check className="h-3 w-3" />
                            ) : (
                              <Copy className="h-3 w-3" />
                            )}
                          </Button>
                        </div>
                      </div>

                      <div className="flex items-center justify-between">
                        <span className="text-xs text-text-muted">Secret:</span>
                        <div className="flex items-center gap-2">
                          <code className="text-xs bg-background-secondary px-2 py-1 rounded font-mono">
                            {tunnelInfo.secret.substring(0, 8)}...
                          </code>
                          <Button
                            size="sm"
                            variant="ghost"
                            onClick={() => copyToClipboard(tunnelInfo.secret, 'secret')}
                          >
                            {copiedField === 'secret' ? (
                              <Check className="h-3 w-3" />
                            ) : (
                              <Copy className="h-3 w-3" />
                            )}
                          </Button>
                        </div>
                      </div>
                    </div>

                    <div className="flex gap-2">
                      <Button
                        onClick={() => setShowQRModal(true)}
                        variant="secondary"
                        className="flex-1"
                      >
                        <QrCode className="mr-2 h-4 w-4" />
                        Show QR Code
                      </Button>

                      <Button
                        onClick={stopTunnel}
                        disabled={isStopping}
                        variant="destructive"
                        className="flex-1"
                      >
                        {isStopping ? (
                          <>
                            <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                            Stopping...
                          </>
                        ) : (
                          'Stop Tunnel'
                        )}
                      </Button>
                    </div>
                  </div>
                )}
              </div>
            )}
          </div>
        </CardContent>
      </Card>

      {/* QR Code Modal */}
      <Dialog open={showQRModal} onOpenChange={setShowQRModal}>
        <DialogContent className="max-w-md">
          <DialogHeader>
            <DialogTitle>Scan with iOS App</DialogTitle>
          </DialogHeader>
          <div className="flex flex-col items-center space-y-4 p-4">
            {tunnelInfo?.qrCodeDataUrl ? (
              <>
                <img
                  src={tunnelInfo.qrCodeDataUrl}
                  alt="QR Code for mobile connection"
                  className="w-64 h-64"
                />
                <p className="text-sm text-center text-text-muted">
                  Scan this QR code with your iOS device camera to connect to Goose
                </p>
                <div className="w-full space-y-2 text-xs">
                  <div className="flex justify-between">
                    <span className="text-text-muted">Tunnel URL:</span>
                    <code className="bg-background-secondary px-2 py-1 rounded">
                      {tunnelInfo.url}
                    </code>
                  </div>
                  <div className="flex justify-between">
                    <span className="text-text-muted">Secret:</span>
                    <code className="bg-background-secondary px-2 py-1 rounded font-mono">
                      {tunnelInfo.secret.substring(0, 12)}...
                    </code>
                  </div>
                </div>
              </>
            ) : (
              <div className="flex items-center justify-center h-64">
                <Loader2 className="h-8 w-8 animate-spin" />
              </div>
            )}
          </div>
        </DialogContent>
      </Dialog>
    </>
  );
}
