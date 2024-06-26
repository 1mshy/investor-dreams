"use client";

import PriceGraph from "@/components/PriceGraph";
import PercentageFormat from "../PercentageFormat";

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
const BigStockWidget = ({ symbol, name, exchange, price, percent_change, date, historical_prices }) => {
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
                <div className={"company_name"}>{name} ({exchange})</div>
            </div>
            <div className={"content"}>
                <div className={"price"}>${price}</div>
                <PriceGraph prices={historical_prices} size={"big"} />
                <div className={"price-data"}>
                    <div className={"price-change"}>
                        <PercentageFormat percent_change={percent_change}/>
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