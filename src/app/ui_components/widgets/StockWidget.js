"use client";

import {  useState } from "react";
import BigStockWidget from './BigStockWidget';
import SmallStockWidget from './SmallStockWidget';


/**
 * @param props.isPositive {boolean}
 * @type {IStyledComponent<"web", Substitute<import("react").DetailedHTMLProps<import("react").HTMLAttributes<HTMLDivElement>, HTMLDivElement>, BaseObject>> & BaseObject & {}}
 * @desc affects colour if positive or negative (green or red)
 */
export const StockChange = (props) => {
    return <div style={{ color: props.isPositive ? '#4caf50' : '#e74c3c' }}>
        {...props.children}
    </div>
}


export const StockWidget = ({ symbol, name, exchange, price, change, date, historical_prices }) => {
    const [big, setBig] = useState(false);
    return (
        <>
            <SmallStockWidget symbol={symbol} name={name} exchange={exchange} price={price} change={change} date={date} historical_prices={historical_prices} onClick={() => { setBig(true) }} />
            <BigStockWidget symbol={symbol} name={name} exchange={exchange} price={price} change={change} date={date} historical_prices={historical_prices} onClick={() => { setBig(false) }} open={big} />
        </>
    );
};