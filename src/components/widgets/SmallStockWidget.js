"use client";

import { useEffect, useState } from "react";
import PriceGraph from "@/components/PriceGraph";
import { StockChange } from './StockWidget';
import FavoriteBorderIcon from '@mui/icons-material/FavoriteBorder';
import FavoriteIcon from '@mui/icons-material/Favorite';
import { IconButton } from "@mui/material";
import { is_ticker_favourite, toggle_favourite } from "@/app/funcs/tools";
const SmallStockWidget = ({ symbol, name, price, change, date, historical_prices, onClick }) => {
    const [isPositive, setIsPositive] = useState(change >= 0);
    const [is_favourite, set_favourite] = useState(is_ticker_favourite(symbol));
    // Optionally, use an effect to update isPositive when the change prop updates
    useEffect(() => {
        setIsPositive(change >= 0);
    }, [change]);
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
                            <StockChange isPositive={isPositive}>{isPositive ? '+' : ''}{change}%</StockChange>
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
export default SmallStockWidget;