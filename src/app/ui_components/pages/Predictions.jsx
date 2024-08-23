import { SoftPaper, theme } from "@/app/mui/theme";
import { Autocomplete, Button, Select, Stack, TextField, ThemeProvider } from '@mui/material';
import { Component } from "react";

import "@/app/css/Playground.css";
import "../../css/Homepage.css";
import { retrieve } from "@/app/funcs/cache";
import localforage from "localforage";
import Grid2 from "@mui/material/Unstable_Grid2/Grid2";
import MenuButton from "@/components/MenuButton";
import PredictionPopup from "../popups/PredictionPopup";
import { get_all_symbols, get_ticker_technicals } from "@/app/funcs/stock_api";
import { Link } from "react-router-dom";
import { delay } from "@/app/funcs/tools";

export default class Predictions extends Component {
    constructor(props) {
        super(props);

        this.state = {
            all_symbols: ["AAPL", "TSLA", "AMZN", "GOOGL", "MSFT"],
            searched_symbols: [],
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

    async fetch_all_data() {
        const { all_symbols } = this.state;
        let counter = 0;
        for (let symbol of all_symbols) {
            const data = await get_ticker_technicals(symbol);
            const searched_symbols = this.state.searched_symbols;
            searched_symbols.push(symbol);
            this.setState({ searched_symbols });
            console.log(counter++)
            // await delay(1000);
        }
    }

    render() {
        const { all_symbols, search_value, searched_symbols} = this.state;

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
                        <Button variant="standard" color="primary" onClick={this.fetch_all_data}>
                            Add
                        </Button>
                    </Stack>
                </div>
                <PredictionPopup >
                    <h1>Click here to open popup</h1>
                </PredictionPopup>
                <div>
                        {searched_symbols.map((symbol) => {
                            return <div key={symbol}>
                                <h1>{symbol}</h1>
                            </div>

                        })}
                </div>
            </div>
        </ThemeProvider>
    }
}