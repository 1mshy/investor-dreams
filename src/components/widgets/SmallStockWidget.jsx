;

import { get_month_prices, get_percent_change_month } from "@/app/funcs/historical_pricing";
import { format_currency_with_symbols } from "@/app/funcs/tools";
import { SoftPaper } from "@/app/mui/theme";
import StockGraph from "../Graphing/StockGraph";
import PercentageFormat from "../PercentageFormat";

/**
 * @param {String} symbol
 * @param {String} name
 * @param {Number} price
 * @param {Number} percent_change
 * @param {function} onClick
 * Small stock WIdget includes the same info as the mini, but includes a small graph of the month's pricing
 */
const SmallStockWidget = ({ symbol, name, price, percent_change, onClick, historical_prices, historical_data, show_name = true }) => {
    const percent_change_month = get_percent_change_month(historical_data);
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
                    <StockGraph timeset={"M"} symbol={symbol} size={"full"} />
                </div>
            </SoftPaper>
        </>
    );
};
export default SmallStockWidget;