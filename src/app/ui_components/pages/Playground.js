"use client"
import { get_index_stocks, get_lazy_percent_change, get_portfolio_weight, get_sp_500_data } from '@/app/funcs/scraper';
import {
    fetch_widget_data,
    get_index_info,
    get_all_sectors
} from "@/app/funcs/stock_api";
import { SoftPaper, theme } from '@/app/mui/theme';
import { Stack, TextField, ThemeProvider } from '@mui/material';
import Grid2 from '@mui/material/Unstable_Grid2/Grid2';
import Link from 'next/link';
import { Component } from 'react';
import MenuButton from '../../../components/MenuButton';
import { DynamicStockWidget } from '../../../components/widgets/DynamicStockWidget';
import SectorSelect from '../misc/SectorSelect';
/**
 * css imports
 */
import "@/app/css/Playground.css";
import "@/app/css/Widgets.css";
import EasySelection from '../misc/EasySelection';

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
            sort_method: "Volitility", // Weight, Volitility, Bullish, Bearish
            add_sector_menu: false,
            create_sector_popup: false,
        };
        this.set_sector = this.set_sector.bind(this);
        this.set_tickers = this.set_tickers.bind(this);

        this.sorting_content = {
            "Weight": () => { this.set_sorting("Weight") },
            "Volitility": () => { this.set_sorting("Volitility") },
            "Bullish": () => { this.set_sorting("Bullish") },
            "Bearish": () => { this.set_sorting("Bearish") },
        }
    }

    /**
     * 
     * @param {[string]} ticker_symbols 
     */
    async set_tickers(ticker_symbols, func) {
        this.setState({ ticker_symbols });
        get_sp_500_data().then(async sp_500_data => {
            let stock_data = this.state.stock_data;
            ticker_symbols.forEach(async (ticker_symbol) => {
                if (!sp_500_data[ticker_symbol]) return; // checks if stock exists in the large dataset
                const { symbol, company, current_price, change, percent_change } = sp_500_data[ticker_symbol];
                stock_data[ticker_symbol] = {
                    symbol: symbol,
                    name: company,
                    price: current_price,
                    change: change,
                    percent_change: percent_change,
                };
            });

            if (typeof window !== 'undefined') {
                let stock_data = this.state.stock_data;
                Promise.all(ticker_symbols.map(async (ticker_symbol) => {
                    const data = await fetch_widget_data(ticker_symbol);
                    stock_data[ticker_symbol] = data;
                    this.setState({ stock_data });
                })).then(func)

                // for (const ticker_symbol of ticker_symbols) {
                //     const data = await fetch_widget_data(ticker_symbol);
                //     stock_data[ticker_symbol] = data;
                //     this.setState({ stock_data });
                // }
            }
        });
    }

    /**
     * sets the sector/industry filter
     * @param {string} sector 
     */
    set_sector(sector) {
        get_all_sectors().then(sectors => {
            if (sectors.includes(sector)) {
                get_index_info().then((data) => {
                    get_sp_500_data().then(sp_500_data => {
                        let tickers_in_sector = [];
                        for (const ticker in sp_500_data) {
                            if (data[ticker] && data[ticker].sector === sector) {
                                tickers_in_sector.push(ticker);
                            }
                        };

                        this.set_tickers(tickers_in_sector, () => this.set_sorting(this.state.sort_method)).then(_ => { });
                    });
                });
            }
        })
    };

    async set_sorting(sort_method) {
        const { ticker_symbols, stock_data } = this.state;
        console.log(sort_method)
        this.setState({ sort_method })
        switch (sort_method) {
            case "Weight": {
                const weight_promises = ticker_symbols.map(async (ticker_symbol) => {
                    const weight = await get_portfolio_weight(ticker_symbol);
                    return { ticker_symbol, weight };
                });
                // collect and wait for all the promises in the array to resolve
                const weights = await Promise.all(weight_promises);
                // sort by highest weight
                const sorted_by_weight = weights.sort((a, b) => Math.abs(b.weight) - Math.abs(a.weight)).map(item => item.ticker_symbol);
                // update the state with the sorted ticker symbols
                await this.set_tickers(sorted_by_weight);
                break;
            }
            case "Volitility": {
                const change_promises = ticker_symbols.map(async (ticker_symbol) => {
                    const change = stock_data[ticker_symbol].percent_change;
                    return { ticker_symbol, change };
                });
                const changes = await Promise.all(change_promises);
                // sort by highest weight
                const sorted_by_weight = changes.sort((a, b) => Math.abs(b.change) - Math.abs(a.change)).map(item => item.ticker_symbol);
                // update the state with the sorted ticker symbols
                await this.set_tickers(sorted_by_weight);
                break;
            }
            case "Bullish": {
                const change_promises = ticker_symbols.map(async (ticker_symbol) => {
                    const change = stock_data[ticker_symbol].percent_change;
                    return { ticker_symbol, change };
                });
                const changes = await Promise.all(change_promises);
                // sort by highest weight
                const sorted_by_weight = changes.sort((a, b) => b.change - a.change).map(item => item.ticker_symbol);
                // update the state with the sorted ticker symbols
                await this.set_tickers(sorted_by_weight);
                break;
            }
            case "Bearish": {
                const change_promises = ticker_symbols.map(async (ticker_symbol) => {
                    const change = stock_data[ticker_symbol].percent_change;
                    return { ticker_symbol, change };
                });
                const changes = await Promise.all(change_promises);
                // sort by highest weight
                const sorted_by_weight = changes.sort((a, b) => a.change - b.change).map(item => item.ticker_symbol);
                // update the state with the sorted ticker symbols
                await this.set_tickers(sorted_by_weight);
                break;
            }

        }
    }

    /**
     * This function is called when the component is mounted 
     * (aka when the component is added to the DOM) 
     * (aka when the component is finished rendering)
     */
    async componentDidMount() {
        // get the top companies
        const ticker_symbols = await get_index_stocks();
        this.set_tickers(ticker_symbols.slice(0, 12), () => this.set_sorting(this.state.sort_method));
    }

    render() {
        const { stock_data, ticker_symbols, sort_method } = this.state;
        return (
            <ThemeProvider theme={theme}>
                <div className={"playground"}>
                    <div className={"generic-header"}>
                        <SoftPaper elevation={8} component={Stack} marginBottom={0} square width={"100%"} style={{ borderTopRightRadius: 0, borderTopLeftRadius: 0 }}>
                            <Grid2 container marginLeft={5} marginTop={1} marginBottom={1} md={{ flexGrow: 1 }} columnGap={1}>
                                <MenuButton component={Link} href="/home" >
                                    Home
                                </MenuButton>
                                <SectorSelect set_sector={this.set_sector} />
                                <EasySelection label="Sort" content={this.sorting_content} default={sort_method} />
                                {/* <TextField id='searchBar' label="Stock" variant='outlined' color='primary' /> */}
                            </Grid2>
                        </SoftPaper>
                    </div>
                    <div className={"playground-content"}>
                        <div className={"widgets-container"} style={{ paddingTop: "3rem" }}>
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