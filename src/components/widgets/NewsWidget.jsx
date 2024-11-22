import React, { useEffect } from 'react';

import "../../app/css/tradingview.css"

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
      colorTheme: 'dark',
      locale: 'en',
    });

    // Append the script to the container
    const container = document.querySelector('.tradingview-widget-container__widget');
    container.appendChild(script);

    // Clean up on component unmount
    return () => {
      container.innerHTML = '';
    };
  }, []);

  return (
    <div className="tradingview-widget-container">
      <div className="tradingview-widget-container__widget"></div>
    </div>
  );
};

export default NewsWidget;
