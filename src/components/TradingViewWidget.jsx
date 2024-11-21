// TradingViewWidget.jsx
import React, { useEffect, useRef, memo } from 'react';

function TradingViewWidget() {
    const container = useRef();

    useEffect(
        () => {
            console.log(container.current.id)
            const script = document.createElement("script");
            script.src = "https://s3.tradingview.com/external-embedding/embed-widget-advanced-chart.js";
            script.type = "text/javascript";
            script.async = true;
            script.innerHTML = JSON.stringify(
                {
                    autosize: false,
                    "symbol": "NASDAQ:AAPL",
                    "timezone": "Etc/UTC",
                    "theme": "dark",
                    "style": "1",
                    "locale": "en",
                    "range": "12M",
                    "allow_symbol_change": true,
                    "calendar": false,
                    "studies": [
                        "STD;RSI"
                    ],
                    "support_host": "https://www.tradingview.com"
                })
            script.id = "current_one";
            container.current.appendChild(script);
        },
        []
    );

    return (
        <div className="tradingview-widget-container" style={{flex: 1}}ref={container} >
            <div className="tradingview-widget-container__widget" >

            </div>
        </div>
    );
}

export default memo(TradingViewWidget);
