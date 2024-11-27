;

import { get_month_prices } from "@/app/funcs/historical_pricing";
import PercentageFormat from "../PercentageFormat";
import PriceGraph from "../PriceGraph";
import { SoftPaper } from "@/app/mui/theme";
import { format_currency_with_symbols } from "@/app/funcs/tools";

/**
 * @param {String} symbol
 * @param {String} name
 * @param {Number} price
 * @param {Number} percent_change
 * @param {function} onClick
 * Small stock WIdget includes the same info as the mini, but includes a small graph of the month's pricing
 */
const SmallStockWidget = ({ symbol, name, price, percent_change, percent_change_month, onClick, historical_prices, historical_data, show_name = true }) => {
    const month_prices = get_month_prices(historical_data)
    return (
        <>
            <SoftPaper className={"container"} onClick={() => {
                if (!historical_prices || !name) return;
                onClick();
            }}>
                <div className={"widget-header"}>
                    <div className={"small-widget-names"}>
                        <div className={"ticker_symbol"}>{symbol}</div>
                        {show_name &&
                            <div className={"small_company_name"}>{name}
                            </div>}
                    </div>
                    <div style={{ background: "inherit" }}>
                        <div className={"price-change"} style={{ background: "inherit" }}>
                            {percent_change && <PercentageFormat percent_change={percent_change} timeset={"D"} />}
                            {percent_change_month && <PercentageFormat percent_change={percent_change_month} timeset={"M"} />}
                        </div>
                        <div className={"smaller-price"}>{format_currency_with_symbols(price)}</div>
                    </div>
                </div>
                <div className={"content"}>
                    <PriceGraph prices={month_prices} size={"full"} historical_data={historical_data} />
                </div>
            </SoftPaper>
        </>
    );
};
export default SmallStockWidget;