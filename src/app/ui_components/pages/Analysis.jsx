import { SoftPaper, theme } from "@/app/mui/theme";
import { Autocomplete, Button, Select, Stack, TextField, ThemeProvider, Tooltip } from '@mui/material';
import { Component } from "react";

import "@/app/css/Playground.css";
import "@/app/css/Analysis.css"
import "@/app/css/Homepage.css";
import { retrieve } from "@/app/funcs/cache";
import localforage from "localforage";
import Grid2 from "@mui/material/Unstable_Grid2/Grid2";
import MenuButton from "@/components/MenuButton";
import PredictionPopup from "../popups/PredictionPopup";
import { get_all_symbols, get_all_technical_data_keys, get_cached_ticker_technicals, get_ticker_technicals, percentage_change } from "@/app/funcs/stock_api";
import { Link } from "react-router-dom";
import { delay, unformat_number } from "@/app/funcs/tools";
import { get_state } from "@/app/funcs/states";
import { toast, ToastContainer } from "react-toastify";
import StockWidget from "@/components/widgets/StockWidget";

export default class Analysis extends Component {
    constructor(props) {
        super(props);

        this.state = {
            all_symbols: ["AAPL", "TSLA", "AMZN", "GOOGL", "MSFT"],
            searched_symbols: new Set(),
            search_value: "",
            filtered_tickers: [],
        }

        this.predictions = localforage.createInstance({
            "name": "predictions"
        });
        this.predict = this.predict.bind(this);
        this.fetch_all_data = this.fetch_all_data.bind(this);
        this.search_highest_price = this.search_highest_price.bind(this);
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
        const { all_symbols } = this.state;
        const random_num_hash = `${Math.random()}_${Date.now()}`;
        let state = get_state();
        state['getting_all_nums'] = random_num_hash; // used to ensure two instances of this function are not running simultaneously.
        let searched_symbols = new Set(await get_all_technical_data_keys());
        this.setState({ searched_symbols });

        const CHUNK_SIZE = 15; // Number of symbols to fetch at once
        let symbolChunks = [];
        // Split the symbols into chunks of size 3
        for (let i = 0; i < all_symbols.length; i += CHUNK_SIZE) {
            symbolChunks.push(all_symbols.slice(i, i + CHUNK_SIZE));
        }

        for (let chunk of symbolChunks) {
            // Skip symbols if skip_cached is true and already cached
            let promises = chunk.map(async symbol => {
                if (skip_cached && searched_symbols.has(symbol)) {
                    return Promise.resolve(); // Skip cached symbols
                }
                return get_ticker_technicals(symbol).then(() => {
                    searched_symbols.add(symbol);
                    this.setState({ searched_symbols, search_value: symbol });
                }).catch(() => {
                    toast.error(`${symbol} failed to fetch`);
                });
            });
            await Promise.all(promises); // Wait for all 3 requests to complete
            if (state['getting_all_nums'] !== random_num_hash) {
                console.log("Stopping current fetch for technicals");
                return;
            }
        }
    }

    async search_highest_price() {
        const all_keys = await get_all_technical_data_keys();
        const final_list = [];
        for (let key of all_keys) {
            const data = await get_cached_ticker_technicals(key);
            // console.log(data)
            if (!data || !data["data"] || !data["data"]["summaryData"]) continue;
            const summaryData = data["data"]["summaryData"];
            const bid_ask_spread = data["data"]["bidAsk"]
            const bid_spread = unformat_number(bid_ask_spread["Bid * Size"]["value"].split("*")[0])
            const ask_spread = unformat_number(bid_ask_spread["Ask * Size"]["value"].split("*")[0])
            const current_price = (bid_spread + ask_spread) / 2;
            const price_target = unformat_number(summaryData["OneYrTarget"]["value"])
            if (bid_spread === 0 || ask_spread === 0 || current_price === 0 || price_target === 0) continue;
            const percent_difference = percentage_change(price_target, current_price)

            const market_cap = unformat_number(summaryData["MarketCap"]["value"])

            if (market_cap < 10_000_000_000) continue;
            final_list.push({ symbol: key, market_cap, current_price, price_target, percent_difference })
        }
        final_list.sort((a, b) => b.percent_difference - a.percent_difference);
        this.setState({ filtered_tickers: final_list })
        console.log(final_list);
        console.log(final_list[0])
        console.log("finished")
    }

    render() {
        const { all_symbols, search_value, searched_symbols, filtered_tickers } = this.state;

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
                        <Button onClick={this.search_highest_price}>
                            Search highest price
                        </Button>
                    </Stack>
                </div>
                <PredictionPopup >
                    <h1>Click here to open popup</h1>
                </PredictionPopup>
                <div>
                    {`Searched symbols: ${(searched_symbols.size / all_symbols.length * 100).toFixed(2)}% (${searched_symbols.size}/${all_symbols.length})`}
                </div>
                <div>
                    {`Just got info on: ${search_value}`}
                </div>
                <div className={"widgets-container"}>
                    {filtered_tickers.slice(0, 100).map((data) => {
                        return <StockWidget symbol={data.symbol} size="small" key={data.symbol} />
                    })}
                </div>
            </div>
        </ThemeProvider>
    }
}