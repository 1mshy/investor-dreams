"use client";

import { get_month_prices } from "@/app/funcs/historical_pricing";
import PercentageFormat from "../PercentageFormat";
import PriceGraph from "../PriceGraph";

/**
 * @param {string} symbol
 * @param {string} name
 * @param {number} price
 * @param {number} percent_change
 * @param {function} onClick
 * Small stock WIdget includes the same info as the mini, but includes a small graph of the month's pricing
 */
const SmallStockWidget = ({ symbol, name, price, percent_change, percent_change_month, onClick, historical_prices, show_name = true }) => {
    const month_prices = get_month_prices(historical_prices)
    return (
        <>
            <div className={"container"} onClick={onClick}>
                <div className={"widget-header"}>
                    <div style={{ width: "100%" }}>
                        <div className={"ticker_symbol"}>{symbol}</div>
                        <div style={{ float: "right" }}>
                            <div className={"price-change"}>
                                {percent_change && <PercentageFormat percent_change={percent_change} timeset={"D"} />}
                                {percent_change_month && <PercentageFormat percent_change={percent_change_month} timeset={"M"} />}
                            </div>
                            <div className={"smaller-price"}>${price}</div>
                        </div>
                        {show_name && <div className={"company_name"}>{name}
                        </div>}
                    </div>
                </div>
                <div className={"content"}>
                    <PriceGraph prices={month_prices} size={"full"} />
                </div>
            </div>
        </>
    );
};
export default SmallStockWidget;