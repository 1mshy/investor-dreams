;

import { SoftPaper } from "@/app/mui/theme";
import PercentageFormat from "../PercentageFormat";

/**
 * @param {String} symbol
 * @param {String} name
 * @param {Number} price
 * @param {Number} percent_change
 * @param {function} onClick
 * @desc Smallest stock widget, only shows necceary information
 */
const MiniStockWidget = ({ symbol, name, price, percent_change, onClick }) => {
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
                        <PercentageFormat percent_change={percent_change}/>
                        </div>
                    </div>
                </div>
            </SoftPaper>
        </>
    );
};
export default MiniStockWidget;