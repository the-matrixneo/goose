import { defineConfig } from 'vite';

// https://vitejs.dev/config
export default defineConfig({
  // No external dependencies needed anymore!
  // cloudflared binary is downloaded at runtime, not bundled
});
