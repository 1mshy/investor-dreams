"use client"
import React, {Component} from 'react';
import {
    change_from_data, get_list_prices, last_date_from_data, price_from_data, request_ticker_data, ticker_price
} from "@/app/funcs/stock_api";
import StockWidget from "@/app/ui_components/StockWidget";
import {invoke} from "@tauri-apps/api/tauri";

/**
 * css imports
 */
import "@/app/css/Widgets.css"

export default class Home extends Component {
    constructor(props) {
        super(props);
        this.state = {
            price: null, stock_data: []
        };
        // setup the data in the backend
        invoke("read_csv").then(_ => {
        })
    }

    async componentDidMount() {
        const stocks = ["IBM", "AAPL", "TSLA", "AMZN", "MSFT", "INTC", "NFLX", "COST", "NVDA"]
        for (const ticker_symbol of stocks) {
            console.log(ticker_symbol)
            await invoke("get_company_name", {tickerSymbol: ticker_symbol}).then(async company_name => {
                await invoke("get_company_exchange", {tickerSymbol: ticker_symbol}).then(async company_exchange => {
                    console.log(ticker_symbol)
                    await request_ticker_data(ticker_symbol).then(ticker_data => {
                        const price = price_from_data(ticker_data)
                        const change = change_from_data(ticker_data)
                        const date = last_date_from_data(ticker_data)
                        const historical_prices = get_list_prices(ticker_data)
                        let data = {
                            symbol: ticker_symbol,
                            name: company_name,
                            exchange: company_exchange,
                            price: price.toFixed(2),
                            change: change.toFixed(2),
                            date: date,
                            historical_prices: historical_prices
                        }
                        let stock_data = this.state.stock_data;
                        stock_data.push(data)
                        this.setState({stock_data})
                    })
                })
            })
        }


    }

    ticker_event = (event) => {
        ticker_price(event.target.value).then((price) => {
            this.setState({price: price})
        });
    };

    render() {
        const {stock_data} = this.state;
        return (<div className={"widgets-container"}>
            {/*<h1>Hello, World!</h1>*/}
            {/*<input onChange={this.ticker_event}/>*/}
            {/*{this.state.price && <p>Price: {this.state.price}</p>}*/}
            {stock_data.map(data => {
                const {symbol, name, exchange, price, change, date, historical_prices} = data;
                return <StockWidget symbol={symbol} name={name} exchange={exchange} price={price} change={change}
                                    date={date} historical_prices={historical_prices}/>
            })}
        </div>);
    }
}