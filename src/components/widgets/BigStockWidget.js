"use client";

import PriceGraph from "@/components/PriceGraph";
import PercentageFormat from "../PercentageFormat";
import { get_five_year_prices, get_month_change, get_month_prices, get_percent_change_five_year, get_percent_change_month, get_percent_change_ten_year, get_ten_year_prices, get_year_change, get_year_prices } from "@/app/funcs/historical_pricing";
import ButtonPercentageFormat from "../ButtonPercentageFormat";
import { useState } from "react";

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

    const percent_change_month = get_percent_change_month(historical_prices);
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
                <div className={"price"}>${price}</div>
                <PriceGraph prices={graph_prices} size={"big"} />
                <div className={"price-data"}>
                    <div className={"price-change"}>
                        <ButtonPercentageFormat percent_change={percent_change} timeset={"D"} func={() => { set_graph_prices(get_month_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_month} timeset={"M"} func={() => { set_graph_prices(get_month_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_year} timeset={"Y"} func={() => { set_graph_prices(get_year_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_five_year} timeset={"5Y"} func={() => { set_graph_prices(get_five_year_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_ten_year} timeset={"10Y"} func={() => { set_graph_prices(get_ten_year_prices(historical_prices)) }} />
                    </div>
                    <div className={"date"}>
                        {date}
                    </div>
                </div>
            </div>
        </div>
    );
};

export default BigStockWidget;