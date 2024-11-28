import { cache_is_valid, STOCK_CACHE } from "@/app/funcs/cache";
import { get_all_nasdaq_info } from "@/app/funcs/scraper";
import { get_state } from "@/app/funcs/states";
import { clear_all_technical_data, get_all_symbols, get_all_technical_data, get_all_technical_data_keys, get_cached_ticker_technicals, get_ticker_technicals, NASDAQ_NEWS, NASDAQ_TECHNICALS } from "@/app/funcs/stock_api";
import { delay } from "@/app/funcs/tools";
import { BackGroundPaper, theme } from "@/app/mui/theme";
import TableDownloadPopup from "@/components/popups/TableDownloadPopup"; 
import StockWidget from "@/components/widgets/StockWidget";
import { Button, Checkbox, FormControl, InputLabel, MenuItem, Select, Stack, TextField, ThemeProvider, Tooltip, Typography } from '@mui/material';
import localforage from "localforage";
import React, { Component } from "react";
import { Link } from "react-router-dom";
import { toast } from "react-toastify";
import "@/app/css/Analysis.css";
import "@/app/css/Homepage.css";
import "@/app/css/Playground.css";
import CustomSectorNamePopup from "@/components/popups/CustomSectorNamePopup";
import { filter_tickers } from "@/app/funcs/analysis";
import { save_dynamic_sector } from "@/app/funcs/sectors";
import MarketCapSlider from "@/components/misc/MarketCapSlider";

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

        this.save_popup_ref = React.createRef(null);
    }

    openPopup() {
        this.save_popup_ref.current.open();
    };

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
    /**
     * 
     * @param {Boolean} skip_cached - should ignore the cached data if any
     * @returns {Promise<void>}
     */
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
        // console.log(JSON.stringify(all_symbols))
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
        const all_tickers = await get_all_symbols();
        const all_nasdaq_info = await get_all_nasdaq_info();
        const all_technical_data = await get_all_technical_data();
        const final_list = filter_tickers(searching_options, all_tickers, all_nasdaq_info, all_technical_data);
        this.setState({ filtered_tickers: final_list })
        console.log(final_list);
        console.log(final_list[0])
        console.log("finished")
    }
    render() {
        const { all_symbols, search_value, searched_symbols, filtered_tickers, downloadable_stores,
            show_searching_options, searching_options } = this.state;

        return <ThemeProvider theme={theme}>
            <div className={"analysis-whole"}>
                <div className={"homepage-header"} data-tauri-drag-region>
                    <div className={"homepage-nav"} data-tauri-drag-region>
                        <Link to="/home" className={"homepage-navButton"}>Home</Link>
                    </div>
                </div>
                <div className="">
                    <div>
                        <h1>Predictions</h1>
                        <div>
                            {`Searched symbols: ${(searched_symbols.size / all_symbols.length * 100).toFixed(2)}% (${searched_symbols.size}/${all_symbols.length})`}
                        </div>
                        <div>
                            {`Just got info on: ${search_value}`}
                        </div>
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
                        <Tooltip title={"Fetches all data + the ones that are out of date"}>
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
                        <div>
                            <Typography id="track-false-slider" gutterBottom>
                                Market Cap
                            </Typography>
                            <MarketCapSlider callback={(left, right) => {
                                this.setState({ searching_options: { ...searching_options, min_market_cap: left, max_market_cap: right } })
                            }} />
                        </div>
                        {/* <CurrencyTextField variant="standard" label="Min Market Cap" value={searching_options.min_market_cap} on_change={(value) => {
                            console.log(value)
                            if (isNaN(value) || value < 0) return;
                            this.setState({ searching_options: { ...searching_options, min_market_cap: Number(value) } })
                        }} />
                        <CurrencyTextField variant="standard" label="Max Market Cap" value={searching_options.max_market_cap} on_change={(value) => {
                            if (isNaN(value) || value < 0) return;
                            this.setState({ searching_options: { ...searching_options, max_market_cap: Number(value) } })
                        }} /> */}
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

                        <CustomSectorNamePopup ref={this.save_popup_ref} onSubmit={(user_input) => {
                            if (user_input && user_input !== "")
                                save_dynamic_sector(user_input, searching_options)
                        }} />

                        <div style={{ flex: 1 }}>
                            <TableDownloadPopup downloadable_stores={downloadable_stores}>
                                <Button style={{ float: "right" }}>
                                    Download Data
                                </Button>
                            </TableDownloadPopup>
                        </div>
                    </Stack>
                </BackGroundPaper>}
                <div className={"widgets-container"} data-tauri-drag-region style={{ height: "auto", flex: 7 }}>
                    {filtered_tickers.map((data) => {
                        return <StockWidget symbol={data.symbol} size="small" key={data.symbol} />
                    })}
                </div>
            </div>
        </ThemeProvider>
    }
}