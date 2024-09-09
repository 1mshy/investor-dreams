"use client";

import { Backdrop, Dialog } from '@mui/material';
import { Transition } from '../../app/funcs/themes';
import BigStockWidget from "./BigStockWidget";
import { SoftPaper, SolidPaper } from '@/app/mui/theme';

/**
 * @param {String} symbol
 * @param {String} name
 * @param {String} exchange
 * @param {Number} price
 * @param {Number} percent_change
 * @param {String} date
 * @param {Array<number>} historical_prices
 * @param {String} size - "big" or "medium" or "mini"
 * @desc Popup on the screen, blocks all other elements to focus on this one.
 *      It is large and includes the most detail out of all the stock widgets
 */
const PopupWidget = (props) => {
    const { onClick, open } = props;

    return (
        <Backdrop open={open} data-tauri-drag-region onClick={onClick} invisible={true} style={{ width: "100%", maxWidth: "100%" }}>
            <Dialog
            data-tauri-drag-region
                open={open}
                aria-labelledby="responsive-dialog-title"
                TransitionComponent={Transition}
                maxWidth='lg'
                fullWidth
                PaperComponent={SolidPaper}
                PaperProps={{
                    sx: {
                        width: "100%",
                        maxWidth: "90%",
                        height: "100%",
                        maxHeight: "80%",
                    }
                }}
            >
                <BigStockWidget {...props} />
            </Dialog>
        </Backdrop>
    );
};
export default PopupWidget;