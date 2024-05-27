"use client"
import React, { Component } from 'react';
import {
    change_from_data, get_list_prices, last_date_from_data, price_from_data, request_ticker_data
} from "@/app/funcs/stock_api";
import { invoke } from "@tauri-apps/api/tauri";

/**
 * css imports
 */
import "@/app/css/Widgets.css"
import { Divider, Paper, Stack, ThemeProvider } from '@mui/material';
import Grid2 from '@mui/material/Unstable_Grid2/Grid2';
import MenuButton from '../../../components/MenuButton';
import { StockWidget } from '../../../components/widgets/StockWidget';
import theme from '../../mui/theme';
import { get_portfolio_weight, ticker_to_name } from '@/app/funcs/scraper';

export default class Playground extends Component {
    constructor(props) {
        super(props);
        /**
         * @type {{stock_data: {}, ticker_symbols: string[]}}
         * @desc stock_data is a dictionary where the key is the ticker symbol and the value is the necessary stock data
         * @desc ticker_symbols is a list of ticker symbols to fetch data for
         * @desc The state of a component is an object that holds some information that may change over the lifetime of the component
         * @desc When the state of a component changes, the component will re-render to reflect the new state
         * @desc For example if I add a new item to {ticker_symbols} the component will re-render to reflect the new item
         */
        this.state = {
            stock_data: {},
            ticker_symbols: ["AAPL", "TSLA", "AMZN", "MSFT", "INTC", "NFLX", "COST", "NVDA", "AMD", "GOOGL", "META", "ALB"]
            // ticker_symbols: ["MMM", "AOS", "ABT", "ABBV", "ACN", "ADBE", "AMD", "AES", "AFL", "A", "APD", "ABNB", "AKAM", "ALB", "ARE", "ALGN", "ALLE", "LNT", "ALL", "GOOGL", "GOOG", "MO", "AMZN", "AMCR", "AEE", "AAL", "AEP", "AXP", "AIG", "AMT", "AWK", "AMP", "AME", "AMGN", "APH", "ADI", "ANSS", "AON", "APA", "AAPL", "AMAT", "APTV", "ACGL", "ADM", "ANET", "AJG", "AIZ", "T", "ATO", "ADSK", "ADP", "AZO", "AVB", "AVY", "AXON", "BKR", "BALL", "BAC", "BK", "BBWI", "BAX", "BDX", "BRK.B", "BBY", "BIO", "TECH", "BIIB", "BLK", "BX", "BA", "BKNG", "BWA", "BXP", "BSX", "BMY", "AVGO", "BR", "BRO", "BF.B", "BLDR", "BG", "CDNS", "CZR", "CPT", "CPB", "COF", "CAH", "KMX", "CCL", "CARR", "CTLT", "CAT", "CBOE", "CBRE", "CDW", "CE", "COR", "CNC", "CNP", "CF", "CHRW", "CRL", "SCHW", "CHTR", "CVX", "CMG", "CB", "CHD", "CI", "CINF", "CTAS", "CSCO", "C", "CFG", "CLX", "CME", "CMS", "KO", "CTSH", "CL", "CMCSA", "CMA", "CAG", "COP", "ED", "STZ", "CEG", "COO", "CPRT", "GLW", "CPAY", "CTVA", "CSGP", "COST", "CTRA", "CCI", "CSX", "CMI", "CVS", "DHR", "DRI", "DVA", "DAY", "DECK", "DE", "DAL", "DVN", "DXCM", "FANG", "DLR", "DFS", "DG", "DLTR", "D", "DPZ", "DOV", "DOW", "DHI", "DTE", "DUK", "DD", "EMN", "ETN", "EBAY", "ECL", "EIX", "EW", "EA", "ELV", "LLY", "EMR", "ENPH", "ETR", "EOG", "EPAM", "EQT", "EFX", "EQIX", "EQR", "ESS", "EL", "ETSY", "EG", "EVRG", "ES", "EXC", "EXPE", "EXPD", "EXR", "XOM", "FFIV", "FDS", "FICO", "FAST", "FRT", "FDX", "FIS", "FITB", "FSLR", "FE", "FI", "FMC", "F", "FTNT", "FTV", "FOXA", "FOX", "BEN", "FCX", "GRMN", "IT", "GE", "GEHC", "GEV", "GEN", "GNRC", "GD", "GIS", "GM", "GPC", "GILD", "GPN", "GL", "GS", "HAL", "HIG", "HAS", "HCA", "DOC", "HSIC", "HSY", "HES", "HPE", "HLT", "HOLX", "HD", "HON", "HRL", "HST", "HWM", "HPQ", "HUBB", "HUM", "HBAN", "HII", "IBM", "IEX", "IDXX", "ITW", "ILMN", "INCY", "IR", "PODD", "INTC", "ICE", "IFF", "IP", "IPG", "INTU", "ISRG", "IVZ", "INVH", "IQV", "IRM", "JBHT", "JBL", "JKHY", "J", "JNJ", "JCI", "JPM", "JNPR", "K", "KVUE", "KDP", "KEY", "KEYS", "KMB", "KIM", "KMI", "KLAC", "KHC", "KR", "LHX", "LH", "LRCX", "LW", "LVS", "LDOS", "LEN", "LIN", "LYV", "LKQ", "LMT", "L", "LOW", "LULU", "LYB", "MTB", "MRO", "MPC", "MKTX", "MAR", "MMC", "MLM", "MAS", "MA", "MTCH", "MKC", "MCD", "MCK", "MDT", "MRK", "META", "MET", "MTD", "MGM", "MCHP", "MU", "MSFT", "MAA", "MRNA", "MHK", "MOH", "TAP", "MDLZ", "MPWR", "MNST", "MCO", "MS", "MOS", "MSI", "MSCI", "NDAQ", "NTAP", "NFLX", "NEM", "NWSA", "NWS", "NEE", "NKE", "NI", "NDSN", "NSC", "NTRS", "NOC", "NCLH", "NRG", "NUE", "NVDA", "NVR", "NXPI", "ORLY", "OXY", "ODFL", "OMC", "ON", "OKE", "ORCL", "OTIS", "PCAR", "PKG", "PANW", "PARA", "PH", "PAYX", "PAYC", "PYPL", "PNR", "PEP", "PFE", "PCG", "PM", "PSX", "PNW", "PNC", "POOL", "PPG", "PPL", "PFG", "PG", "PGR", "PLD", "PRU", "PEG", "PTC", "PSA", "PHM", "QRVO", "PWR", "QCOM", "DGX", "RL", "RJF", "RTX", "O", "REG", "REGN", "RF", "RSG", "RMD", "RVTY", "RHI", "ROK", "ROL", "ROP", "ROST", "RCL", "SPGI", "CRM", "SBAC", "SLB", "STX", "SRE", "NOW", "SHW", "SPG", "SWKS", "SJM", "SNA", "SOLV", "SO", "LUV", "SWK", "SBUX", "STT", "STLD", "STE", "SYK", "SMCI", "SYF", "SNPS", "SYY", "TMUS", "TROW", "TTWO", "TPR", "TRGP", "TGT", "TEL", "TDY", "TFX", "TER", "TSLA", "TXN", "TXT", "TMO", "TJX", "TSCO", "TT", "TDG", "TRV", "TRMB", "TFC", "TYL", "TSN", "USB", "UBER", "UDR", "ULTA", "UNP", "UAL", "UPS", "URI", "UNH", "UHS", "VLO", "VTR", "VLTO", "VRSN", "VRSK", "VZ", "VRTX", "VTRS", "VICI", "V", "VST", "VMC", "WRB", "GWW", "WAB", "WBA", "WMT", "DIS", "WBD", "WM", "WAT", "WEC", "WFC", "WELL", "WST", "WDC", "WRK", "WY", "WMB", "WTW", "WYNN", "XEL", "XYL", "YUM", "ZBRA", "ZBH", "ZTS"],
        };
        // setup the data in the backend
    }

