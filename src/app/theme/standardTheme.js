/**
 * @fileoverview Standardized theme configuration for the entire app.
 * This module provides a centralized dark theme with consistent colors
 * across all components, both MUI and custom CSS.
 * 
 * @module standardTheme
 */

import { createTheme } from '@mui/material/styles';

/**
 * Color palette constants for the standardized dark theme
 */
export const colors = {
  // Primary brand colors
  primary: {
    main: '#0080b3',
    dark: '#005f8a',
    light: '#1a9fd1',
  },
  // Background colors for dark theme
  background: {
    default: '#1e293b',
    darker: '#0f172a',
    lighter: '#334155',
    paper: '#1e293b',
    paperLight: 'rgba(30, 41, 59, 0.9)',
    paperSolid: 'rgba(30, 41, 59, 1)',
    paperTransparent: 'rgba(30, 41, 59, 0.5)',
  },
  // Text colors optimized for dark backgrounds
  text: {
    primary: '#ffffff',
    secondary: '#e2e8f0',
    muted: '#94a3b8',
    disabled: '#64748b',
  },
};

/**
 * Standardized MUI theme for the entire application.
 * Ensures consistent dark theme styling across all MUI components.
 */
export const standardTheme = createTheme({
  palette: {
    mode: 'dark',
    primary: {
      main: colors.primary.main,
      dark: colors.primary.dark,
      light: colors.primary.light,
    },
    background: {
      default: colors.background.default,
      paper: colors.background.paper,
    },
    text: {
      primary: colors.text.primary,
      secondary: colors.text.secondary,
      disabled: colors.text.disabled,
    },
  },
  typography: {
    fontFamily: 'Arial, Helvetica, sans-serif',
    allVariants: {
      color: colors.text.primary,
    },
  },
  components: {
    MuiPaper: {
      styleOverrides: {
        root: {
          backgroundColor: colors.background.paper,
          color: colors.text.primary,
        },
      },
    },
    MuiTextField: {
      styleOverrides: {
        root: {
          '& .MuiInputBase-input': {
            color: colors.text.primary,
          },
          '& .MuiInputLabel-root': {
            color: colors.text.secondary,
          },
        },
      },
    },
    MuiTypography: {
      styleOverrides: {
        root: {
          color: colors.text.primary,
        },
      },
    },
    MuiButton: {
      styleOverrides: {
        root: {
          textTransform: 'none',
        },
        contained: {
          backgroundColor: colors.primary.main,
          color: colors.text.primary,
          '&:hover': {
            backgroundColor: colors.primary.dark,
          },
        },
      },
    },
    MuiCheckbox: {
      styleOverrides: {
        root: {
          color: colors.text.secondary,
          '&.Mui-checked': {
            color: colors.primary.main,
          },
        },
      },
    },
    MuiSelect: {
      styleOverrides: {
        root: {
          color: colors.text.primary,
        },
      },
    },
    MuiMenuItem: {
      styleOverrides: {
        root: {
          color: colors.text.primary,
        },
      },
    },
    MuiInputLabel: {
      styleOverrides: {
        root: {
          color: colors.text.secondary,
        },
      },
    },
  },
});

/**
 * CSS-in-JS style utilities for consistent theming
 */
export const themeStyles = {
  // Common text styles
  primaryText: {
    color: colors.text.primary,
  },
  secondaryText: {
    color: colors.text.secondary,
  },
  mutedText: {
    color: colors.text.muted,
  },
  
  // Common background styles
  primaryBg: {
    backgroundColor: colors.background.default,
  },
  paperBg: {
    backgroundColor: colors.background.paper,
  },
  
  // Common button styles
  primaryButton: {
    backgroundColor: colors.primary.main,
    color: colors.text.primary,
  },
};

export default standardTheme;
