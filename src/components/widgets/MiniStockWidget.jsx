;

import { SoftPaper } from "@/app/mui/theme";
import PercentageFormat from "../PercentageFormat";
import { get_month_change } from "@/app/funcs/historical_pricing";
import { useState } from "react";
import { useEffect } from "react";

/**
 * @param {String} symbol
 * @param {String} name
 * @param {Number} price
 * @param {Number} percent_change
 * @param {function} onClick
 * @desc Smallest stock widget, only shows necceary information
 */
const MiniStockWidget = ({ symbol, name, price, percent_change, onClick, historical_data }) => {
    const [month_change, set_month_change] = useState(0);
    useEffect(() => {
        if (historical_data)
            set_month_change(get_month_change(historical_data));
    });
    return (
        <>
            <SoftPaper className={"container"} style={{ height: "max-content" }} onClick={onClick}>
                <div className={"widget-header"}>
                    <div>
                        <div className={"ticker_symbol"}>{symbol}</div>
                        <div className={"company_name"}>{name}</div>
                    </div>
                </div>
                <div className={"content"}>
                    <div className={"price"}>${price}</div>
                    <div className={"price-data"}>
                        <div className={"price-change"}>
                            <PercentageFormat percent_change={percent_change} timeset={"D"} />
                            <PercentageFormat percent_change={month_change} timeset={"M"} />
                        </div>
                    </div>
                </div>
            </SoftPaper>
        </>
    );
};
export default MiniStockWidget;