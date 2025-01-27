/**
 * @fileoverview MUI theme configuration and custom Paper components.
 * This module defines the global Material-UI theme and custom styled components
 * used throughout the application for consistent styling.
 * 
 * @module theme
 */

import { Paper } from '@mui/material';
import { createTheme } from '@mui/material/styles';
import { styled } from '@mui/system';

/**
 * MUI theme overrides for the entire project.
 * Defines global styles for Material-UI components including
 * custom background colors and text colors.
 * 
 * @type {import("@mui/material").Theme}
 * @property {Object} components - Component-specific style overrides
 * @property {Object} components.MuiPaper - Paper component styles
 * @property {Object} components.MuiTextField - TextField component styles
 */
export const theme = createTheme({
  components: {
    MuiPaper: {
      styleOverrides: {
        root: {
          backgroundColor: '#1e293b',
          color: '#ffffff',
        },
      },
    },
    MuiTextField: {
      styleOverrides: {
        root: {
          color: '#ffffff',
        },
      },
    },
  },
});

/**
 * A Paper component with a semi-transparent background and white text.
 * Suitable as a container throughout the project.
 * 
 * @component
 * @example
 * return (
 *   <SoftPaper>
 *     <Typography>Content with semi-transparent background</Typography>
 *   </SoftPaper>
 * )
 */
export const SoftPaper = styled(Paper)`
  background-color: rgba(30, 41, 59, 0.9) !important;
  color: #ffffff !important;
  border-radius: 1rem !important; 
  padding: 1rem;
  overflow: hidden;
`;

/**
 * A Paper component with an opaque background and white text.
 * Extends SoftPaper to provide a fully solid background.
 * Use this when you need a solid background instead of semi-transparent.
 * 
 * @component
 * @example
 * return (
 *   <SolidPaper>
 *     <Typography>Content with solid background</Typography>
 *   </SolidPaper>
 * )
 */
export const SolidPaper = styled(SoftPaper)`
  background-color: rgba(30, 41, 59, 1) !important;
`;

/**
 * A Paper variant with a partially opaque background.
 * Specifically designed for background use cases where
 * content needs to be visible but distinguished from the foreground.
 * 
 * @component
 * @example
 * return (
 *   <BackGroundPaper>
 *     <Typography>Background content</Typography>
 *   </BackGroundPaper>
 * )
 */
export const BackGroundPaper = styled(Paper)`
  background-color: rgba(30, 41, 59, 0.5);
  color: #ffffff;
  border-radius: 0;
  margin: 0;
  padding: 0;
  width: 100%;
  max-width: 100%;
  box-sizing: border-box;
`;
