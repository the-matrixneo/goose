import { spawn, ChildProcess } from 'child_process';
import * as QRCode from 'qrcode';
import * as crypto from 'crypto';
import { Buffer } from 'node:buffer';
import log from './logger';
import path from 'node:path';
import fs from 'node:fs/promises';
import os from 'node:os';

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
}

export class TunnelManager {
  private tunnelProcess: ChildProcess | null = null;
  private config: TunnelConfig | null = null;
  private isStarting = false;
  private tunnelUrl: string | null = null;

  constructor() {}

  /**
   * Start a localtunnel for the given port with a secret
   */
  async start(port: number, secret?: string): Promise<TunnelConnectionInfo> {
    if (this.tunnelProcess) {
      throw new Error('Tunnel is already running');
    }

    if (this.isStarting) {
      throw new Error('Tunnel is already starting');
    }

    this.isStarting = true;

    try {
      // Generate secret if not provided
      const tunnelSecret = secret || crypto.randomBytes(24).toString('base64');

      this.config = {
        port,
        secret: tunnelSecret,
      };

      // Start localtunnel
      log.info(`Starting localtunnel on port ${port}`);
      this.tunnelUrl = await this.startLocaltunnel(port);

      if (!this.tunnelUrl) {
        throw new Error('Failed to get tunnel URL');
      }

      // Format connection URL (remove https:// and add :443)
      const connectUrl = this.tunnelUrl.replace('https://', '') + ':443';

      // Create connection config for QR code
      const configData = {
        url: connectUrl,
        secret: tunnelSecret,
      };

      // Create app URL for deep linking
      const configJson = JSON.stringify(configData);
      const urlEncodedConfig = encodeURIComponent(configJson);
      const appUrl = `goosechat://configure?data=${urlEncodedConfig}`;

      // Generate QR code
      const qrCodeDataUrl = await this.generateQRCode(appUrl);

      // Save QR code to file
      const qrCodePath = await this.saveQRCodeToFile(qrCodeDataUrl);

      this.config.url = connectUrl;
      this.config.qrCodePath = qrCodePath;
      this.config.qrCodeDataUrl = qrCodeDataUrl;

      const result: TunnelConnectionInfo = {
        url: connectUrl,
        secret: tunnelSecret,
        appUrl,
        qrCodeDataUrl,
        qrCodePath,
      };

      log.info('Tunnel started successfully:', {
        url: connectUrl,
        qrCodePath,
      });

      return result;
    } finally {
      this.isStarting = false;
    }
  }

  /**
   * Start localtunnel process and capture the URL
   */
  private startLocaltunnel(port: number): Promise<string> {
    return new Promise((resolve, reject) => {
      const lt = spawn('npx', ['localtunnel', '--port', port.toString()], {
        shell: true,
        stdio: ['ignore', 'pipe', 'pipe'],
      });

      this.tunnelProcess = lt;

      let output = '';
      let errorOutput = '';
      let urlFound = false;
      const timeout = setTimeout(() => {
        if (!urlFound) {
          this.stop();
          reject(new Error('Timeout waiting for tunnel URL'));
        }
      }, 30000); // 30 second timeout

      lt.stdout?.on('data', (data) => {
        const text = data.toString();
        output += text;
        log.info('localtunnel stdout:', text);

        // Look for the URL in the output
        const urlMatch = text.match(/your url is: (https:\/\/.*\.loca\.lt)/);
        if (urlMatch && !urlFound) {
          urlFound = true;
          clearTimeout(timeout);
          this.tunnelUrl = urlMatch[1];
          resolve(urlMatch[1]);
        }
      });

      lt.stderr?.on('data', (data) => {
        errorOutput += data.toString();
        log.error('localtunnel stderr:', data.toString());
      });

      lt.on('error', (error) => {
        clearTimeout(timeout);
        log.error('Failed to start localtunnel:', error);
        this.tunnelProcess = null;
        reject(error);
      });

      lt.on('close', (code) => {
        clearTimeout(timeout);
        log.info(`localtunnel process exited with code ${code}`);
        this.tunnelProcess = null;

        if (!urlFound) {
          reject(
            new Error(
              `Localtunnel exited without providing URL. Output: ${output}, Error: ${errorOutput}`
            )
          );
        }
      });
    });
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
   * Save QR code to a file
   */
  private async saveQRCodeToFile(dataUrl: string): Promise<string> {
    try {
      // Create temp directory for QR codes
      const tempDir = path.join(os.tmpdir(), 'goose-tunnels');
      await fs.mkdir(tempDir, { recursive: true });

      // Generate unique filename
      const timestamp = Date.now();
      const randomStr = crypto.randomBytes(4).toString('hex');
      const filename = `tunnel-qr-${timestamp}-${randomStr}.png`;
      const filepath = path.join(tempDir, filename);

      // Extract base64 data from data URL
      const base64Data = dataUrl.replace(/^data:image\/png;base64,/, '');
      const buffer = Buffer.from(base64Data, 'base64');

      // Write to file
      await fs.writeFile(filepath, buffer);

      log.info('QR code saved to:', filepath);
      return filepath;
    } catch (error) {
      log.error('Failed to save QR code to file:', error);
      throw new Error('Failed to save QR code');
    }
  }

  /**
   * Stop the tunnel
   */
  stop(): void {
    if (this.tunnelProcess) {
      log.info('Stopping localtunnel');
      try {
        this.tunnelProcess.kill();
      } catch (error) {
        log.error('Error stopping tunnel:', error);
      }
      this.tunnelProcess = null;
    }

    this.config = null;
    this.tunnelUrl = null;
  }

  /**
   * Get current tunnel status
   */
  isRunning(): boolean {
    return this.tunnelProcess !== null;
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
export async function startTunnel(port: number, secret?: string): Promise<TunnelConnectionInfo> {
  const manager = getTunnelManager();
  return manager.start(port, secret);
}

/**
 * Stop the current tunnel
 */
export function stopTunnel(): void {
  const manager = getTunnelManager();
  manager.stop();
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
  manager.stop();
  await manager.cleanupQRCodes();
}
