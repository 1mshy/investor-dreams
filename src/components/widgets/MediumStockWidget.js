"use client";

import { useEffect, useState } from "react";
import PriceGraph from "@/components/PriceGraph";
import { StockChange } from './DynamicStockWidget';
import FavoriteBorderIcon from '@mui/icons-material/FavoriteBorder';
import FavoriteIcon from '@mui/icons-material/Favorite';
import { IconButton } from "@mui/material";
import { is_ticker_favourite, toggle_favourite } from "@/app/funcs/tools";

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
const MediumStockWidget = ({ symbol, name, price, percent_change, date, historical_prices, onClick }) => {
    const [isPositive, setIsPositive] = useState(percent_change >= 0);
    const [is_favourite, set_favourite] = useState(is_ticker_favourite(symbol));
    // Optionally, use an effect to update isPositive when the change prop updates
    useEffect(() => {
        setIsPositive(percent_change >= 0);
    }, [percent_change]);
    return (
        <>
            <div className={"container"} onClick={onClick}>
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
                    </div>
                </div>
                <div className={"content"}>
                    <div className={"price"}>${price}</div>
                    <PriceGraph prices={historical_prices} />
                    <div className={"price-data"}>
                        <div className={"price-change"}>
                            <StockChange isPositive={isPositive}>{isPositive ? '+' : ''}{percent_change}%</StockChange>
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