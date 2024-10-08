;

import { get_month_prices } from "@/app/funcs/historical_pricing";
import PercentageFormat from "../PercentageFormat";
import PriceGraph from "../PriceGraph";
import { SoftPaper } from "@/app/mui/theme";

/**
 * @param {String} symbol
 * @param {String} name
 * @param {Number} price
 * @param {Number} percent_change
 * @param {function} onClick
 * Small stock WIdget includes the same info as the mini, but includes a small graph of the month's pricing
 */
const SmallStockWidget = ({ symbol, name, price, percent_change, percent_change_month, onClick, historical_prices, show_name = true }) => {
    const month_prices = get_month_prices(historical_prices)
    return (
        <>
            <SoftPaper className={"container"} style={{ height: "max-content" }} onClick={() => {
                if (!historical_prices || !name) return;
                onClick();
            }}>
                <div className={"widget-header"}>
                    <div style={{ width: "100%" }}>
                        <div className={"ticker_symbol"}>{symbol}</div>
                        <div style={{ float: "right", background: "inherit", zIndex: 100000000 }}>
                            <div className={"price-change"} style={{  background: "inherit", zIndex: 100000000 }}>
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
            </SoftPaper>
        </>
    );
};
export default SmallStockWidget;