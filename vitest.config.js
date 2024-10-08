import { defineConfig } from 'vitest/config'
import viteConfig from './vite.config'

export default defineConfig({
  ...viteConfig,
  test: {
    environment: 'jsdom',
    setupFiles: './setupTests.js',
    // setupFiles: './setupTests.ts', // Optional, if you need global setups
  },
})
