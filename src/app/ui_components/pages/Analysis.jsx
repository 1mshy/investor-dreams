import { BackGroundPaper, SoftPaper, theme } from "@/app/mui/theme";
import { Button, Checkbox, FormControl, InputLabel, MenuItem, Select, Stack, TextField, ThemeProvider, Tooltip } from '@mui/material';
import { Component } from "react";
import { cache_is_valid, STOCK_CACHE } from "@/app/funcs/cache";
import { get_all_nasdaq_info } from "@/app/funcs/scraper";
import { get_state } from "@/app/funcs/states";
import { clear_all_technical_data, get_all_symbols, get_all_technical_data_keys, get_cached_ticker_technicals, get_ticker_technicals, NASDAQ_NEWS, NASDAQ_TECHNICALS, percentage_change } from "@/app/funcs/stock_api";
import { delay, unformat_number } from "@/app/funcs/tools";
import { CurrencyTextField } from "@/app/mui/other";
import MenuButton from "@/components/MenuButton";
import StockWidget from "@/components/widgets/StockWidget";
import Grid2 from "@mui/material/Unstable_Grid2/Grid2";
import localforage from "localforage";
import { Link } from "react-router-dom";
import { toast } from "react-toastify";
import TableDownloadPopup from "../popups/TableDownloadPopup";

import "@/app/css/Analysis.css";
import "@/app/css/Homepage.css";
import "@/app/css/Playground.css";

export default class Analysis extends Component {
    constructor(props) {
        super(props);

        this.state = {
            all_symbols: ["AAPL", "TSLA", "AMZN", "GOOGL", "MSFT"],
            downloadable_stores: [STOCK_CACHE, NASDAQ_TECHNICALS, NASDAQ_NEWS,],
            searched_symbols: new Set(),
            search_value: "",
            filtered_tickers: [],
            show_searching_options: false,
            searching_options: {
                min_market_cap: 1_000_000_000,
                max_market_cap: 1_000_000_000_000_000,
                tickers_shown: 10,
                sort_by: "market_cap",
                reverse: false,
            }
        }

        this.predictions = localforage.createInstance({
            "name": "predictions"
        });
        this.predict = this.predict.bind(this);
        this.fetch_all_data = this.fetch_all_data.bind(this);
        this.sort_all_tickers = this.sort_all_tickers.bind(this);
        this.toggle_searching_options = this.toggle_searching_options.bind(this);
    }

    async componentDidMount() {
        const all_symbols = await get_all_symbols();
        this.setState({ all_symbols }, async () => {
            if (get_state()['getting_all_nums']) {
                this.fetch_all_data(true);
            }
        });
    }

    predict(symbol) {
        this.predictions.setItem("AAPL", 180)
    }

    async fetch_all_data(skip_cached = true) {
        const all_symbols = await get_all_symbols();
        const random_num_hash = `${Math.random()}_${Date.now()}`;
        let state = get_state();
        state['getting_all_nums'] = random_num_hash; // used to ensure two instances of this function are not running simultaneously.
        let searched_symbols = skip_cached ? new Set(await get_all_technical_data_keys()) : new Set();
        this.setState({ searched_symbols });

        const MAX_CHUNK_SIZE = 20; // Number of symbols to fetch at once
        let symbol_chunks = []; // array of the chunks
        let i = 0; // index in all_symbols
        let chunk = []; // current chunk
        const eval_chunks = () => {
            if (chunk.length >= MAX_CHUNK_SIZE || (i >= all_symbols.length - 1 && chunk.length > 0)) {
                symbol_chunks.push(chunk);
                chunk = [];
            }
        }
        while (i < all_symbols.length) {
            eval_chunks();
            const symbol = all_symbols[i];
            if ((skip_cached && searched_symbols.has(symbol)) || (!skip_cached && await cache_is_valid(symbol, await get_cached_ticker_technicals(symbol)))) {
                searched_symbols.add(symbol);
                i++;
                continue;
            }
            chunk.push(all_symbols[i++]);
        }
        eval_chunks();
        console.log(JSON.stringify(all_symbols))
        // console.log(symbol_chunks)

        for (let chunk of symbol_chunks) {
            const start = Date.now();
            // Skip symbols if skip_cached is true and already cached
            let promises = chunk.map(async symbol => {
                // if (skip_cached && searched_symbols.has(symbol)) {
                //     return Promise.resolve(); // Skip cached symbols
                // }
                return get_ticker_technicals(symbol).then(() => {
                    searched_symbols.add(symbol);
                    this.setState({ searched_symbols, search_value: symbol });
                }).catch(() => {
                    toast.error(`${symbol} failed to fetch`);
                });
            });
            promises.push(delay(900)); // Delay between each chunk
            await Promise.all(promises); // Wait for all 3 requests to complete
            const end = Date.now();
            // console.log(`Chunk took ${(end - start) / 1000}s`);
            if (state['getting_all_nums'] !== random_num_hash) {
                console.log("Stopping current fetch for technicals");
                return;
            }
        }
        this.setState({ searched_symbols });
    }