    async fetchStockData(ticker_symbol) {
        if (typeof window !== 'undefined') {
            try {
                const company_name = await ticker_to_name(ticker_symbol) // gets the name of the company
                const ticker_data = await request_ticker_data(ticker_symbol); // gets the stock data for the company, mostly historical prices
                // this should never happen, but if it does we should log it
                if (ticker_data === undefined) {
                    console.log("Error fetching data for " + ticker_symbol);
                    return;
                }

                const price = price_from_data(ticker_data);
                const change = change_from_data(ticker_data);
                const date = last_date_from_data(ticker_data);
                const historical_prices = get_list_prices(ticker_data);

                let data = {
                    symbol: ticker_symbol,
                    name: company_name,
                    price: price.toFixed(2),
                    change: change.toFixed(2),
                    date: date,
                    historical_prices: historical_prices
                };

                let stock_data = this.state.stock_data;
                stock_data[ticker_symbol] = data;
                this.setState({ stock_data });
            } catch (error) {
                console.log("Error fetching data for " + ticker_symbol + ": " + error.message);
            }
        }
    }

    /**
     * This function is called when the component is mounted 
     * (aka when the component is added to the DOM) 
     * (aka when the component is finished rendering)
     */
    async componentDidMount() {
        // map to an array of promises
        const weight_promises = this.state.ticker_symbols.map(async (ticker_symbol) => {
            const weight = await get_portfolio_weight(ticker_symbol);
            return { ticker_symbol, weight };
        });
        // collect and wait for all the promises in the array to resolve
        const weights = await Promise.all(weight_promises);
        // sort by highest weight
        const sorted_by_weight = weights.sort((a, b) => b.weight - a.weight).map(item => item.ticker_symbol);
        // update the state with the sorted ticker symbols
        this.setState({ ticker_symbols: sorted_by_weight });

        if (typeof window !== 'undefined') {
            const { ticker_symbols } = this.state;
            // Use Promise.all to fetch all data concurrently
            for (const ticker_symbol of ticker_symbols) {
                await this.fetchStockData(ticker_symbol);
            }
        }
    }


    render() {
        const { stock_data, ticker_symbols } = this.state;
        return (
            <ThemeProvider theme={theme}>
                <div className={"main-page"}>
                    <div className={"header"}>
                        <Paper elevation={8} component={Stack} marginBottom={5} square>
                            <Grid2 container marginLeft={5} marginTop={1} marginBottom={1} md={{ flexGrow: 1 }} columnGap={1}>
                                <MenuButton disabled>
                                    Home
                                </MenuButton>
                                <MenuButton onClick={() => {
                                }}>
                                    Favourites
                                </MenuButton>
                                <Divider />
                                <MenuButton onClick={() => {
                                }}>
                                    Technology
                                </MenuButton>
                                <MenuButton color="success" onClick={() => {
                                }}>
                                    Stonks
                                </MenuButton>
                            </Grid2>
                        </Paper>

                    </div>
                    <div className={"widgets-container"}>
                        {ticker_symbols.map(ticker_symbol => {
                            if (stock_data[ticker_symbol] === undefined) {
                                return <StockWidget symbol={ticker_symbol} key={ticker_symbol} />;
                            }
                            const { symbol, name, exchange, price, change, date, historical_prices } = stock_data[ticker_symbol];
                            return <StockWidget symbol={symbol} name={name} exchange={exchange} price={price} change={change}
                                date={date} historical_prices={historical_prices} key={symbol} />
                        })}
                    </div>
                </div>
            </ThemeProvider>
        );
    }
}