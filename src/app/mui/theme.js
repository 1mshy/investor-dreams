import { Paper } from '@mui/material';
import { createTheme } from '@mui/material/styles';
import styled from 'styled-components';

/**
 * Changes the theme for each MUI component in the whole project from here
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
 * Paper that has a soft background color and text color
 looks good for use in the whole project as a container
 */
export const SoftPaper = styled(Paper)`
 background-color: rgba(30, 41, 59, 0.9) !important;
 color: #ffffff !important;
 border-radius: 1rem !important; 
 padding: 1rem;
 overflow: hidden;
`;

/**
 * Paper that has a soft background color and text color
 looks good for use in the whole project as a container
 */
export const SolidPaper = styled(SoftPaper)`
 background-color: rgba(30, 41, 59, 1) !important;
`;

export const BackGroundPaper = styled(SoftPaper)`
  background-color: rgba(30, 41, 59, 0.5) !important;
  `;