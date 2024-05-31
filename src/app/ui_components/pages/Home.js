"use client"
import React, { Component } from 'react';
import Link from 'next/link';
import { invoke } from '@tauri-apps/api';
import { get_sp_500_data } from '@/app/funcs/scraper';
import MiniStockWidget from '@/components/widgets/MiniStockWidget';
import "../../css/Homepage.css";
import "@/app/css/Widgets.css"
import StockSearch from '@/components/searching/SeachBoxes';
import { get_favourite_array, get_favourites, has_favourites } from '@/app/funcs/tools';
import ImplementedDynamicStockWidget from '@/components/widgets/ImplementedDynamicStockWidget';


export default class Home extends Component {
    constructor(props) {
        super(props);
        this.state = {
            username: "",
            top_3_changes: [],
            worst_3_changes: []
        }
    }

    async componentDidMount() {
        // Only access browser-specific APIs here
        invoke("get_username").then((response) => {
            this.setState({ username: response });
        }).catch(error => {
            console.error("Failed to fetch username:", error);
        });
        // TODO algorithm to get top 3 changes
        get_sp_500_data().then((response) => {
            const changes = Object.keys(response)
                .sort((a, b) => response[b].percent_change - response[a].percent_change);
            const top3 = changes
                .slice(0, 3)
                .map((ticker_symbol) => response[ticker_symbol]);
            const bottom3 = changes
                .slice(changes.length - 4, changes.length - 1)
                .map(ticker_symbol => response[ticker_symbol])
                .reverse();
            // console.log(top_3_changes)
            this.setState({ top_3_changes: top3 });
            this.setState({ worst_3_changes: bottom3 });
        })
    }

    render() {
        const { username, top_3_changes, worst_3_changes } = this.state;
        return (
            <div className={"homepage-mainPage"}>
                <header className={"homepage-header"}>
                    <h1 className={"homepage-title"}>To the moon {username} 🚀🚀</h1>
                    <div>
                        <StockSearch label="" variant="standard" fullWidth />
                    </div>
                    <nav className={"homepage-nav"}>
                        <Link href="/playground" className={"homepage-navButton"}>Join Playground</Link>
                        <Link
                            href={{
                                pathname: '/tickers',
                                query: { ticker_symbol: 'AAPL' },
                            }}
                            passHref
                        >
                            trest ticker</Link>
                    </nav>
                </header>

                <div className={"homepage-content"}>
                    {has_favourites() && <div>
                        <h3>Favourites:</h3>
                        <div className={"homepage-favourties"}>

                            {get_favourite_array().map(ticker_symbol => {
                                return <ImplementedDynamicStockWidget ticker_symbol={ticker_symbol} size={"mini"} />
                            })}
                        </div>
                    </div>}
                    <div>
                        <h3>Best Performing</h3>
                        <div className={"top3-list"}>
                            {top_3_changes.map((ticker_data) => {
                                const { change, ticker_symbol, company, current_price, percent_change, portfolio_percent } = ticker_data;
                                return <MiniStockWidget
                                    symbol={ticker_symbol}
                                    name={company}
                                    price={current_price}
                                    percent_change={percent_change}
                                    key={ticker_symbol}
                                />
                            })}
                        </div>
                    </div>
                    <div>
                        <h3>Worst performing</h3>
                        <div className={"bottom3-list"}>
                            {worst_3_changes.map((ticker_data) => {
                                const { change, ticker_symbol, company, current_price, percent_change, portfolio_percent } = ticker_data;
                                return <MiniStockWidget
                                    symbol={ticker_symbol}
                                    name={company}
                                    price={current_price}
                                    percent_change={percent_change}
                                    key={ticker_symbol}
                                />
                            })}
                        </div>
                    </div>
                </div>
            </div>
        );
    }
}