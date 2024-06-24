import { fetch_widget_data } from "@/app/funcs/stock_api";
import React, { Component } from "react";
import BigStockWidget from "./BigStockWidget";
import MiniStockWidget from "./MiniStockWidget";
import MediumStockWidget from "./MediumStockWidget";
import SmallStockWidget from "./SmallStockWidget";

export default class StockWidget extends Component {
    constructor(props) {
        super(props);
        this.state = {
            symbol: props.symbol,
            ticker_data: {},

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

        return <>
            {this.props.size === "mini" && <MiniStockWidget
                {...this.state.ticker_data}
            />}
            {this.props.size === "small" && <SmallStockWidget
                {...this.state.ticker_data}
            />}
            {this.props.size === "medium" && <MediumStockWidget
                {...this.state.ticker_data} />}
            {this.props.size === "big" &&
                <BigStockWidget {...this.state.ticker_data} />}
        </>;
    }
}