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
  background-color: #1e293b;
  color: #ffffff;
  border-radius: 1rem;
  padding: 1rem;
  overflow: hidden;
`;
