"use client";

import { useEffect, useState } from "react";
import PriceGraph from "@/components/PriceGraph";
import { Backdrop, Dialog } from '@mui/material';
import { StockChange } from './StockWidget';
import { Transition } from '../../app/funcs/themes';

const BigStockWidget = ({ symbol, name, exchange, price, change, date, historical_prices, onClick, open }) => {
    const [isPositive, setIsPositive] = useState(change >= 0);

    // Optionally, use an effect to update isPositive when the change prop updates
    useEffect(() => {
        setIsPositive(change >= 0);
    }, [change]);

    return (
        <Backdrop open={open} onClick={onClick} invisible={true} style={{ width: "100%", maxWidth: "100%" }}>
            <Dialog
                open={open}
                aria-labelledby="responsive-dialog-title"
                TransitionComponent={Transition}
                maxWidth='lg'
                fullWidth
                PaperProps={{
                    sx: {
                        width: "100%",
                        maxWidth: "90%",
                        height: "100%",
                        maxHeight: "80%",
                    }
                }}

            >
                <div className={"big"}
                    onClick={(e) => {
                        e.stopPropagation();
                        e.preventDefault();
                        e.nativeEvent.stopImmediatePropagation();
                    }}
                >
                    <div className={"head"}>
                        <div className={"ticker_symbol"}>{symbol}</div>
                        <div className={"company_name"}>{name} ({exchange})</div>
                    </div>
                    <div className={"content"}>
                        <div className={"price"}>${price}</div>
                        <PriceGraph prices={historical_prices} size={"big"} />
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
            </Dialog>
        </Backdrop>
    );
};
export default BigStockWidget;