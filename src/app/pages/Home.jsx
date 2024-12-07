import { has_favourites, top_favourite_changes } from "@/app/funcs/favourites";
import StockWidget from "@/components/widgets/StockWidget";
import { Component } from "react";

import "@/app/css/Homepage.css";
import "@/app/css/Playground.css";
import "@/app/css/Widgets.css";
import { extractInfo } from "@/app/funcs/reddit";
import { nasdaq_sorted_by } from "@/app/funcs/stock_api";
import { invoke } from "@tauri-apps/api/core";
import { Link } from "react-router-dom";

export default class Home extends Component {
    constructor(props) {
        super(props);
        this.state = {
            username: "",
            top_3_changes: [],
            bottom_3_changes: [],
            top_favs: [],
        };
    }

    async componentDidMount() {
        extractInfo();

        // Only access browser-specific APIs here
        console.log("getting username");
        const username = await invoke("get_username");
        this.setState({ username });

        const top_favs = await top_favourite_changes();
        const top_500 = (await nasdaq_sorted_by("marketCap")).slice(0, 500);
        const top_500_change = await nasdaq_sorted_by("pctchange", top_500);
        const top_3_changes = top_500_change.slice(0, 3);
        const bottom_3_changes = top_500_change
            .slice(top_500_change.length - 4, top_500_change.length - 1)
            .reverse();

        this.setState({
            top_favs,
            top_3_changes,
            bottom_3_changes,
        });
    }

    render() {
        const { username, top_3_changes, bottom_3_changes, top_favs } =
            this.state;
        const sampleData = [
            { date: "2024-11-19", open: 100, high: 110, low: 95, close: 105, volume: 5000 },
            { date: "2024-11-20", open: 105, high: 115, low: 100, close: 110, volume: 6000 },
            { date: "2024-11-21", open: 110, high: 120, low: 105, close: 115, volume: 7000 },
        ];
        return (
            <div className={"homepage-mainPage"}>
                <div className={"homepage-header"} data-tauri-drag-region>
                    <h1
                        className={"homepage-title"}
                        style={{ display: "inline-flex" }}
                    >
                        Welcome, {username}
                    </h1>
                    {/* <div style={{ display: "inline-flex" }}>
                        <StockSearch label="" variant="standard" fullWidth />
                    </div> */}
                    <div className={"homepage-nav"}>
                        <Link to="/playground" className={"homepage-navButton"}>
                            Playground
                        </Link>
                        <Link to="/portfolio" className={"homepage-navButton"}>
                            Portfolio
                        </Link>
                        <Link to="/analysis" className={"homepage-navButton"}>
                            Analysis
                        </Link>
                        <Link to="/tradingview" className={"homepage-navButton"}>
                            Trading View
                        </Link>
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
                <div className={"homepage-content"} data-tauri-drag-region>
                    {/* <div className={"widgets-container"}> */}
                    {has_favourites() && (
                        <div className={"homepage-columns"}>
                            <h3>Favourites:</h3>
                            <div className={"homepage-favourties"}>
                                {top_favs.map((ticker_symbol) => {
                                    // console.log(top_favs)
                                    return (
                                        <StockWidget
                                            symbol={ticker_symbol}
                                            size={"small"}
                                            key={ticker_symbol}
                                        />
                                    );
                                })}
                            </div>
                        </div>
                    )}
                    <div className={"homepage-columns"}>
                        <h3>Best Performing</h3>
                        <div className={"top3-list"}>
                            {top_3_changes.map((ticker_symbol) => {
                                // const { change, ticker_symbol, company, current_price, percent_change, portfolio_percent } = ticker_data;
                                return (
                                    <StockWidget
                                        size={"small"}
                                        symbol={ticker_symbol}
                                        // name={company}
                                        // price={current_price}
                                        // percent_change={percent_change}
                                        key={ticker_symbol}
                                    />
                                );
                            })}
                        </div>
                    </div>
                    <div className={"homepage-columns"}>
                        <h3>Worst performing</h3>
                        <div className={"bottom3-list"}>
                            {bottom_3_changes.map((ticker_symbol) => {
                                // const { change, ticker_symbol, company, current_price, percent_change, portfolio_percent } = ticker_data;
                                return (
                                    <StockWidget
                                        size={"small"}
                                        symbol={ticker_symbol}
                                        // name={company}
                                        // price={current_price}
                                        // percent_change={percent_change}
                                        key={ticker_symbol}
                                    />
                                );
                            })}
                        </div>
                        {/* </div> */}
                    </div>
                </div>
            </div>
        );
    }
}
