import { spawn, ChildProcess, execSync } from 'child_process';
import * as fs from 'fs';
import * as path from 'path';
import * as https from 'https';
import * as os from 'os';
import log from './logger';
import { EventEmitter } from 'events';
import { Buffer } from 'buffer';

const RELEASE_BASE = 'https://github.com/cloudflare/cloudflared/releases/';

const LINUX_URL: Record<string, string> = {
  arm64: 'cloudflared-linux-arm64',
  arm: 'cloudflared-linux-arm',
  x64: 'cloudflared-linux-amd64',
  ia32: 'cloudflared-linux-386',
};

const MACOS_URL: Record<string, string> = {
  arm64: 'cloudflared-darwin-arm64.tgz',
  x64: 'cloudflared-darwin-amd64.tgz',
};

const WINDOWS_URL: Record<string, string> = {
  x64: 'cloudflared-windows-amd64.exe',
  ia32: 'cloudflared-windows-386.exe',
};

export class CloudflareTunnel extends EventEmitter {
  private binaryPath: string;
  private process: ChildProcess | null = null;
  private url: string | null = null;

  constructor() {
    super();
    // Store the binary in app's user data directory
    const appDataPath =
      process.env.APPDATA ||
      (process.platform == 'darwin'
        ? path.join(os.homedir(), 'Library', 'Application Support')
        : path.join(os.homedir(), '.local', 'share'));

    const gooseDataPath = path.join(appDataPath, 'Goose');
    this.binaryPath = path.join(
      gooseDataPath,
      'cloudflared',
      process.platform === 'win32' ? 'cloudflared.exe' : 'cloudflared'
    );
  }

  /**
   * Ensure cloudflared binary is installed
   */
  private async ensureBinary(): Promise<void> {
    if (fs.existsSync(this.binaryPath)) {
      log.info('Cloudflared binary already exists at:', this.binaryPath);
      return;
    }

    log.info('Downloading cloudflared binary...');
    await this.downloadBinary();
  }

  /**
   * Download the cloudflared binary for the current platform
   */
  private async downloadBinary(): Promise<void> {
    const platform = process.platform;
    const arch = process.arch;

    let fileUrl: string;
    let fileName: string;

    if (platform === 'linux') {
      fileName = LINUX_URL[arch];
      if (!fileName) throw new Error(`Unsupported Linux architecture: ${arch}`);
      fileUrl = `${RELEASE_BASE}latest/download/${fileName}`;
    } else if (platform === 'darwin') {
      fileName = MACOS_URL[arch] || MACOS_URL['x64']; // Fallback to x64 for older versions
      if (!fileName) throw new Error(`Unsupported macOS architecture: ${arch}`);
      fileUrl = `${RELEASE_BASE}latest/download/${fileName}`;
    } else if (platform === 'win32') {
      fileName = WINDOWS_URL[arch];
      if (!fileName) throw new Error(`Unsupported Windows architecture: ${arch}`);
      fileUrl = `${RELEASE_BASE}latest/download/${fileName}`;
    } else {
      throw new Error(`Unsupported platform: ${platform}`);
    }

    // Ensure directory exists
    const dir = path.dirname(this.binaryPath);
    if (!fs.existsSync(dir)) {
      fs.mkdirSync(dir, { recursive: true });
    }

    // Download the file
    if (platform === 'darwin') {
      // macOS files are .tgz, need to extract
      const tgzPath = `${this.binaryPath}.tgz`;
      await this.download(fileUrl, tgzPath);

      // Extract the tgz file
      execSync(`tar -xzf ${path.basename(tgzPath)}`, { cwd: dir });
      fs.unlinkSync(tgzPath);

      // The extracted file is named 'cloudflared'
      const extractedPath = path.join(dir, 'cloudflared');
      if (extractedPath !== this.binaryPath) {
        fs.renameSync(extractedPath, this.binaryPath);
      }
    } else {
      // Direct download for Linux and Windows
      await this.download(fileUrl, this.binaryPath);
    }

    // Make executable on Unix-like systems
    if (platform !== 'win32') {
      fs.chmodSync(this.binaryPath, '755');
    }

    log.info('Cloudflared binary downloaded successfully to:', this.binaryPath);
  }

  /**
   * Download a file from URL to destination
   */
  private download(url: string, dest: string): Promise<string> {
    return new Promise((resolve, reject) => {
      log.info(`Downloading ${url} to ${dest}`);

      const request = https.get(url, (res) => {
        // Handle redirects
        if (
          res.statusCode &&
          res.statusCode >= 300 &&
          res.statusCode < 400 &&
          res.headers.location
        ) {
          request.destroy();
          return resolve(this.download(res.headers.location, dest));
        }

        if (res.statusCode && res.statusCode >= 200 && res.statusCode < 300) {
          const file = fs.createWriteStream(dest);

          file.on('finish', () => {
            file.close(() => resolve(dest));
          });

          file.on('error', (err) => {
            fs.unlink(dest, () => reject(err));
          });

          res.pipe(file);
        } else {
          request.destroy();
          reject(new Error(`HTTP response with status code: ${res.statusCode}`));
        }
      });

      request.on('error', (err) => {
        reject(err);
      });

      request.end();
    });
  }

  /**
   * Start a tunnel for the specified port
   */
  async start(port: number): Promise<string> {
    if (this.process) {
      throw new Error('Tunnel is already running');
    }

    // Ensure binary is available
    await this.ensureBinary();

    return new Promise((resolve, reject) => {
      // Start cloudflared tunnel
      const args = ['tunnel', '--url', `http://localhost:${port}`];

      log.info('Starting cloudflared with args:', args);

      this.process = spawn(this.binaryPath, args, {
        stdio: ['ignore', 'pipe', 'pipe'],
      });

      this.process.on('error', (error) => {
        log.error('Cloudflared process error:', error);
        this.emit('error', error);
        reject(error);
      });

      this.process.on('exit', (code, signal) => {
        log.info('Cloudflared process exited:', { code, signal });
        this.emit('exit', code, signal);
        this.process = null;
        this.url = null;
      });

      // Listen for the tunnel URL in stdout/stderr
      const handleOutput = (data: Buffer) => {
        const output = data.toString();
        log.info('Cloudflared output:', output);

        // Look for the tunnel URL pattern
        const urlMatch = output.match(/https:\/\/[a-z0-9-]+\.trycloudflare\.com/);
        if (urlMatch && !this.url) {
          this.url = urlMatch[0];
          log.info('Tunnel URL established:', this.url);
          this.emit('url', this.url);
          resolve(this.url);
        }
      };

      if (this.process.stdout) {
        this.process.stdout.on('data', handleOutput);
      }

      if (this.process.stderr) {
        this.process.stderr.on('data', handleOutput);
      }

      // Timeout if URL not received
      setTimeout(() => {
        if (!this.url) {
          this.stop();
          reject(new Error('Timeout waiting for tunnel URL'));
        }
      }, 30000); // 30 second timeout
    });
  }

  /**
   * Stop the tunnel
   */
  stop(): void {
    if (this.process) {
      log.info('Stopping cloudflared tunnel');
      this.process.kill('SIGINT');
      this.process = null;
      this.url = null;
    }
  }

  /**
   * Get the current tunnel URL
   */
  getUrl(): string | null {
    return this.url;
  }

  /**
   * Check if tunnel is running
   */
  isRunning(): boolean {
    return this.process !== null && !this.process.killed;
  }
}
