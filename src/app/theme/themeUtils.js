/**
 * @fileoverview Theme utility hooks and helpers for consistent styling
 * @module themeUtils
 */

import { useTheme } from '@mui/material/styles';
import { colors } from './standardTheme';

/**
 * Custom hook to access theme colors in functional components
 * @returns {Object} Theme colors object
 * @example
 * const MyComponent = () => {
 *   const themeColors = useThemeColors();
 *   return <div style={{ color: themeColors.text.primary }}>Hello</div>;
 * };
 */
export const useThemeColors = () => {
  return colors;
};

/**
 * Get inline style object for primary text
 * @returns {Object} Style object with primary text color
 */
export const getPrimaryTextStyle = () => ({
  color: colors.text.primary,
});

/**
 * Get inline style object for secondary text
 * @returns {Object} Style object with secondary text color
 */
export const getSecondaryTextStyle = () => ({
  color: colors.text.secondary,
});

/**
 * Get inline style object for muted text
 * @returns {Object} Style object with muted text color
 */
export const getMutedTextStyle = () => ({
  color: colors.text.muted,
});

/**
 * Get inline style object for disabled text
 * @returns {Object} Style object with disabled text color
 */
export const getDisabledTextStyle = () => ({
  color: colors.text.disabled,
});

/**
 * Get inline style object with custom text color from theme
 * @param {'primary' | 'secondary' | 'muted' | 'disabled'} variant - Text variant
 * @returns {Object} Style object with specified text color
 */
export const getTextStyle = (variant = 'primary') => ({
  color: colors.text[variant] || colors.text.primary,
});

/**
 * Get inline style object for primary background
 * @returns {Object} Style object with primary background color
 */
export const getPrimaryBgStyle = () => ({
  backgroundColor: colors.background.default,
});

/**
 * Get inline style object for paper background
 * @returns {Object} Style object with paper background color
 */
export const getPaperBgStyle = () => ({
  backgroundColor: colors.background.paper,
});

/**
 * Tailwind class name helper for text colors
 * Returns the appropriate Tailwind class for text color
 * @param {'primary' | 'secondary' | 'muted' | 'disabled'} variant - Text variant
 * @returns {string} Tailwind class name
 */
export const getTextClass = (variant = 'primary') => {
  const classMap = {
    primary: 'text-text-primary',
    secondary: 'text-text-secondary',
    muted: 'text-text-muted',
    disabled: 'text-text-disabled',
  };
  return classMap[variant] || classMap.primary;
};

/**
 * Tailwind class name helper for background colors
 * Returns the appropriate Tailwind class for background color
 * @param {'default' | 'darker' | 'lighter' | 'paper'} variant - Background variant
 * @returns {string} Tailwind class name
 */
export const getBgClass = (variant = 'default') => {
  const classMap = {
    default: 'bg-background',
    darker: 'bg-background-darker',
    lighter: 'bg-background-lighter',
    paper: 'bg-background-paper',
  };
  return classMap[variant] || classMap.default;
};

/**
 * Common component style presets
 */
export const componentStyles = {
  card: {
    backgroundColor: colors.background.paper,
    color: colors.text.primary,
    borderRadius: '1rem',
    padding: '1rem',
  },
  button: {
    backgroundColor: colors.primary.main,
    color: colors.text.primary,
  },
  input: {
    backgroundColor: colors.background.lighter,
    color: colors.text.primary,
    borderColor: colors.text.disabled,
  },
};

export default {
  colors,
  useThemeColors,
  getPrimaryTextStyle,
  getSecondaryTextStyle,
  getMutedTextStyle,
  getDisabledTextStyle,
  getTextStyle,
  getPrimaryBgStyle,
  getPaperBgStyle,
  getTextClass,
  getBgClass,
  componentStyles,
};
