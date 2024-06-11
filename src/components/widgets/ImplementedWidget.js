import { fetch_widget_data } from "@/app/funcs/stock_api";
import React, { Component } from "react";
import BigStockWidget from "./BigStockWidget";

export default class ImplementedWidget extends Component {
    constructor(props) {
        super(props);
        this.state = {
            symbol: props.symbol,
            ticker_data: {}
        }
    }

    async componentDidMount() {
        fetch_widget_data(this.state.symbol).then((response) => {
            this.setState({ ticker_data: response });
        }).catch(error => {
            console.error("Failed to fetch data for " + this.state.symbol + ":", error);
        });
    }

    render() {
        return <div>
            <BigStockWidget {...this.state.ticker_data} />
        </div>;
    }
}