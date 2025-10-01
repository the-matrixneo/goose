import { CloudflareTunnel } from './cloudflare-tunnel';
import * as QRCode from 'qrcode';
import * as crypto from 'node:crypto';
import { Buffer } from 'node:buffer';
import path from 'node:path';
import fs from 'node:fs/promises';
import os from 'node:os';
import https from 'node:https';
import log from './logger';

export interface TunnelConfig {
  port: number;
  secret: string;
  url?: string;
  qrCodePath?: string;
  qrCodeDataUrl?: string;
}

export interface TunnelConnectionInfo {
  url: string;
  secret: string;
  appUrl: string;
  qrCodeDataUrl: string;
  qrCodePath: string;
  ntfyUrl: string; // The ntfy.sh URL used in the QR code
}

const HEALTH_CHECK_INTERVAL_MS = 10 * 60 * 1000; // 10 minutes
const FAILURE_THRESHOLD = 3; // Restart after 3 consecutive failures

// ============================================================================
// keep alive interval for tunnel - say an hour or so
// ============================================================================
const PROACTIVE_RESTART_INTERVAL_MS = 60 * 60 * 1000;

// ============================================================================
// TODO: TESTING TOGGLE - Set to false to disable ntfy.sh and use direct URLs
// When false, QR code will contain the tunnel URL directly (old behavior)
// When true, QR code contains ntfy.sh URL which resolves to tunnel URL
// ============================================================================
const USE_NTFY_SH = true;

const MACHINE_ID_FILE = 'machine-id.txt';

export class TunnelManager {
  private cloudflaredTunnel: CloudflareTunnel | null = null;
  private config: TunnelConfig | null = null;
  private isStarting = false;
  private tunnelUrl: string | null = null;
  private healthCheckInterval: ReturnType<typeof setInterval> | null = null;
  private proactiveRestartTimeout: ReturnType<typeof setTimeout> | null = null;
  private consecutiveFailures = 0;
  private machineId: string | null = null;
  private port: number = 0;

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
      // Store port and secret for potential restarts
      this.port = port;
      // Use the provided secret (from goosed)
      const tunnelSecret = secret;

      this.config = {
        port,
        secret: tunnelSecret,
      };

      // Start Cloudflare tunnel
      log.info(`Starting Cloudflare tunnel on port ${port}`);
      this.tunnelUrl = await this.startCloudflareTunnel(port);

      if (!this.tunnelUrl) {
        throw new Error('Failed to get tunnel URL');
      }

      // Format connection URL (remove https:// and add :443)
      const connectUrl = this.tunnelUrl.replace('https://', '') + ':443';

      // Get machine ID for ntfy.sh topic
      const machineId = await this.getMachineId();
      const ntfyTopic = `goose-tunnel-${machineId}`;
      const ntfyUrl = `https://ntfy.sh/${ntfyTopic}`;

      // Create connection config for QR code
      // If USE_NTFY_SH is true, use ntfy.sh URL; otherwise use direct tunnel URL
      const configData = {
        url: USE_NTFY_SH ? ntfyUrl : connectUrl,
        secret: tunnelSecret,
      };

      // Create app URL for deep linking
      const configJson = JSON.stringify(configData);
      const urlEncodedConfig = encodeURIComponent(configJson);
      const appUrl = `goosechat://configure?data=${urlEncodedConfig}`;

      // Generate QR code (only for GUI display, no need to save to disk)
      const qrCodeDataUrl = await this.generateQRCode(appUrl);

      this.config.url = connectUrl;
      this.config.qrCodePath = ''; // Not saving to disk anymore
      this.config.qrCodeDataUrl = qrCodeDataUrl;

      const result: TunnelConnectionInfo = {
        url: connectUrl,
        secret: tunnelSecret,
        appUrl,
        qrCodeDataUrl,
        qrCodePath: '', // QR code only shown in GUI, not saved to disk
        ntfyUrl: USE_NTFY_SH ? ntfyUrl : connectUrl, // Show the URL that's actually in the QR code
      };

      log.info('Tunnel started successfully:', {
        url: connectUrl,
        ntfyUrl,
        useNtfySh: USE_NTFY_SH,
      });

      // Send notification to ntfy.sh only if using ntfy.sh
      if (USE_NTFY_SH) {
        await this.notifyNewUrl(this.tunnelUrl);
      }

      // Start health monitoring
      this.startHealthMonitoring();

      // Schedule proactive restart
      this.scheduleProactiveRestart();

