/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Primary brand colors
        primary: {
          DEFAULT: '#0080b3',
          dark: '#005f8a',
          light: '#1a9fd1',
        },
        // Dark theme background colors
        background: {
          DEFAULT: '#1e293b',
          darker: '#0f172a',
          lighter: '#334155',
        },
        // Text colors for dark theme
        text: {
          primary: '#ffffff',
          secondary: '#e2e8f0',
          muted: '#94a3b8',
          disabled: '#64748b',
        },
      },
    },
  },
  plugins: [],
  // Prevent Tailwind from conflicting with MUI
  corePlugins: {
    preflight: false,
  },
}
