import React, { useEffect } from 'react';

const CompanyProfile = () => {
    useEffect(() => {
        const script = document.createElement('script');
        script.src = 'https://s3.tradingview.com/external-embedding/embed-widget-company-profile.js';
        script.async = true;
        script.innerHTML = JSON.stringify({
            symbol: "NASDAQ:AAPL",
            isTransparent: false,
            colorTheme: "dark",

        });
        document.querySelector('.company-profile-container').appendChild(script);
    }, []);

    return <div className="company-profile-container"></div>;
};

export default CompanyProfile;
