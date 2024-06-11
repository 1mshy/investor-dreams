// src/app/tickers/page.js
'use client';

import ImplementedWidget from '@/components/widgets/ImplementedWidget';
import { ThemeProvider } from '@mui/material';
import { Component } from 'react';
import { SoftPaper, theme } from '../../mui/theme';

class TickerPage extends Component {
    constructor(props) {
        super(props);
        this.state = {
            tickerSymbol: this.get_param('ticker_symbol'),
            isClient: false
        };
    }

    get_param(param) {
        const urlParams = new URLSearchParams(window.location.search);
        const tickerSymbol = urlParams.get(param);
        return tickerSymbol;
    }

    render() {
        const { tickerSymbol } = this.state;
        console.log(tickerSymbol)

        return (
            <ThemeProvider theme={theme}>
                <SoftPaper>
                    <h1>Tickers Page</h1>
                    {tickerSymbol ? <p>Ticker Symbol: {tickerSymbol}</p> : <p>Loading...</p>}
                    <ImplementedWidget symbol={tickerSymbol} />
                </SoftPaper>
            </ThemeProvider>
        );
    }
}

export default TickerPage;