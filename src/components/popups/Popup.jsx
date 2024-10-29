import React, { useState, forwardRef, useImperativeHandle } from 'react';

const Popup = forwardRef(({ onSubmit }, ref) => {
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

  return (
    isOpen && (
      <div style={styles.overlay}>
        <div style={styles.popup}>
          <h3>Enter Data</h3>
          <input
            type="text"
            value={inputData}
            onChange={(e) => setInputData(e.target.value)}
            style={styles.input}
          />
          <button onClick={handleSubmit} style={styles.button}>Submit</button>
          <button onClick={() => setIsOpen(false)} style={styles.button}>Cancel</button>
        </div>
      </div>
    )
  );
});

const styles = {
  overlay: {
    position: 'fixed',
    top: 0,
    left: 0,
    width: '100%',
    height: '100%',
    backgroundColor: 'rgba(0, 0, 0, 0.5)',
    display: 'flex',
    justifyContent: 'center',
    alignItems: 'center',
    zIndex: 1000,
  },
  popup: {
    backgroundColor: '#fff',
    padding: '20px',
    borderRadius: '8px',
    width: '300px',
    boxShadow: '0 4px 8px rgba(0, 0, 0, 0.1)',
    textAlign: 'center',
  },
  input: {
    width: '100%',
    padding: '8px',
    margin: '10px 0',
  },
  button: {
    padding: '8px 16px',
    margin: '5px',
    cursor: 'pointer',
  },
};

export default Popup;
