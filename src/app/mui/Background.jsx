import React from 'react';
import { Box } from '@mui/material';

const Background = ({ children }) => {
  return (
    <Box
      sx={[
        {
          justifyContent: 'center',
          minHeight: '100%',
        //   position: 'relative', // Required for pseudo-elements
          overflow: 'hidden', // Prevent content spilling
        },
        (theme) => ({
          '&::before': {
            content: '""',
            display: 'block',
            position: 'absolute',
            zIndex: -1,
            inset: 0,
            backgroundImage:
              'radial-gradient(at 50% 50%, hsla(210, 100%, 16%, 0.5), hsl(220, 30%, 5%))',
            backgroundRepeat: 'no-repeat',
            ...theme.applyStyles?.('dark', {
              backgroundImage:
                'radial-gradient(at 50% 50%, hsla(210, 100%, 16%, 0.5), hsl(220, 30%, 5%))',
            }),
          },
        }),
      ]}
    >
      {children}
    </Box>
  );
};

export default Background;
