"use client";

import { useEffect, useState } from "react";
import PriceGraph from "@/components/PriceGraph";
import PercentageFormat from "../PercentageFormat";
import FavoriteBorderIcon from '@mui/icons-material/FavoriteBorder';
import FavoriteIcon from '@mui/icons-material/Favorite';
import { IconButton } from "@mui/material";
import { is_ticker_favourite, toggle_favourite } from "@/app/funcs/favourites";
import AddIcon from '@mui/icons-material/Add';
import GeneralPopup from "../popups/GeneralPopup";
import { get_month_prices } from "@/app/funcs/historical_pricing";
import CustomSector from "@/app/ui_components/popups/CustomSector";

/**
 * @param {string} symbol
 * @param {string} name
 * @param {number} price
 * @param {number} percent_change
 * @param {string} date
 * @param {Array<number>} historical_prices
 * @param {function} onClick
 * @param {string} size - "big" or "medium" or "mini"
 * @desc Medium sized stock widget, includes a price graph and a price change percentage
 */
const MediumStockWidget = ({ symbol, name, price, percent_change, percent_change_month, date, historical_prices, onClick }) => {
    const [is_favourite, set_favourite] = useState(is_ticker_favourite(symbol));
    const month_prices = get_month_prices(historical_prices);
    return (
        <>
            <div className={"container"} style={{ width: "40rem" }} onClick={onClick}>
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
                    <PriceGraph prices={month_prices} />
                    <div className={"price-data"}>
                        <div className={"price-change"}>
                            {percent_change && <PercentageFormat percent_change={percent_change} />}
                            {percent_change_month && <PercentageFormat percent_change={percent_change_month} timeset={"M"} />}
                        </div>
                        <div className={"date"}>
                            {date}
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
};
export default MediumStockWidget;