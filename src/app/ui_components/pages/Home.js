"use client"
import "@/app/css/Widgets.css";
import { has_favourites, top_favourite_changes } from '@/app/funcs/favourites';
import { get_sp_500_data } from '@/app/funcs/scraper';
import StockWidget from '@/components/widgets/StockWidget';
import { invoke } from "@tauri-apps/api/core"
import Link from 'next/link';
import { Component } from 'react';

import "@/app/css/Playground.css";
import "../../css/Homepage.css";


export default class Home extends Component {
    constructor(props) {
        super(props);
        this.state = {
            username: "",
            top_3_changes: [],
            worst_3_changes: [],
            top_favs: []
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
            console.log(response)
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

        top_favourite_changes().then(top_favs => {
            this.setState({ top_favs });
        });
    }

    render() {
        const { username, top_3_changes, worst_3_changes, top_favs } = this.state;
        return (
            <div className={"homepage-mainPage"}>
                <div className={"homepage-header"}>
                    <h1 className={"homepage-title"} style={{ display: "inline-flex" }}>To the moon {username} 🚀🚀</h1>
                    {/* <div style={{ display: "inline-flex" }}>
                        <StockSearch label="" variant="standard" fullWidth />
                    </div> */}
                    <div className={"homepage-nav"} >
                    <Link href="/playground" className={"homepage-navButton"}>Playground</Link>
                    <Link href="/predictions" className={"homepage-navButton"}>Betting</Link>
                        {/* <Link href="/playground" className={"homepage-navButton"}>Pages</Link>
                        <Link
                            href={{
                                pathname: '/tickers',
                                query: { ticker_symbol: 'AAPL' },
                            }}
                            passHref
                        >
                            trest ticker</Link> */}
                        {/* <Button onClick={clear_application_data}>Clear Application Data</Button> */}
                    </div>
                </div>
                <div className={"homepage-content"}>
                    {/* <div className={"widgets-container"}> */}
                    {has_favourites() &&
                        <div className={"homepage-columns"}>
                            <h3>Favourites:</h3>
                            <div className={"homepage-favourties"}>
                                {top_favs.map(ticker_symbol => {
                                    // console.log(top_favs)
                                    // get_sp_500_data().then((response) => { console.log(response) });
                                    return <StockWidget symbol={ticker_symbol} size={"small"}
                                        key={ticker_symbol} />
                                })}
                            </div>
                        </div>}
                    <div className={"homepage-columns"}>
                        <h3>Best Performing</h3>
                        <div className={"top3-list"}>
                            {top_3_changes.map((ticker_data) => {
                                const { change, ticker_symbol, company, current_price, percent_change, portfolio_percent } = ticker_data;
                                return <StockWidget
                                    size={"small"}
                                    symbol={ticker_symbol}
                                    name={company}
                                    price={current_price}
                                    percent_change={percent_change}
                                    key={ticker_symbol}
                                />
                            })}
                        </div>
                    </div>
                    <div className={"homepage-columns"}>
                        <h3>Worst performing</h3>
                        <div className={"bottom3-list"}>
                            {worst_3_changes.map((ticker_data) => {
                                const { change, ticker_symbol, company, current_price, percent_change, portfolio_percent } = ticker_data;
                                return <StockWidget
                                    size={"small"}
                                    symbol={ticker_symbol}
                                    name={company}
                                    price={current_price}
                                    percent_change={percent_change}
                                    key={ticker_symbol}
                                />
                            })}
                        </div>
                        {/* </div> */}
                    </div>
                </div>
            </div>
        );
    }
}