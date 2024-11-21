import { SolidPaper } from '@/app/mui/theme';
import { Backdrop, Dialog } from '@mui/material';
import { Transition } from '../../app/funcs/themes';
import TradingViewWidget from '../widgets/TradingViewWidget';

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
const TradingViewPopup = (props) => {
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
                        maxWidth: "95%",
                        height: "100%",
                        maxHeight: "85%",
                        zIndex: 1300, // Ensure Dialog has lower z-index than the tooltip
                        padding: 0,
                    }
                }}
            >
                {/* Allows dragging from top of popup */}
                <div style={{
                    padding: "1rem",
                    width: "100%",
                    height: "100%",
                }} data-tauri-drag-region>
                    <TradingViewWidget {...props} />
                </div>
            </Dialog>
        </Backdrop>
    );
};
export default TradingViewPopup;