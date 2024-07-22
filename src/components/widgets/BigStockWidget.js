"use client";

import PriceGraph from "@/components/PriceGraph";
import PercentageFormat from "../PercentageFormat";
import { get_five_year_prices, get_month_change, get_month_prices, get_percent_change_five_year, get_percent_change_month, get_percent_change_ten_year, get_ten_year_prices, get_year_change, get_year_prices, get_ytd_change, get_ytd_prices } from "@/app/funcs/historical_pricing";
import ButtonPercentageFormat from "../ButtonPercentageFormat";
import { useEffect, useState } from "react";
import { get_index_info, get_ticker_info } from "@/app/funcs/stock_api";
import { open } from "@tauri-apps/api/shell";
import { ArrowBack } from "@mui/icons-material";
import { IconButton } from "@mui/material";

/**
 * @param {string} symbol
 * @param {string} name
 * @param {string} exchange
 * @param {number} price
 * @param {number} percent_change
 * @param {string} date
 * @param {Array<number>} historical_prices
 * @param {string} size - "big" or "medium" or "mini"
 * @desc Popup on the screen, blocks all other elements to focus on this one.
 *      It is large and includes the most detail out of all the stock widgets
 */
const BigStockWidget = ({ symbol, name, price, percent_change, date, historical_prices }) => {



    const [graph_prices, set_graph_prices] = useState(get_month_prices(historical_prices));
    const [ticker_info, set_ticker_info] = useState({});

    useEffect(() => {
        get_ticker_info(symbol).then((info) => {
            set_ticker_info(info);
        });
    });

    const percent_change_month = get_percent_change_month(historical_prices);
    const percent_change_ytd = get_ytd_change(historical_prices);
    const percent_change_year = get_year_change(historical_prices);
    const percent_change_five_year = get_percent_change_five_year(historical_prices);
    const percent_change_ten_year = get_percent_change_ten_year(historical_prices);


    return (
        <div className={"big"}
            onClick={(e) => {
                e.stopPropagation();
                e.preventDefault();
                e.nativeEvent.stopImmediatePropagation();
            }}
        >
            <div className={"head"}>
                <div className={"ticker_symbol"}>{symbol}</div>
                <div className={"company_name"}>{name}</div>
            </div>
            <div className={"content"}>
                <PriceGraph prices={graph_prices} size={"big"} />
                <div className={"price-data"}>
                    <div className={"price-change"}>
                        <ButtonPercentageFormat percent_change={percent_change} timeset={"D"} func={() => { set_graph_prices(get_month_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_month} timeset={"M"} func={() => { set_graph_prices(get_month_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_ytd} timeset={"YTD"} func={() => { set_graph_prices(get_ytd_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_year} timeset={"Y"} func={() => { set_graph_prices(get_year_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_five_year} timeset={"5Y"} func={() => { set_graph_prices(get_five_year_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_ten_year} timeset={"10Y"} func={() => { set_graph_prices(get_ten_year_prices(historical_prices)) }} />
                    </div>
                    <div className={"date"}>
                        {date}
                    </div>
                </div>

            </div>
            <div className={"info"}>
                <div className={"info-section"}>
                    {ticker_info && <>
                        <h2>Info on {String(ticker_info.name).split(",")[0]}</h2>
                        <div>
                            <div className={"info-title"}>Exchange</div>
                            <div className={"info-value"}>{ticker_info.exchange}</div>
                            <div className={"info-title"}>Sector</div>
                            <div className={"info-value"}>{ticker_info.sector}</div>
                            <div className={"info-title"}>Industry</div>
                            <div className={"info-value"}>{`${ticker_info.industry} (${ticker_info.industry_group})`}</div>
                        </div>
                    </>}
                </div>
                <div className={"info-section"}>
                    {ticker_info &&
                        <div>
                            <div className={"info-title"}>Headquarters Location</div>
                            <div className={"info-value"}>{`${ticker_info.state},
                             ${ticker_info.city}, ${ticker_info.country}`}</div>
                            <div className={"info-title"}>Market</div>
                            <div className={"info-value"}>{ticker_info.market}</div>
                            <div className={"info-title"}>Website</div>
                            <div className={"info-value"} style={{ cursor: "pointer" }} onClick={() => {
                                open(ticker_info.website);
                            }}>
                                {ticker_info.website}
                            </div>
                        </div>}
                </div>
                <div className={"info-section"}>
                    <div className={"info-title"}>Yesterday</div>
                    <div className={"info-value"}>${historical_prices[historical_prices.length - 2]}</div>
                    <div className={"info-title"}>Currently</div>
                    <div className={"price"}>${price}</div>
                </div>

            </div>
            {ticker_info && <div className="summary" style={{ width: "100%" }}>
                <div className={"info-title"} >
                    <div>
                        {"Summary"}
                    </div>
                </div>
                <div className={""}>{ticker_info.summary}</div>
            </div>}
        </div>
    );
};

export default BigStockWidget;