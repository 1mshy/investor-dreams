import { createTheme } from '@mui/material/styles';

/**
 * Changes the theme for each MUI component in the whole project from here
 */
const theme = createTheme({
  components: {
    MuiPaper: {
      styleOverrides: {
        root: {
          backgroundColor: '#1e293b',
          color: '#ffffff', // text colour
          borderRadius: '1rem',
          padding: '1rem',
          overflow: 'hidden',
        },
      },
    },
  },
});

export default theme;