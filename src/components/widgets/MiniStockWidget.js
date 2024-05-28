"use client";

import { useEffect, useState } from "react";
import { StockChange } from './DynamicStockWidget';

/**
 * @param {string} symbol
 * @param {string} name
 * @param {number} price
 * @param {number} percent_change
 * @param {function} onClick
 * @desc Smallest stock widget, only shows necceary information
 */
const MiniStockWidget = ({ symbol, name, price, percent_change, onClick }) => {
    const [isPositive, setIsPositive] = useState(percent_change >= 0);
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
                </div>
                <div className={"content"}>
                    <div className={"price"}>${price}</div>
                    <div className={"price-data"}>
                        <div className={"price-change"}>
                            <StockChange isPositive={isPositive}>{isPositive ? '+' : ''}{percent_change}%</StockChange>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
};
export default MiniStockWidget;