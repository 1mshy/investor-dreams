import React, { useEffect } from 'react';

const SymbolInfo = () => {
    useEffect(() => {
        const script = document.createElement('script');
        script.src = 'https://s3.tradingview.com/external-embedding/embed-widget-symbol-info.js';
        script.async = true;
        script.innerHTML = JSON.stringify({
            symbol: "NASDAQ:AAPL",
            isTransparent: false,
            colorTheme: "dark",

        });
        document.querySelector('.symbol-info-container').appendChild(script);
    }, []);

    return <div className="symbol-info-container"></div>;
};

export default SymbolInfo;
