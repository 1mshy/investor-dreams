import { SoftPaper, theme } from "@/app/mui/theme";
import { Autocomplete, Button, Select, Stack, TextField, ThemeProvider, Tooltip } from '@mui/material';
import { Component } from "react";

import "@/app/css/Playground.css";
import "../../css/Homepage.css";
import { retrieve } from "@/app/funcs/cache";
import localforage from "localforage";
import Grid2 from "@mui/material/Unstable_Grid2/Grid2";
import MenuButton from "@/components/MenuButton";
import PredictionPopup from "../popups/PredictionPopup";
import { get_all_symbols, get_all_technical_data_keys, get_ticker_technicals } from "@/app/funcs/stock_api";
import { Link } from "react-router-dom";
import { delay } from "@/app/funcs/tools";
import { get_state } from "@/app/funcs/states";
import { toast, ToastContainer } from "react-toastify";

export default class Predictions extends Component {
    constructor(props) {
        super(props);

        this.state = {
            all_symbols: ["AAPL", "TSLA", "AMZN", "GOOGL", "MSFT"],
            searched_symbols: new Set(),
            search_value: "",
        }

        this.predictions = localforage.createInstance({
            "name": "predictions"
        });
        this.predict = this.predict.bind(this);
        this.fetch_all_data = this.fetch_all_data.bind(this);
    }

    async componentDidMount() {
        const keys = await this.predictions.keys();
        console.log(keys);

        const all_symbols = await get_all_symbols();
        this.setState({ all_symbols });
    }

    predict(symbol) {
        this.predictions.setItem("AAPL", 180)
    }

    async fetch_all_data(skip_cached = true) {
        const { all_symbols } = this.state;
        const random_num_hash = `${Math.random()}_${Date.now()}`;
        let state = get_state();
        state['getting_all_nums'] = random_num_hash; // used to make sure two instances of this function are not running at the same time.
        let counter = 0;
        let searched_symbols = new Set(await get_all_technical_data_keys())
        this.setState({ searched_symbols });
        for (let symbol of all_symbols) {
            // skip if skip_cached is true and already cached
            if (skip_cached && searched_symbols.has(symbol)) {
                continue;
            }
            try {
                const data = await get_ticker_technicals(symbol);
                searched_symbols.add(symbol);
                this.setState({ searched_symbols, search_value: symbol });
            }
            catch (_) {
                toast.error(`${symbol} failed to fetch`)
            }
            if (state['getting_all_nums'] !== random_num_hash) {
                console.log("Stopping current fetch for technicals");
                return;
            };
        }
    }

    async search_highest_price() {
        // const all_keys = await get_all_technical_data_keys();

    }

    render() {
        const { all_symbols, search_value, searched_symbols } = this.state;

        return <ThemeProvider theme={theme}>
            <div className={"playground"}>
                <div className={"generic-header"} data-tauri-drag-region>
                    <SoftPaper data-tauri-drag-region elevation={8} component={Stack} marginBottom={0} square width={"100%"} style={{ borderTopRightRadius: 0, borderTopLeftRadius: 0 }}>
                        <Grid2 data-tauri-drag-region container marginLeft={5} marginTop={1} marginBottom={1} md={{ flexGrow: 1 }} columnGap={1}>
                            <MenuButton component={Link} to="/home" >
                                Home
                            </MenuButton>
                            {/* <TextField id='searchBar' label="Stock" variant='outlined' color='primary' /> */}
                        </Grid2>
                    </SoftPaper>
                </div>
                <div>
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
                            <Button variant="standard" color="primary" onClick={() => {
                                this.fetch_all_data(true)
                            }}>
                                Fetch Missing
                            </Button>
                        </Tooltip>
                        <Tooltip title={"Fetches all data that is out of date (NOT RECOMMENDED as this can take over an hour)"}>
                            <Button variant="standard" color="primary" onClick={() => {
                                this.fetch_all_data(false)
                            }}>
                                Fetch Up to date
                            </Button>
                        </Tooltip>
                    </Stack>
                </div>
                <PredictionPopup >
                    <h1>Click here to open popup</h1>
                </PredictionPopup>
                <div>
                    {`Searched symbols: ${(searched_symbols.size / all_symbols.length * 100).toFixed(2)}%`}
                </div>
                <div>
                    {`Just got info on: ${search_value}`}
                </div>
            </div>
        </ThemeProvider>
    }
}