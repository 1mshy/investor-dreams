"use client";

import styled from 'styled-components';
import {useEffect, useState} from "react";
import PriceGraph from "@/app/ui_components/PriceGraph";


/**
 * @param props.isPositive {boolean}
 * @type {IStyledComponent<"web", Substitute<import("react").DetailedHTMLProps<import("react").HTMLAttributes<HTMLDivElement>, HTMLDivElement>, BaseObject>> & BaseObject & {}}
 * @desc affects colour if positive or negative (green or red)
 */
const StockChange = (props) => {
    return <div style={{color: props.isPositive ? '#4caf50' : '#e74c3c'}}>
        {...props.children}
    </div>
}


const StockWidget = ({symbol, name, exchange, price, change, date, historical_prices}) => {
    const [isPositive, setIsPositive] = useState(change >= 0);

    // Optionally, use an effect to update isPositive when the change prop updates
    useEffect(() => {
        setIsPositive(change >= 0);
    }, [change]);
    return (
        <div className={"container"}>
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
    );
};
export default StockWidget;