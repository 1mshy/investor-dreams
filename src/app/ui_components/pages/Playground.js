"use client"
import { get_index_stocks, get_portfolio_weight, get_sp_500_data } from '@/app/funcs/scraper';
import {
    all_data,
    fetch_widget_data,
    get_all_data,
    get_sector
} from "@/app/funcs/stock_api";
import { Divider, Paper, Stack, TextField, ThemeProvider } from '@mui/material';
import Grid2 from '@mui/material/Unstable_Grid2/Grid2';
import { Component } from 'react';
import MenuButton from '../../../components/MenuButton';
import { DynamicStockWidget } from '../../../components/widgets/DynamicStockWidget';
import AccountMenu from '../accountMenu';
import Link from 'next/link';
import { SoftPaper, theme } from '@/app/mui/theme';
import SectorSelect from '../SectorSelect';
/**
 * css imports
 */
import "@/app/css/Widgets.css";
import "@/app/css/Playground.css";

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
            ticker_symbols: [],
        };
        this.set_sector = this.set_sector.bind(this);
        this.set_tickers = this.set_tickers.bind(this);
    }
    /**
     * 
     * @param {[string]} ticker_symbols 
     */
    async set_tickers(ticker_symbols) {
        this.setState({ ticker_symbols });
        const sp_500_data = await get_sp_500_data();
        let stock_data = this.state.stock_data;
        ticker_symbols.forEach(async (ticker_symbol) => {
            const { symbol, company, portfolio_percent, current_price, change, percent_change } = sp_500_data[ticker_symbol];
            console.log(sp_500_data[ticker_symbol])
            stock_data[ticker_symbol] = {
                symbol: symbol,
                name: company,
                price: current_price,
                change: change,
                percent_change: percent_change,
            };
        });

        if (typeof window !== 'undefined') {
            // Use Promise.all to fetch all data concurrently
            for (const ticker_symbol of ticker_symbols) {
                const data = await fetch_widget_data(ticker_symbol);
                let stock_data = this.state.stock_data;
                stock_data[ticker_symbol] = data;
                this.setState({ stock_data });
            }
        }
    }
    /**
     * sets the sector/industry filter
     * @param {string} sector 
     */
    set_sector(sector) {
        console.log(sector)
        get_all_data().then((data) => {
            let tickers_in_sector = [];
            for (const ticker in data) {
                if (data[ticker].sector === sector) {
                    tickers_in_sector.push(ticker);
                }
            };
            this.set_tickers(tickers_in_sector).then(_ => { });
        });
    };

    /**
     * This function is called when the component is mounted 
     * (aka when the component is added to the DOM) 
     * (aka when the component is finished rendering)
     */
    async componentDidMount() {
        // get the top companies
        const ticker_symbols = (await get_index_stocks())
        console.log(ticker_symbols)
        // map to an array of promises
        const weight_promises = ticker_symbols.slice(0, 10).map(async (ticker_symbol) => {
            const weight = await get_portfolio_weight(ticker_symbol);
            return { ticker_symbol, weight };
        });
        // collect and wait for all the promises in the array to resolve
        const weights = await Promise.all(weight_promises);
        // sort by highest weight
        const sorted_by_weight = weights.sort((a, b) => b.weight - a.weight).map(item => item.ticker_symbol);
        // update the state with the sorted ticker symbols
        await this.set_tickers(sorted_by_weight);
    }


    render() {
        const { stock_data, ticker_symbols } = this.state;
        return (
            <ThemeProvider theme={theme}>
                <div className={"playground"}>
                    <div className={"generic-header"}>
                        <SoftPaper elevation={8} component={Stack} marginBottom={0} square width={"100%"} style={{ borderTopRightRadius: 0, borderTopLeftRadius: 0 }}>
                            <Grid2 container marginLeft={5} marginTop={1} marginBottom={1} md={{ flexGrow: 1 }} columnGap={1}>
                                <MenuButton component={Link} href="/home" >
                                    Home
                                </MenuButton>
                                <MenuButton onClick={() => {
                                }}>
                                    Favourites
                                </MenuButton>
                                <SectorSelect set_sector={this.set_sector} />
                                <TextField id='searchBar' label="Stock" variant='filled' color='primary' />
                                <AccountMenu flexGrow>
                                </AccountMenu>
                            </Grid2>
                        </SoftPaper>

                    </div>
                    <div className={"playground-content"}>
                        <div className={"widgets-container"} style={{paddingTop: "3rem"}}>
                            {ticker_symbols.map(ticker_symbol => {
                                return <DynamicStockWidget {...stock_data[ticker_symbol]} size={"medium"} key={ticker_symbol} />
                            })}
                        </div>
                    </div>
                </div>
            </ThemeProvider>
        );
    }
}