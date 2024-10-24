;

import { is_ticker_favourite, toggle_favourite } from "@/app/funcs/favourites";
import { get_month_prices } from "@/app/funcs/historical_pricing";
import { format_currency } from "@/app/funcs/tools";
import { SoftPaper } from "@/app/mui/theme";
import CustomSector from "@/app/ui_components/popups/CustomSector";
import PriceGraph from "@/components/PriceGraph";
import AddIcon from '@mui/icons-material/Add';
import FavoriteIcon from '@mui/icons-material/Favorite';
import FavoriteBorderIcon from '@mui/icons-material/FavoriteBorder';
import { IconButton } from "@mui/material";
import { useState } from "react";
import PercentageFormat from "../PercentageFormat";

/**
 * @param {String} symbol
 * @param {String} name
 * @param {Number} price
 * @param {Number} percent_change
 * @param {String} date
 * @param {Array<number>} historical_prices
 * @param {function} onClick
 * @param {String} size - "big" or "medium" or "mini"
 * @desc Medium sized stock widget, includes a price graph and a price change percentage
 */
const MediumStockWidget = (props) => {
    const { symbol, name, price, percent_change, percent_change_month, date, historical_prices, marketCap, onClick, historical_data } = props;

    const [is_favourite, set_favourite] = useState(is_ticker_favourite(symbol));
    const month_prices = get_month_prices(historical_prices);
    return (
        <>
            <SoftPaper className={"container"} style={{ width: "40rem", height: "max-content" }} onClick={() => {
                if(!month_prices) return;
                onClick();
            }}>
                <div className={"widget-header"}>
                    <div>
                        <div className={"ticker_symbol"}>{symbol}</div>
                        <div className={"company_name"}>{name}</div>
                    </div>
                    <div>
                        <IconButton onClick={(e) => {
                            set_favourite(!is_favourite);
                            toggle_favourite(symbol);
                            e.stopPropagation();
                            e.preventDefault();
                            e.nativeEvent.stopImmediatePropagation();
                        }}>
                            {is_favourite ? <FavoriteIcon /> : <FavoriteBorderIcon />}
                        </IconButton>
                        <CustomSector>
                            <IconButton onClick={(e) => {
                                e.stopPropagation();
                                e.preventDefault();
                                e.nativeEvent.stopImmediatePropagation();
                            }}>
                                <AddIcon />
                            </IconButton>
                        </CustomSector>
                    </div>
                </div>
                <div className={"content"}>
                    <div className={"price"}>${price}</div>
                    <PriceGraph prices={month_prices} historical_data={historical_data}/>
                    <div className={"price-data"}>
                        <div className={"price-change"}>
                            {percent_change && <PercentageFormat percent_change={percent_change} />}
                            {percent_change_month && <PercentageFormat percent_change={percent_change_month} timeset={"M"} />}
                        </div>
                        <div className={"date"}>
                            {marketCap && <div className={"market-cap"}>MC: {format_currency(marketCap)}</div>}
                            {date}
                        </div>
                    </div>
                </div>
            </SoftPaper>
        </>
    );
};
export default MediumStockWidget;