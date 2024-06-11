"use client"
import "@/app/css/Widgets.css";
import { has_favourites, top_favourite_changes } from '@/app/funcs/favourites';
import { get_sp_500_data } from '@/app/funcs/scraper';
import StockSearch from '@/components/searching/SeachBoxes';
import ImplementedDynamicStockWidget from '@/components/widgets/ImplementedDynamicStockWidget';
import { invoke } from '@tauri-apps/api';
import Link from 'next/link';
import { Component } from 'react';
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
        invoke("get_current_monitor_info").then(response => {
            console.log(response)
            console.log(response.size)
        })
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
                        {/* <Button onClick={clear_application_data}>Clear Application Data</Button> */}
                    </nav>
                </header>

                <div className={"homepage-content"}>
                    {has_favourites() && <div>
                        <h3>Favourites:</h3>
                        <div className={"homepage-favourties"}>
                            {top_favs.map(ticker_symbol => {
                                console.log(top_favs)
                                get_sp_500_data().then((response) => { console.log(response) });
                                return <ImplementedDynamicStockWidget symbol={ticker_symbol} size={"small"}
                                    key={ticker_symbol} />
                            })}
                        </div>
                    </div>}
                    <div>
                        <h3>Best Performing</h3>
                        <div className={"top3-list"}>
                            {top_3_changes.map((ticker_data) => {
                                const { change, ticker_symbol, company, current_price, percent_change, portfolio_percent } = ticker_data;
                                return <ImplementedDynamicStockWidget
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
                    <div>
                        <h3>Worst performing</h3>
                        <div className={"bottom3-list"}>
                            {worst_3_changes.map((ticker_data) => {
                                const { change, ticker_symbol, company, current_price, percent_change, portfolio_percent } = ticker_data;
                                return <ImplementedDynamicStockWidget
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
                </div>
            </div>
        );
    }
}