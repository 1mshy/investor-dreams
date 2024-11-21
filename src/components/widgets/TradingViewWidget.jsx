import React, { useEffect, useRef, memo } from 'react';

function TradingViewWidget(props) {
    const container = useRef();

    useEffect(() => {
        // Reset the container
        container.current.innerHTML = "";

        const script = document.createElement("script");
        script.src = "https://s3.tradingview.com/external-embedding/embed-widget-advanced-chart.js";
        script.type = "text/javascript";
        script.async = true;
        script.innerHTML = JSON.stringify({
            autosize: true,
            hide_top_toolbar: false,
            hide_side_toolbar: false,
            withdateranges: true,
            save_image: false,
            details: true,
            allow_symbol_change: false,
            calendar: false,
            interval: "D",
            symbol: props.symbol ? props.symbol : "AAPL",
            timezone: "Etc/UTC",
            theme: "dark",
            style: "1",
            locale: "en",
            range: "12M",
            studies: ["STD;RSI"],
            support_host: "https://www.tradingview.com",
        });

        container.current.appendChild(script);
    }, []);

    return (
        <div
            className="tradingview-widget-container"
            ref={container}
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

export default memo(TradingViewWidget);
