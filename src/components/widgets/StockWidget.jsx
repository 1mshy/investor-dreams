import { fetch_widget_data } from "@/app/funcs/stock_api";
import { Component } from "react";
import { DynamicStockWidget } from "./DynamicStockWidget";

/**
 * The StockWidget class is meant as a simple way to display stock data in a widget format
 * All the logic is handled by the DynamicStockWidget, and the StockWidget is just a wrapper
 * that fetches the data needed
 */
export default class StockWidget extends Component {
    constructor(props) {
        super(props);
        this.state = {
            symbol: props.symbol,
            size: props.size,
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
            <DynamicStockWidget {...this.state.ticker_data} size={this.state.size} />
        </>;
    }
}