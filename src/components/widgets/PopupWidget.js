"use client";

import { Backdrop, Dialog } from '@mui/material';
import { Transition } from '../../app/funcs/themes';
import BigStockWidget from "./BigStockWidget";
import { SoftPaper } from '@/app/mui/theme';

/**
 * @param {string} symbol
 * @param {string} name
 * @param {string} exchange
 * @param {number} price
 * @param {number} percent_change
 * @param {string} date
 * @param {Array<number>} historical_prices
 * @param {string} size - "big" or "medium" or "mini"
 * @desc Popup on the screen, blocks all other elements to focus on this one.
 *      It is large and includes the most detail out of all the stock widgets
 */
const PopupWidget = ({ symbol, name, exchange, price, percent_change, date, historical_prices, onClick, open }) => {

    return (
        <Backdrop open={open} onClick={onClick} invisible={true} style={{ width: "100%", maxWidth: "100%" }}>
            <Dialog
                open={open}
                aria-labelledby="responsive-dialog-title"
                TransitionComponent={Transition}
                maxWidth='lg'
                fullWidth
                PaperComponent={SoftPaper}
                PaperProps={{
                    sx: {
                        width: "100%",
                        maxWidth: "90%",
                        height: "100%",
                        maxHeight: "80%",
                    }
                }}
            >
                <BigStockWidget symbol={symbol} name={name} exchange={exchange} price={price}
                    percent_change={percent_change} date={date} historical_prices={historical_prices} />
            </Dialog>
        </Backdrop>
    );
};
export default PopupWidget;