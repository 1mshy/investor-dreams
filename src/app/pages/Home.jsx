import { has_favourites, top_favourite_changes } from "@/app/funcs/favourites";
import StockWidget from "@/components/widgets/StockWidget";
import { Component } from "react";

import "@/app/css/Homepage.css";
import "@/app/css/Playground.css";
import "@/app/css/Widgets.css";
import { nasdaq_sorted_by } from "@/app/funcs/stock_api";
import { invoke } from "@tauri-apps/api/core";
import { Link } from "react-router-dom";
import { fetch_subreddit_posts } from "../funcs/reddit";
import HeatMapPopup from "@/components/popups/HeatMapPopup";
import NewsWidget from "@/components/widgets/NewsWidget";
import { Button, Icon, IconButton } from "@mui/material";
import MACDChart from "@/components/Graphing/MACDChart";
import SettingsIcon from '@mui/icons-material/Settings';
import { SettingsContext } from '@/app/settings/SettingsContext';
import TradingViewWidget from '@/components/widgets/TradingViewWidget';

export default class Home extends Component {
    static contextType = SettingsContext;
    constructor(props) {
        super(props);
        this.state = {
            username: "",
            top_3_changes: [],
            bottom_3_changes: [],
            top_favs: [],
            heatmap: false,
            stock_size: "small",
        };
    }

    async componentDidMount() {
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
        const { username, top_3_changes, bottom_3_changes, top_favs, heatmap, stock_size } =
            this.state;
        return (
            <div className={"homepage-mainPage"}>
                <div className={"homepage-header"} data-tauri-drag-region>
                    <h1
                        className={"homepage-title"}
                        style={{ display: "inline-flex" }}
                    >
                        Welcome, {username}
                    </h1>
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
                        <IconButton component={Link} to="/settings">
                            <SettingsIcon />
                        </IconButton>
                        {/* <Link to="/settings" className={"homepage-navButton"} component={<IconButton >
                            <SettingsIcon/>
                        </IconButton>}>
                            
                        </Link> */}

                        {/* <Link to="/opportunities" className={"homepage-navButton"}>
                            Opportunities
                        </Link>
                        <Link to="/tradingview" className={"homepage-navButton"}>
                            Trading View
                        </Link> */}
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
                    {has_favourites() && (
                        <div className={"homepage-columns"}>
                            <h3>Favourites:</h3>
                            <div className={"homepage-favourties"}>
                                {top_favs.map((ticker_symbol) => {
                                    return (
                                        <StockWidget
                                            symbol={ticker_symbol}
                                            size={stock_size}
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
                                return (
                                    <StockWidget
                                        size={stock_size}
                                        symbol={ticker_symbol}
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
                                return (
                                    <StockWidget
                                        size={stock_size}
                                        symbol={ticker_symbol}
                                        key={ticker_symbol}
                                    />
                                );
                            })}
                        </div>
                    </div>
                    <div className={"homepage-columns"}>
                        <h3>Extra</h3>
                        <div className={"homepage-favourties"}>
                            <Button onClick={() => this.setState({ heatmap: true })}>Open Heat Map</Button>
                            <HeatMapPopup open={heatmap} onClick={() => this.setState({ heatmap: false })} />
                            <NewsWidget />
                        </div>
                    </div>
                    
                    {/* TradingView Widget Section */}
                    {this.context.settings.TradingView_Widget?.settings?.show_tradingview_on_home?.value && (
                        <div className="homepage-columns" style={{ width: "80%", marginTop: "2rem" }}>
                            <h3>Market Overview</h3>
                            <TradingViewWidget 
                                symbol={this.context.settings.TradingView_Widget.settings.default_symbol.value} 
                                range={this.context.settings.TradingView_Widget.settings.default_range.value}
                                theme={this.context.settings.TradingView_Widget.settings.dark_theme.value ? 'dark' : 'light'}
                            />
                        </div>
                    )}
                </div>
            </div>
        );
    }
}
