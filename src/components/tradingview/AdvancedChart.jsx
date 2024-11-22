import React, { useEffect } from 'react';

const AdvancedChart = () => {
    useEffect(() => {
        const script = document.createElement('script');
        script.src = 'https://s3.tradingview.com/external-embedding/embed-widget-advanced-chart.js';
        script.async = true;
        script.innerHTML = JSON.stringify({
            symbol: "NASDAQ:AAPL",
            theme: "dark",
            isTransparent: false,
            autosize: true,
            colorTheme: "dark",

        });
        document.querySelector('.advanced-chart-container').appendChild(script);
    }, []);

    return <div className="advanced-chart-container" style={{ height: "500px" }}></div>;
};

export default AdvancedChart;
