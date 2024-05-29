import { Autocomplete } from "@mui/material"
import React, { Component } from "react"
import { SearchBarTop } from "./SearchBarTop"
import { get_index_stocks } from "@/app/funcs/scraper"

export default class StockSearch extends Component {
    constructor(props) {
        super(props)
        this.state = {
            search: "",
            options: []
        }
    }

    handleChange = (e) => {
        this.setState({
            search: e.target.value
        })
    }

    async componentDidMount() {
        const list_of_tickers = await get_index_stocks();
        this.setState({ options: list_of_tickers });
    }


    render() {
        const { options } = this.state;
        return (
            <Autocomplete
                disablePortal
                id="combo-box-demo"
                options={options}
                sx={{ width: 300 }}
                renderInput={(params) => <SearchBarTop {...params} />}
            />
        )
    }
}