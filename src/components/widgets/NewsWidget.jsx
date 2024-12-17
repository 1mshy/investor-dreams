import React, { useEffect } from 'react';
import "../../app/css/tradingview.css";

const NewsWidget = () => {
  useEffect(() => {
    // Dynamically load the TradingView script
    const script = document.createElement('script');
    script.src = 'https://s3.tradingview.com/external-embedding/embed-widget-timeline.js';
    script.async = true;
    script.innerHTML = JSON.stringify({
      feedMode: 'all_symbols',
      isTransparent: false,
      displayMode: 'adaptive',
      height: 550,
      width: 350,
      colorTheme: 'dark',
      locale: 'en',
    });

    // Append the script to the container
    const container = document.querySelector('.tradingview-widget-container__widget');
    container.appendChild(script);
    

    // Wait for the element to be created
    const interval = setInterval(() => {
      const bodyElement = container.querySelector('.tv-embed-widget-wrapper__body');
      if (bodyElement) {
        // Modify the element
        bodyElement.style.backgroundColor = 'inherit !important'; // Example change
        bodyElement.style.border = 'none !important'; // Example change

        // Stop checking once the element is found
        clearInterval(interval);
      }
    }, 1000);

    // Clean up on component unmount
    return () => {
      container.innerHTML = '';
      clearInterval(interval);
    };
  }, []);

  /**
   * The TradingView widget container uses a custom overlay
   * found in the tradingview.css file.
   */

  return (
    <div className="tradingview-widget-container" style={{ width: '350px', height: '550px' }}>
      <div className="custom-overlay"></div>
      <div className="tradingview-widget-container__widget"></div>
    </div>
  );
  
};

export default NewsWidget;
