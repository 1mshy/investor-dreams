import { get_all_nasdaq_info, get_sp_500_data } from '@/app/funcs/scraper';
import {
    clean_ticker,
    fetch_widget_data,
    get_all_sectors,
    get_all_static_ticker_info,
    get_all_symbols,
    get_market_cap,
    nasdaq_sorted_by
} from "@/app/funcs/stock_api";
import { SoftPaper, theme } from '@/app/mui/theme';
import { Button, Stack, TextField, ThemeProvider } from '@mui/material';
import Grid2 from '@mui/material/Unstable_Grid2/Grid2';
import { Component } from 'react';
import MenuButton from '@/components/MenuButton';
import { DynamicStockWidget } from '@/components/widgets/DynamicStockWidget';
import SectorSelect from '@/app/ui_components/misc/SectorSelect';
/**
 * css imports
 */
import "@/app/css/Playground.css";
import "@/app/css/Widgets.css";
import { Link } from 'react-router-dom';
import EasySelection from '@/app/ui_components/misc/EasySelection';
import { retrieve, store } from '@/app/funcs/cache';
import { LoadingTextField } from '../mui/other';
import StockWidget from '@/components/widgets/StockWidget';
import { unformat_number } from '../funcs/tools';

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
            ticker_symbols: [],
            sort_method: this.get_sorting_method(), // Weight, Volitility, Bullish, Bearish
            add_sector_menu: false,
            create_sector_popup: false,
            ticker_search: "",
            show_loading_ticker_search: false,
        };
        this.set_sector = this.set_sector.bind(this);
        this.set_tickers = this.set_tickers.bind(this);
        this.set_top_20 = this.set_top_20.bind(this);

        this.sorting_content = {
            "MarketCap": () => { this.set_sorting("MarketCap") },
            "Volitility": () => { this.set_sorting("Volitility") },
            "Bullish": () => { this.set_sorting("Bullish") },
            "Bearish": () => { this.set_sorting("Bearish") },
        }
    }

    get_sorting_method() {
        const default_sort = "Volitility";
        const stored_sort = retrieve("sort_method");
        return stored_sort ? stored_sort : default_sort;
    }

    async searching_ticker(ticker_search) {
        const SHOWN_TICKER_AMOUNT = 5;
        this.setState({ ticker_search });
        if (ticker_search === "") return this.set_top_20();
        console.log(ticker_search)
        this.setState({ show_loading_ticker_search: true });
        const all_tickers = await get_all_symbols();
        console.log(all_tickers)
        const sorted_tickers = all_tickers.filter(ticker => !clean_ticker(ticker_search).includes(".") && ticker.toLocaleUpperCase().startsWith(clean_ticker(ticker_search).toLocaleUpperCase()))
        console.log(sorted_tickers)
        this.setState({ ticker_symbols: sorted_tickers.slice(0, SHOWN_TICKER_AMOUNT) });
        this.setState({ show_loading_ticker_search: false });
    }

    /**
     * 
     * @param {[string]} ticker_symbols 
     */
    async set_tickers(ticker_symbols, func) {
        if(!func) return this.setState({ ticker_symbols }, () => this.set_sorting(this.state.sort_method));
        this.setState({ ticker_symbols }, func);
    }

    /**
     * sets the sector/industry filter
     * @param {String} sector 
     */
    async set_sector(sector) {
        const sectors = await get_all_sectors();
        if (!sectors.includes(sector)) return;
        const all_data = await get_all_nasdaq_info();
        const top_500 = (await nasdaq_sorted_by("marketCap")).slice(0, 500);
        const tickers_in_sector = top_500.filter(ticker_symbol => all_data[ticker_symbol].sector === sector);
        this.set_tickers(tickers_in_sector);
    };

    async set_sorting(sort_method) {
        const { ticker_symbols } = this.state;
        const all_data = await get_all_nasdaq_info();
        this.setState({ sort_method })
        store("sort_method", sort_method);
        switch (sort_method) {
            case "MarketCap": {
                const weight_promises = ticker_symbols.map(async (ticker_symbol) => {
                    const weight = await get_market_cap(ticker_symbol);
                    return { ticker_symbol, weight };
                });
                // collect and wait for all the promises in the array to resolve
                const weights = await Promise.all(weight_promises);
                // sort by highest weight
                const sorted_by_weight = weights.sort((a, b) => Math.abs(b.weight) - Math.abs(a.weight)).map(item => item.ticker_symbol);
                // update the state with the sorted ticker symbols
                this.setState({ticker_symbols: sorted_by_weight});
                break;
            }
            case "Volitility": {
                const change_promises = ticker_symbols.map(async (ticker_symbol) => {
                    const change = unformat_number(all_data[ticker_symbol].pctchange);
                    return { ticker_symbol, change };
                });
                const changes = await Promise.all(change_promises);
                // sort by highest weight
                const sorted_by_weight = changes.sort((a, b) => Math.abs(b.change) - Math.abs(a.change)).map(item => item.ticker_symbol);
                // update the state with the sorted ticker symbols
                this.setState({ticker_symbols: sorted_by_weight});
                break;
            }
            case "Bullish": {
                const change_promises = ticker_symbols.map(async (ticker_symbol) => {
                    const change = unformat_number(all_data[ticker_symbol].pctchange);
                    return { ticker_symbol, change };
                });
                const changes = await Promise.all(change_promises);
                // sort by highest weight
                const sorted_by_weight = changes.sort((a, b) => b.change - a.change).map(item => item.ticker_symbol);
                // update the state with the sorted ticker symbols
                this.setState({ticker_symbols: sorted_by_weight});
                break;
            }
            case "Bearish": {
                const change_promises = ticker_symbols.map(async (ticker_symbol) => {
                    const change = unformat_number(all_data[ticker_symbol].pctchange);
                    return { ticker_symbol, change };
                });
                const changes = await Promise.all(change_promises);
                // sort by highest weight
                const sorted_by_weight = changes.sort((a, b) => a.change - b.change).map(item => item.ticker_symbol);
                // update the state with the sorted ticker symbols
                this.setState({ticker_symbols: sorted_by_weight});
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
        this.set_top_20();
    }

    async set_top_20() {
        // get the top companies
        const top_20_by_market_cap = (await nasdaq_sorted_by("marketCap")).slice(0, 20);
        this.set_tickers(top_20_by_market_cap);
    }


    render() {
        const { ticker_symbols, sort_method, ticker_search, show_loading_ticker_search } = this.state;
        console.log(ticker_symbols)
        return (
            <ThemeProvider theme={theme}>
                <div className={"playground"} >
                    <div className={"generic-header"} >
                        <SoftPaper data-tauri-drag-region elevation={8} component={Stack} marginBottom={0} square width={"100%"} style={{ borderTopRightRadius: 0, borderTopLeftRadius: 0 }}>
                            <Grid2 container marginLeft={5} marginTop={1} marginBottom={1} md={{ flexGrow: 1 }} columnGap={1} style={{ alignItems: "center" }} data-tauri-drag-region>
                                <SectorSelect set_sector={this.set_sector} />
                                <EasySelection label="Sort" content={this.sorting_content} default={sort_method} />
                                <Link to="/home" className={"homepage-navButton"} style={{ marginLeft: "auto", order: 2, height: "auto" }}>Home</Link>
                                <LoadingTextField id='searchBar' label="Stock" variant='outlined' color='primary' onChange={e => this.searching_ticker(e.target.value)} value={ticker_search} loading={show_loading_ticker_search} />
                                {ticker_search !== "" && <Button onClick={() => this.searching_ticker("")}>Clear</Button>}
                            </Grid2>
                        </SoftPaper>
                    </div>
                    <div className={"playground-content"} >
                        <div className={"widgets-container"} style={{ paddingTop: "3rem" }}>
                            {ticker_symbols.map(ticker_symbol => {
                                return <StockWidget size={"medium"} symbol={ticker_symbol} key={ticker_symbol} /> // {...stock_data[ticker_symbol]}
                            })}
                        </div>
                    </div>
                </div>
            </ThemeProvider>
        );
    }
}