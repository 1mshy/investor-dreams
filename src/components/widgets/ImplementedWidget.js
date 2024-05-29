import { fetch_widget_data } from "@/app/funcs/stock_api";
import React, { Component } from "react";
import BigStockWidget from "./BigStockWidget";

export default class ImplementedWidget extends Component {
    constructor(props) {
        super(props);
        this.state = {
            ticker_symbol: props.ticker_symbol,
            ticker_data: {}
        }
    }

    async componentDidMount() {
        fetch_widget_data(this.state.ticker_symbol).then((response) => {
            this.setState({ ticker_data: response });
        }).catch(error => {
            console.error("Failed to fetch data for " + this.state.ticker_symbol + ":", error);
        });
    }

    render() {
        return <div>
            <BigStockWidget {...this.state.ticker_data} />
        </div>;
    }
}