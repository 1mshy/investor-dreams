import React, { useEffect } from 'react';

const TopStories = () => {
    useEffect(() => {
        const script = document.createElement('script');
        script.src = 'https://s3.tradingview.com/external-embedding/embed-widget-timeline.js';
        script.async = true;
        script.innerHTML = JSON.stringify({
            feedMode: "all_symbols",
            colorTheme: "dark",
            isTransparent: false,
        });
        document.querySelector('.top-stories-container').appendChild(script);
    }, []);

    return <div className="top-stories-container"></div>;
};

export default TopStories;
