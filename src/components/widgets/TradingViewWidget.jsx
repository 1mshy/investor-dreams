/**
 * @fileoverview TradingView advanced chart widget component.
 * Implements an advanced chart widget from TradingView that displays
 * interactive price charts with technical indicators.
 */

import React, { useEffect, useRef, memo } from 'react';

/**
 * @typedef {Object} ChartConfig
 * @property {boolean} autosize - Whether chart should auto-size to container
 * @property {boolean} hide_top_toolbar - Whether to hide the top toolbar
 * @property {boolean} hide_side_toolbar - Whether to hide the side toolbar
 * @property {boolean} withdateranges - Whether to show date range selector
 * @property {boolean} save_image - Whether to allow saving chart images
 * @property {boolean} details - Whether to show detailed information
 * @property {boolean} allow_symbol_change - Whether to allow symbol changes
 * @property {boolean} calendar - Whether to show calendar events
 * @property {'D'} interval - Chart interval (D for daily)
 * @property {string} symbol - Trading symbol to display
 * @property {'Etc/UTC'} timezone - Chart timezone
 * @property {'dark' | 'light'} theme - Chart theme
 * @property {'1'} style - Chart style preset
 * @property {'en'} locale - Chart language
 * @property {string} range - Time range to display
 * @property {string[]} studies - Technical indicators to show
 * @property {string} support_host - TradingView support host URL
 */

/**
 * TradingViewWidget component that displays an advanced chart.
 * Renders an interactive price chart with customizable timeframes
 * and technical indicators.
 * 
 * @component
 * @example
 * return (
 *   <TradingViewWidget 
 *     symbol="AAPL" 
 *     range="1M"
 *   />
 * )
 * 
 * @param {Object} props - Component props
 * @param {string} [props.symbol='AAPL'] - The trading symbol to display (e.g., "AAPL", "MSFT")
 * @param {string} [props.range='12M'] - The time range to display (e.g., "1D", "5D", "1M", "3M", "6M", "YTD", "1Y", "5Y", "ALL")
 * @returns {JSX.Element} The rendered chart widget
 */
function TradingViewWidget({ symbol = 'AAPL', range = '12M' }) {
  /** @type {React.RefObject<HTMLDivElement>} */
  const containerRef = useRef(null);

  useEffect(() => {
    if (!containerRef.current) return;

    // Reset the container
    containerRef.current.innerHTML = '';

    /** @type {ChartConfig} */
    const chartConfig = {
      autosize: true,
      hide_top_toolbar: false,
      hide_side_toolbar: false,
      withdateranges: true,
      save_image: false,
      details: true,
      allow_symbol_change: false,
      calendar: false,
      interval: 'D',
      symbol,
      timezone: 'Etc/UTC',
      theme: 'dark',
      style: '1',
      locale: 'en',
      range,
      studies: ['STD;RSI'],
      support_host: 'https://www.tradingview.com',
    };

    /**
     * Creates and configures the TradingView script element.
     * Generates a script tag with the required configuration for the TradingView widget.
     * 
     * @function
     * @returns {HTMLScriptElement} Configured script element ready to be appended to the DOM
     */
    const createChartScript = () => {
      const script = document.createElement('script');
      script.src = 'https://s3.tradingview.com/external-embedding/embed-widget-advanced-chart.js';
      script.type = 'text/javascript';
      script.async = true;
      script.innerHTML = JSON.stringify(chartConfig);
      return script;
    };

    containerRef.current.appendChild(createChartScript());
  }, [range, symbol]);

  return (
    <div
      className="tradingview-widget-container"
      ref={containerRef}
      style={{
        display: 'flex',
        flex: 1,
        width: '100%',
        height: '100%',
      }}
    >
      <div className="tradingview-widget-container__widget" />
    </div>
  );
}

// Memoize to prevent unnecessary re-renders when parent components update
export default memo(TradingViewWidget);
