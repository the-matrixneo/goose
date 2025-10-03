import { CloudflareTunnel } from './cloudflare-tunnel';
import * as QRCode from 'qrcode';
import log from './logger';

export interface TunnelConfig {
  port: number;
  secret: string;
  url?: string;
  qrCodeDataUrl?: string;
}

export interface TunnelConnectionInfo {
  url: string;
  secret: string;
  appUrl: string;
  qrCodeDataUrl: string;
}

export class TunnelManager {
  private cloudflaredTunnel: CloudflareTunnel | null = null;
  private config: TunnelConfig | null = null;
  private isStarting = false;

  constructor() {}

  /**
   * Start a Cloudflare tunnel for the given port with a secret
   */
  async start(port: number, secret: string): Promise<TunnelConnectionInfo> {
    if (this.cloudflaredTunnel && this.cloudflaredTunnel.isRunning()) {
      throw new Error('Tunnel is already running');
    }

    if (this.isStarting) {
      throw new Error('Tunnel is already starting');
    }

    this.isStarting = true;

    try {
      this.config = {
        port,
        secret,
      };

      // Start Cloudflare tunnel
      log.info(`Starting Cloudflare tunnel on port ${port}`);

      if (!this.cloudflaredTunnel) {
        this.cloudflaredTunnel = new CloudflareTunnel();
      }

      const url = await this.cloudflaredTunnel.start(port);

      if (!url) {
        throw new Error('Failed to get Cloudflare tunnel URL');
      }

      log.info(`Cloudflare tunnel established: ${url}`);

      // Format connection URL (remove https:// and add :443)
      const connectUrl = url.replace('https://', '') + ':443';

      // Create connection config for QR code
      const configData = {
        url: connectUrl,
        secret,
      };

      // Create app URL for deep linking
      const configJson = JSON.stringify(configData);
      const urlEncodedConfig = encodeURIComponent(configJson);
      const appUrl = `goosechat://configure?data=${urlEncodedConfig}`;

      // Generate QR code
      const qrCodeDataUrl = await QRCode.toDataURL(appUrl, {
        errorCorrectionLevel: 'M',
        type: 'image/png',
        width: 400,
        margin: 2,
      });

      this.config.url = connectUrl;
      this.config.qrCodeDataUrl = qrCodeDataUrl;

      const result: TunnelConnectionInfo = {
        url: connectUrl,
        secret,
        appUrl,
        qrCodeDataUrl,
      };

      log.info('Tunnel started successfully:', { url: connectUrl });

      return result;
    } catch (error) {
      log.error('Failed to start Cloudflare tunnel:', error);
      this.cloudflaredTunnel = null;
      throw error;
    } finally {
      this.isStarting = false;
    }
  }

  /**
   * Stop the tunnel
   */
  async stop(): Promise<void> {
    if (this.cloudflaredTunnel) {
      log.info('Stopping Cloudflare tunnel');
      try {
        this.cloudflaredTunnel.stop();
      } catch (error) {
        log.error('Error stopping Cloudflare tunnel:', error);
      }
      this.cloudflaredTunnel = null;
    }

    this.config = null;
  }

  /**
   * Get current tunnel status
   */
  isRunning(): boolean {
    return this.cloudflaredTunnel !== null && this.cloudflaredTunnel.isRunning();
  }

  /**
   * Get current tunnel configuration
   */
  getConfig(): TunnelConfig | null {
    return this.config;
  }
}

// Singleton instance
let tunnelManager: TunnelManager | null = null;

/**
 * Get or create the tunnel manager instance
 */
export function getTunnelManager(): TunnelManager {
  if (!tunnelManager) {
    tunnelManager = new TunnelManager();
  }
  return tunnelManager;
}

/**
 * Start a tunnel for the given port
 */
export async function startTunnel(port: number, secret: string): Promise<TunnelConnectionInfo> {
  const manager = getTunnelManager();
  return manager.start(port, secret);
}

/**
 * Stop the current tunnel
 */
export async function stopTunnel(): Promise<void> {
  const manager = getTunnelManager();
  await manager.stop();
}

/**
 * Check if tunnel is running
 */
export function isTunnelRunning(): boolean {
  const manager = getTunnelManager();
  return manager.isRunning();
}

/**
 * Get current tunnel configuration
 */
export function getTunnelConfig(): TunnelConfig | null {
  const manager = getTunnelManager();
  return manager.getConfig();
}

/**
 * Cleanup tunnel resources
 */
export async function cleanupTunnel(): Promise<void> {
  const manager = getTunnelManager();
  await manager.stop();
}