    toggle_searching_options() {
        this.setState({ show_searching_options: !this.state.show_searching_options });
    }

    async sort_all_tickers() {
        const { searching_options } = this.state;
        const all_keys = await get_all_technical_data_keys();
        const all_nasdaq_info = await get_all_nasdaq_info();
        const final_list = [];
        for (let key of all_keys) {
            const data = await get_cached_ticker_technicals(key);
            if (!data || !data["data"] || !data["data"]["summaryData"] || !all_nasdaq_info[key]) continue;
            const summaryData = data["data"]["summaryData"];
            const current_price = unformat_number(all_nasdaq_info[key]["lastsale"])
            const price_target = unformat_number(summaryData["OneYrTarget"]["value"])
            const pe_ratio = unformat_number(summaryData["PERatio"]["value"])
            const forward_pe_ratio = unformat_number(summaryData["ForwardPE1Yr"]["value"])
            const divided_yield = unformat_number(summaryData["Yield"]["value"])
            const net_change = unformat_number(all_nasdaq_info[key]["netchange"])
            const percent_change = unformat_number(all_nasdaq_info[key]["pctchange"])
            if (current_price === 0 || price_target === 0) continue;
            const target_percent_difference = percentage_change(price_target, current_price)
            // console.log(pe_ratio)
            const market_cap = unformat_number(summaryData["MarketCap"]["value"])
            if (market_cap < searching_options.min_market_cap || market_cap > searching_options.max_market_cap) continue;
            const final_data = {
                symbol: key, market_cap, current_price, price_target, percent_change, net_change,
                target_percent_difference, pe_ratio, forward_pe_ratio, divided_yield
            }
            final_list.push(final_data);
            // this.setState({ filtered_tickers: final_list })
        }
        final_list.sort((a, b) => b[searching_options.sort_by] - a[searching_options.sort_by]);
        if (searching_options.reverse) final_list.reverse();
        this.setState({ filtered_tickers: final_list })
        console.log(final_list);
        console.log(final_list[0])
        console.log("finished")
    }
    render() {
        const { all_symbols, search_value, searched_symbols, filtered_tickers, downloadable_stores,
            show_searching_options, searching_options } = this.state;

        return <ThemeProvider theme={theme}>
            <div className={"playground"}>
                <div className={"generic-header"} data-tauri-drag-region>
                    <SoftPaper data-tauri-drag-region elevation={8} component={Stack} marginBottom={0} width={"100%"} style={{ borderTopRightRadius: 0, borderTopLeftRadius: 0 }}>
                        <Grid2 data-tauri-drag-region container marginLeft={5} marginTop={1} marginBottom={1} md={{ flexGrow: 1 }} columnGap={1}>
                            <MenuButton component={Link} to="/home" >
                                Home
                            </MenuButton>
                            {/* <TextField id='searchBar' label="Stock" variant='outlined' color='primary' /> */}
                        </Grid2>
                    </SoftPaper>
                </div>
                <div className="">
                    <h1>Predictions</h1>
                    <div>
                        {`Searched symbols: ${(searched_symbols.size / all_symbols.length * 100).toFixed(2)}% (${searched_symbols.size}/${all_symbols.length})`}
                    </div>
                    <div>
                        {`Just got info on: ${search_value}`}
                    </div>
                    <Stack spacing={2} direction={"row"}>
                        {/* <Autocomplete
                            disablePortal
                            id="combo-box-demo"
                            options={all_symbols}
                            onChange={(event, search_value) => this.setState({ search_value })}
                            value={search_value}
                            sx={{ width: 300 }}
                            renderInput={(params) => <TextField {...params} label="Symbol" />}
                        /> */}
                        <Tooltip title={"Fetches all the missing data"}>
                            <Button onClick={() => {
                                this.fetch_all_data(true)
                            }}>
                                Fetch Missing
                            </Button>
                        </Tooltip>
                        <Tooltip title={"Fetches all data that is out of date (NOT RECOMMENDED as this can take over an hour)"}>
                            <Button onClick={() => {
                                this.fetch_all_data(false)
                            }}>
                                Fetch Up to date
                            </Button>
                        </Tooltip>
                        <Button onClick={this.toggle_searching_options}>
                            Analysis Options
                        </Button>
                        <div style={{ flex: 1, paddingRight: "1rem" }}>
                            <Button onClick={clear_all_technical_data} style={{ float: "right" }}>
                                Clear Cache
                            </Button>
                        </div>

                    </Stack>
                </div>
                {/* <PredictionPopup >
                    <h1>Click here to open popup</h1>
                </PredictionPopup> */}

                {show_searching_options && <BackGroundPaper style={{ padding: "1rem", minHeight: "4rem", flex: 1 }}>
                    <Stack spacing={2} direction={"row"}>
                        <Button onClick={() => {
                            this.sort_all_tickers()
                        }}>
                            Load
                        </Button>
                        <CurrencyTextField variant="standard" label="Min Market Cap" value={searching_options.min_market_cap} on_change={(value) => {
                            console.log(value)
                            if (isNaN(value) || value < 0) return;
                            this.setState({ searching_options: { ...searching_options, min_market_cap: Number(value) } })
                        }} />
                        <CurrencyTextField variant="standard" label="Max Market Cap" value={searching_options.max_market_cap} on_change={(value) => {
                            if (isNaN(value) || value < 0) return;
                            this.setState({ searching_options: { ...searching_options, max_market_cap: Number(value) } })
                        }} />
                        <TextField variant="standard" label="Tickers Shown" value={searching_options.tickers_shown} onChange={(e) => {
                            const value = e.target.value;
                            if (isNaN(value) || value < 0) return;
                            this.setState({ searching_options: { ...searching_options, tickers_shown: Number(value) } })
                        }} />

                        <FormControl>
                            <InputLabel id="demo-simple-select-label">Sorting</InputLabel>
                            <Select
                                labelId="demo-simple-select-label"
                                id="demo-simple-select"
                                value={searching_options.sort_by}
                                label="Sort by"
                                onChange={(e) => {
                                    this.setState({ searching_options: { ...searching_options, sort_by: e.target.value } })
                                }}
                            >
                                <MenuItem value={"market_cap"}>Market Cap</MenuItem>
                                <MenuItem value={"target_percent_difference"}>Price target %</MenuItem>
                                <MenuItem value={"pe_ratio"}>PE ratio</MenuItem>
                                <MenuItem value={"forward_pe_ratio"}>FPE ratio</MenuItem>
                                <MenuItem value={"divided_yield"}>Divided Yield</MenuItem>

                                <MenuItem value={"percent_change"}>Daily percent change</MenuItem>
                                <MenuItem value={"net_change"}>Daily net change</MenuItem>
                            </Select>
                        </FormControl>

                        <FormControl>
                            Reverse:
                            <Checkbox label="Reverse" value={searching_options.reverse} onChange={() => {
                                this.setState({ searching_options: { ...searching_options, reverse: !searching_options.reverse } })
                            }} />
                        </FormControl>

                        <div style={{ flex: 1 }}>
                            <TableDownloadPopup downloadable_stores={downloadable_stores}>
                                <Button style={{ float: "right" }}>
                                    Download Data
                                </Button>
                            </TableDownloadPopup>
                        </div>
                    </Stack>
                </BackGroundPaper>}
                <div className={"widgets-container"} style={{ height: "auto", flex: 7 }}>
                    {filtered_tickers.slice(0, searching_options.tickers_shown).map((data) => {
                        return <StockWidget symbol={data.symbol} size="small" key={data.symbol} />
                    })}
                </div>
            </div>
        </ThemeProvider>
    }
}