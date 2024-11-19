import { Paper } from '@mui/material';
import { createTheme } from '@mui/material/styles';
import { styled } from '@mui/system';

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
    // MuiCssBaseline: {
    //   styleOverrides: {
    //     body: {
    //       position: 'relative', // Enable pseudo-elements
    //       minHeight: '100%',
    //       '&::before': {
    //         content: '""',
    //         display: 'block',
    //         position: 'absolute',
    //         inset: 0,
    //         zIndex: -1,
    //         backgroundImage:
    //           'radial-gradient(ellipse at 50% 50%, hsl(210, 100%, 97%), hsl(0, 0%, 100%))',
    //         backgroundRepeat: 'no-repeat',
    //       },
    //     },
    //   },
    // },
  }
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