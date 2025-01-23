/**
 * @fileoverview TradingView news timeline widget component.
 * Implements a news feed widget from TradingView that displays
 * real-time market news and updates.
 */

import React, { useEffect } from 'react';
import '../../app/css/tradingview.css';

/**
 * @typedef {Object} TradingViewConfig
 * @property {'all_symbols'} feedMode - Mode for news feed display
 * @property {boolean} isTransparent - Whether widget background is transparent
 * @property {'adaptive'} displayMode - How widget adapts to container size
 * @property {number} height - Widget height in pixels
 * @property {number} width - Widget width in pixels
 * @property {'dark' | 'light'} colorTheme - Widget color theme
 * @property {'en'} locale - Widget language code
 */

/**
 * NewsWidget component that displays a TradingView news timeline.
 * Dynamically loads TradingView's widget script and applies custom styling.
 * The widget displays market news and updates in real-time.
 *
 * @component
 * @example
 * return (
 *   <div className="news-section">
 *     <NewsWidget />
 *   </div>
 * )
 *
 * @returns {JSX.Element} The rendered news widget container
 */
const NewsWidget = () => {
  useEffect(() => {
    /** @type {TradingViewConfig} */
    const widgetConfig = {
      feedMode: 'all_symbols',
      isTransparent: false,
      displayMode: 'adaptive',
      height: 550,
      width: 350,
      colorTheme: 'dark',
      locale: 'en',
    };

    /**
     * Creates and configures the TradingView script element.
     * Sets up the script tag with the required configuration for the news widget.
     * 
     * @function
     * @returns {HTMLScriptElement} Configured script element ready to be appended to the DOM
     */
    const createWidgetScript = () => {
      const script = document.createElement('script');
      script.src = 'https://s3.tradingview.com/external-embedding/embed-widget-timeline.js';
      script.async = true;
      script.innerHTML = JSON.stringify(widgetConfig);
      return script;
    };

    /**
     * Applies custom styles to the widget body element.
     * Periodically checks for the body element's existence and applies styles.
     * 
     * @function
     * @param {HTMLElement} container - The widget container element
     * @returns {number} The interval ID for cleanup
     */
    const applyCustomStyles = (container) => {
      return setInterval(() => {
        const bodyElement = container.querySelector('.tv-embed-widget-wrapper__body');
        if (bodyElement) {
          bodyElement.style.backgroundColor = 'inherit !important';
          bodyElement.style.border = 'none !important';
          clearInterval(interval);
        }
      }, 1000);
    };

    // Append the script to the container
    const container = document.querySelector('.tradingview-widget-container__widget');
    if (container) {
      container.appendChild(createWidgetScript());

      // Apply custom styles and clean up on unmount
      const interval = applyCustomStyles(container);
      return () => {
        container.innerHTML = '';
        clearInterval(interval);
      };
    }
  }, []);

  return (
    <div className="tradingview-widget-container" style={{ width: '350px', height: '550px' }}>
      <div className="custom-overlay"></div>
      <div className="tradingview-widget-container__widget"></div>
    </div>
  );
};

export default NewsWidget;
