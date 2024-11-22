import React, { useEffect } from 'react';

const FundamentalData = () => {
    useEffect(() => {
        const script = document.createElement('script');
        script.src = 'https://s3.tradingview.com/external-embedding/embed-widget-fundamental-data.js';
        script.async = true;
        script.innerHTML = JSON.stringify({
            symbol: "NASDAQ:AAPL",
            isTransparent: false,
            colorTheme: "dark",

        });
        document.querySelector('.fundamental-data-container').appendChild(script);
    }, []);

    return <div className="fundamental-data-container"></div>;
};

export default FundamentalData;
