import React from 'react';
import { Box, IconButton } from '@mui/material';
import { Circle } from '@mui/icons-material';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { invoke } from '@tauri-apps/api/core';


const Background = ({ children }) => {
  return (
    <Box
      sx={[
        {
          justifyContent: 'center',
          minHeight: '100%',
          overflow: 'hidden',
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
      <IconButton
        style={{ position: 'absolute', top: '-0.3rem', right: '-0.3rem' }}
        onClick={async () => {
            await invoke("close_window");
        }}
      >
        <Circle fontSize="small" color="error" />
      </IconButton>
      {children}
    </Box>
  );
};

export default Background;
