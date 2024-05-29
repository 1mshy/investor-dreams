"use client"
import React, { Component } from 'react';
import Link from 'next/link';
import { invoke } from '@tauri-apps/api';
import { get_sp_500_data } from '@/app/funcs/scraper';
import MiniStockWidget from '@/components/widgets/MiniStockWidget';
import "../../css/Homepage.css";
import "@/app/css/Widgets.css"
import StockSearch from '@/components/searching/SeachBoxes';


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
                    <h1 className={"homepage-title"}>Investor Dreams</h1>
                    <div>
                        <StockSearch label="" variant="standard" fullWidth />
                    </div>
                    <nav className={"homepage-nav"}>
                        <Link href="/playground" className={"homepage-navButton"}>Top 10</Link>
                        <Link href="/" className={"homepage-navButton"}>Top 300</Link>
                    </nav>
                </header>

                <div className={"homepage-content"}>
                    <div>
                        <h2 className={"homepage-welcomeText"}>Welcome back {username}</h2>
                    </div>
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
                        <div>
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