      return result;
    } finally {
      this.isStarting = false;
    }
  }

  /**
   * Start Cloudflare tunnel and return the URL
   */
  private async startCloudflareTunnel(port: number): Promise<string> {
    try {
      // Create or get the Cloudflare tunnel instance
      if (!this.cloudflaredTunnel) {
        this.cloudflaredTunnel = new CloudflareTunnel();
      }

      // Start the tunnel
      const url = await this.cloudflaredTunnel.start(port);

      if (!url) {
        throw new Error('Failed to get Cloudflare tunnel URL');
      }

      this.tunnelUrl = url;
      log.info(`Cloudflare tunnel established: ${url}`);

      return url;
    } catch (error) {
      log.error('Failed to start Cloudflare tunnel:', error);
      this.cloudflaredTunnel = null;
      throw new Error(`Failed to start Cloudflare tunnel: ${error}`);
    }
  }

  /**
   * Generate QR code as data URL
   */
  private async generateQRCode(data: string): Promise<string> {
    try {
      const qrCodeDataUrl = await QRCode.toDataURL(data, {
        errorCorrectionLevel: 'M',
        type: 'image/png',
        width: 400,
        margin: 2,
      });
      return qrCodeDataUrl;
    } catch (error) {
      log.error('Failed to generate QR code:', error);
      throw new Error('Failed to generate QR code');
    }
  }
  /**
   * Get or generate a stable machine ID
   */
  private async getMachineId(): Promise<string> {
    if (this.machineId) {
      return this.machineId;
    }

    try {
      // Get the app data directory
      const appDataPath =
        process.env.APPDATA ||
        (process.platform == 'darwin'
          ? path.join(os.homedir(), 'Library', 'Application Support')
          : path.join(os.homedir(), '.local', 'share'));

      const gooseDataPath = path.join(appDataPath, 'Goose');
      await fs.mkdir(gooseDataPath, { recursive: true });

      const machineIdPath = path.join(gooseDataPath, MACHINE_ID_FILE);

      // Try to read existing machine ID
      try {
        const existingId = await fs.readFile(machineIdPath, 'utf-8');
        this.machineId = existingId.trim();
        log.info('Loaded existing machine ID');
        return this.machineId;
      } catch {
        // File doesn't exist, generate new ID
        log.info('Generating new machine ID');
      }

      // Generate a new machine ID based on hostname and random data
      const hostname = os.hostname();
      const randomData = crypto.randomBytes(16).toString('hex');
      const combined = `${hostname}-${randomData}`;

      // Hash it to create a stable, short ID
      const hash = crypto.createHash('sha256').update(combined).digest('hex');
      this.machineId = hash.substring(0, 16);

      // Save it for future use
      await fs.writeFile(machineIdPath, this.machineId, 'utf-8');
      log.info('Generated and saved new machine ID');

      return this.machineId;
    } catch (error) {
      log.error('Failed to get/generate machine ID:', error);
      // Fallback to a session-only ID
      const fallback = crypto.randomBytes(8).toString('hex');
      this.machineId = fallback;
      return this.machineId;
    }
  }

  /**
   * Send notification to ntfy.sh with the tunnel URL
   */
  private async notifyNewUrl(url: string): Promise<void> {
    try {
      const machineId = await this.getMachineId();
      const topic = `goose-tunnel-${machineId}`;
      const ntfyUrl = `https://ntfy.sh/${topic}`;

      log.info(`Sending notification to ntfy.sh topic: ${topic}`);

      return new Promise((resolve) => {
        const postData = url;

        const req = https.request(
          ntfyUrl,
          {
            method: 'POST',
            headers: {
              'Content-Type': 'text/plain',
              'Content-Length': Buffer.byteLength(postData),
            },
          },
          (res) => {
            if (res.statusCode && res.statusCode >= 200 && res.statusCode < 300) {
              log.info('Successfully sent notification to ntfy.sh');
              resolve();
            } else {
              log.warn(`ntfy.sh returned status code: ${res.statusCode}`);
              resolve(); // Don't fail the tunnel start if notification fails
            }
          }
        );

        req.on('error', (error) => {
          log.warn('Failed to send notification to ntfy.sh:', error);
          resolve(); // Don't fail the tunnel start if notification fails
        });

        req.write(postData);
        req.end();
      });
    } catch (error) {
      log.warn('Error in notifyNewUrl:', error);
      // Don't throw, notification failure shouldn't stop tunnel
    }
  }

  /**
   * Check if the tunnel URL is responding with 2xx or 4xx status codes
   */
  private async checkTunnelHealth(): Promise<boolean> {
    if (!this.tunnelUrl) {
      return false;
    }

    const url = this.tunnelUrl; // Capture for TypeScript

    return new Promise((resolve) => {
      const req = https.request(
        url,
        {
          method: 'GET',
          timeout: 10000, // 10 second timeout
        },
        (res) => {
          const statusCode = res.statusCode || 0;
          const isHealthy =
            (statusCode >= 200 && statusCode < 300) || (statusCode >= 400 && statusCode < 500);

          if (isHealthy) {
            log.info(`Tunnel health check passed: ${statusCode}`);
            resolve(true);
          } else {
            log.warn(`Tunnel health check failed: ${statusCode}`);
            resolve(false);
          }
        }
      );

      req.on('error', (error) => {
        log.warn('Tunnel health check error:', error);
        resolve(false);
      });

      req.on('timeout', () => {
        log.warn('Tunnel health check timeout');
        req.destroy();
        resolve(false);
      });

      req.end();
    });
  }

  /**
   * Start health monitoring with periodic checks
   */
  private startHealthMonitoring(): void {
    // Clear any existing interval
    this.stopHealthMonitoring();

    log.info(`Starting health monitoring (interval: ${HEALTH_CHECK_INTERVAL_MS}ms)`);

    this.healthCheckInterval = setInterval(async () => {
      const isHealthy = await this.checkTunnelHealth();

      if (isHealthy) {
        // Reset failure counter on success
        this.consecutiveFailures = 0;
      } else {
        this.consecutiveFailures++;
        log.warn(`Tunnel health check failed (${this.consecutiveFailures}/${FAILURE_THRESHOLD})`);

        if (this.consecutiveFailures >= FAILURE_THRESHOLD) {
          log.error('Tunnel failed health checks, attempting restart...');
          await this.handleTunnelFailure();
        }
      }
    }, HEALTH_CHECK_INTERVAL_MS);
  }

  /**
   * Stop health monitoring
   */
  private stopHealthMonitoring(): void {
    if (this.healthCheckInterval) {
      clearInterval(this.healthCheckInterval);
      this.healthCheckInterval = null;
      log.info('Stopped health monitoring');
    }
  }

  /**
   * Handle tunnel failure by restarting it
   */
  private async handleTunnelFailure(): Promise<void> {
    try {
      log.info('Handling tunnel failure - restarting...');

      // Cancel proactive restart since we're restarting now
      this.cancelProactiveRestart();

      // Stop the current tunnel
      if (this.cloudflaredTunnel) {
        this.cloudflaredTunnel.stop();
        this.cloudflaredTunnel = null;
      }

      // Reset failure counter
      this.consecutiveFailures = 0;

      // Start a new tunnel with the same port and secret
      this.tunnelUrl = await this.startCloudflareTunnel(this.port);

      if (!this.tunnelUrl) {
        throw new Error('Failed to get new tunnel URL');
      }

      // Update config with new URL
      const connectUrl = this.tunnelUrl.replace('https://', '') + ':443';
      if (this.config) {
        this.config.url = connectUrl;
      }

      // No need to regenerate QR code - it contains the ntfy.sh URL which doesn't change (or direct URL which also doesn't need regen)
      log.info('Tunnel restarted successfully with new URL:', this.tunnelUrl);

      // Send notification with new URL (only if using ntfy.sh)
      if (USE_NTFY_SH) {
        await this.notifyNewUrl(this.tunnelUrl);
      }

      // Schedule next proactive restart
      this.scheduleProactiveRestart();
    } catch (error) {
      log.error('Failed to restart tunnel:', error);
      // Will retry on next health check cycle
    }
  }

  /**
   * Schedule a proactive restart of the tunnel after an hour
   */
  private scheduleProactiveRestart(): void {
    // Cancel any existing scheduled restart
    this.cancelProactiveRestart();

    log.info(`Scheduling proactive restart in ${PROACTIVE_RESTART_INTERVAL_MS}ms (1 hour)`);

    this.proactiveRestartTimeout = setTimeout(async () => {
      log.info('Proactive restart triggered - restarting tunnel...');
      await this.handleTunnelFailure();
    }, PROACTIVE_RESTART_INTERVAL_MS);
  }

  /**
   * Cancel scheduled proactive restart
   */
  private cancelProactiveRestart(): void {
    if (this.proactiveRestartTimeout) {
      clearTimeout(this.proactiveRestartTimeout);
      this.proactiveRestartTimeout = null;
      log.info('Cancelled scheduled proactive restart');
    }
  }

  /**
   * Stop the tunnel
   */
  async stop(): Promise<void> {
    // Stop health monitoring
    this.stopHealthMonitoring();

    // Cancel proactive restart
    this.cancelProactiveRestart();

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
    this.tunnelUrl = null;
    this.consecutiveFailures = 0;
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

  /**
   * Clean up QR code files
   */
  async cleanupQRCodes(): Promise<void> {
    try {
      const tempDir = path.join(os.tmpdir(), 'goose-tunnels');
      const files = await fs.readdir(tempDir).catch(() => []);

      for (const file of files) {
        if (file.startsWith('tunnel-qr-')) {
          const filepath = path.join(tempDir, file);
          await fs.unlink(filepath).catch((err) => {
            log.warn('Failed to delete QR code file:', filepath, err);
          });
        }
      }
    } catch (error) {
      log.warn('Failed to cleanup QR codes:', error);
    }
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
  await manager.cleanupQRCodes();
}
