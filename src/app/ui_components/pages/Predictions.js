"use client"

import { SoftPaper, theme } from "@/app/mui/theme";
import { Autocomplete, Button, Select, Stack, TextField, ThemeProvider } from '@mui/material';
import { Component } from "react";

import "@/app/css/Playground.css";
import "../../css/Homepage.css";
import { retrieve } from "@/app/funcs/cache";
import localforage from "localforage";
import Grid2 from "@mui/material/Unstable_Grid2/Grid2";
import MenuButton from "@/components/MenuButton";
import Link from "next/link";
import PredictionPopup from "../popups/PredictionPopup";
import { get_all_symbols } from "@/app/funcs/stock_api";

export default class Predictions extends Component {
    constructor(props) {
        super(props);

        this.state = {
            all_symbols: [],
            search_value: "",
        }

        this.predictions = localforage.createInstance({
            "name": "predictions"
        });
        this.predict = this.predict.bind(this);
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

    render() {
        const { all_symbols, search_value} = this.state;

        return <ThemeProvider theme={theme}>
            <div className={"playground"}>
                <div className={"generic-header"} data-tauri-drag-region>
                    <SoftPaper elevation={8} component={Stack} marginBottom={0} square width={"100%"} style={{ borderTopRightRadius: 0, borderTopLeftRadius: 0 }}>
                        <Grid2 container marginLeft={5} marginTop={1} marginBottom={1} md={{ flexGrow: 1 }} columnGap={1}>
                            <MenuButton component={Link} href="/home" >
                                Home
                            </MenuButton>
                            {/* <TextField id='searchBar' label="Stock" variant='outlined' color='primary' /> */}
                        </Grid2>
                    </SoftPaper>
                </div>
                <div>
                    <h1>Predictions</h1>
                    <Stack spacing={2} direction={"row"}>
                        <Autocomplete
                            disablePortal
                            id="combo-box-demo"
                            options={all_symbols}
                            onChange={(event, search_value) => this.setState({ search_value })}
                            value={search_value}
                            sx={{ width: 300 }}
                            renderInput={(params) => <TextField {...params} label="Symbol" />}
                        />
                        <Button variant="standard" color="primary" onClick={this.predict}>
                            Add
                        </Button>
                    </Stack>
                </div>
                <PredictionPopup >
                    <h1>Click here to open popup</h1>
                </PredictionPopup>
            </div>
        </ThemeProvider>
    }
}