"use client";

import {useEffect, useState} from "react";
import PriceGraph from "@/app/ui_components/PriceGraph";
import { StockChange } from './StockWidget';

const SmallStockWidget = ({symbol, name, exchange, price, change, date, historical_prices, onClick}) => {
    const [isPositive, setIsPositive] = useState(change >= 0);
    const [big, setBig] = useState(false);
    // Optionally, use an effect to update isPositive when the change prop updates
    useEffect(() => {
        setIsPositive(change >= 0);
    }, [change]);
    return (
        <>
        <div className={"container"} onClick={onClick}>
            <div className={"head"}>
                <div className={"ticker_symbol"}>{symbol}</div>
                <div className={"company_name"}>{name} ({exchange})</div>
            </div>
            <div className={"content"}>
                <div className={"price"}>${price}</div>
                <PriceGraph prices={historical_prices}/>
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