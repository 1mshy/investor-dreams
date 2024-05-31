import { fetch_widget_data } from "@/app/funcs/stock_api";
import React, { Component } from "react";
import BigStockWidget from "./BigStockWidget";
import MiniStockWidget from "./MiniStockWidget";
import MediumStockWidget from "./MediumStockWidget";

export default class ImplementedDynamicStockWidget extends Component {
    constructor(props) {
        super(props);
        this.state = {
            ticker_symbol: props.ticker_symbol,
            ticker_data: {},

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
            {this.props.size === "mini" && <MiniStockWidget
                {...this.state.ticker_data}
            />}
            {this.props.size === "medium" && <MediumStockWidget
                {...this.state.ticker_data} />}
            {this.props.size === "big" &&
                <BigStockWidget {...this.state.ticker_data} />}
        </div>;
    }
}