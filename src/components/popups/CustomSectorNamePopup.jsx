import { SolidPaper } from '@/app/mui/theme';
import { Backdrop, Button, Dialog, TextField } from '@mui/material';
import React, { forwardRef, useImperativeHandle, useState } from 'react';
import { Transition } from '../../app/funcs/themes';

const CustomSectorNamePopup = forwardRef(({ onSubmit }, ref) => {
  const [inputData, setInputData] = useState('');

  // Expose `open` and `close` methods to the parent component
  const [isOpen, setIsOpen] = useState(false);
  useImperativeHandle(ref, () => ({
    open: () => setIsOpen(true),
    close: () => setIsOpen(false)
  }));

  const handleSubmit = () => {

    onSubmit(inputData);  // Send data back to parent
    setIsOpen(false);     // Close the popup
    setInputData('');     // Clear input after submit
  };

  return <>
    <Button onClick={() => { setIsOpen(true) }}>Save</Button>
    <Backdrop open={isOpen} data-tauri-drag-region invisible={true} onClick={(e, reason) => {
      console.log('fsdfsdf')
      setIsOpen(false)
    }}>
      <Dialog
        data-tauri-drag-region
        open={isOpen}
        aria-labelledby="responsive-dialog-title"
        TransitionComponent={Transition}
        maxWidth="sm"
        fullWidth
        PaperComponent={SolidPaper}
        sx={{
          justifyContent: "center",
          alignItems: "center",
        }}
        onClick={(e) => {
          e.stopPropagation();
          e.preventDefault();
          e.nativeEvent.stopImmediatePropagation();
        }}
      >
        <h3>Custom Sector Name</h3>
        <TextField
          type="text"
          placeholder='Name to add to playground'
          value={inputData}
          onChange={(e) => setInputData(e.target.value)}
        />
        <div style={{ display: "fle" }}>

          <Button type="submit" variant="contained" onClick={handleSubmit}>
            Submit
          </Button>
          <Button onClick={() => setIsOpen(false)} >Cancel</Button>
        </div>
      </Dialog>
    </Backdrop>
  </>
});

export default CustomSectorNamePopup;
