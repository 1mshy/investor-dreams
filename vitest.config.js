import { defineConfig } from 'vitest/config'
import viteConfig from './vite.config'
import jsconfigPaths from 'vite-jsconfig-paths';

export default defineConfig({
  ...viteConfig,
  plugins: [jsconfigPaths()],
  resolve: {
    alias: {
      '@': '/src', // Ensure @ resolves to the correct path
    },
  },
  test: {
    globals: true,
    environment: 'jsdom',
    setupFiles: './setupTests.js',
    // setupFiles: './setupTests.ts', // Optional, if you need global setups
  },
})