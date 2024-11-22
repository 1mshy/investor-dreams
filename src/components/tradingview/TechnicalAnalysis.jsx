import React, { useEffect } from 'react';

const TechnicalAnalysis = () => {
    useEffect(() => {
        const script = document.createElement('script');
        script.src = 'https://s3.tradingview.com/external-embedding/embed-widget-technical-analysis.js';
        script.async = true;
        script.innerHTML = JSON.stringify({
            symbol: "NASDAQ:AAPL",
            interval: "1D",
            isTransparent: false,
            colorTheme: "dark",

        });
        document.querySelector('.technical-analysis-container').appendChild(script);
    }, []);

    return <div className="technical-analysis-container"></div>;
};

export default TechnicalAnalysis;
