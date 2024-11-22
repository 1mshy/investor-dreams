import React, { useEffect } from 'react';

const TickerTape = () => {
    useEffect(() => {
        const script = document.createElement('script');
        script.src = 'https://s3.tradingview.com/external-embedding/embed-widget-ticker-tape.js';
        script.async = true;
        script.innerHTML = JSON.stringify({
            symbols: [
                { proName: "NASDAQ:AAPL", title: "Apple" },
                { proName: "NASDAQ:GOOGL", title: "Google" },
                { proName: "NASDAQ:MSFT", title: "Microsoft" },
            ],
            colorTheme: "dark",
            isTransparent: false,
            displayMode: "adaptive",
        });
        document.querySelector('.ticker-tape-container').appendChild(script);
    }, []);

    return <div className="ticker-tape-container"></div>;
};

export default TickerTape;
