import { is_ticker_favourite, toggle_favourite } from "@/app/funcs/favourites";
import { get_month_prices, get_percent_change_month, get_percent_change_ytd } from "@/app/funcs/historical_pricing";
import { format_currency, format_currency_with_symbols } from "@/app/funcs/tools";
import { SoftPaper } from "@/app/mui/theme";
import CustomSector from "@/components/popups/CustomSector";
import PriceGraph from "@/components/PriceGraph";
import AddIcon from '@mui/icons-material/Add';
import FavoriteIcon from '@mui/icons-material/Favorite';
import FavoriteBorderIcon from '@mui/icons-material/FavoriteBorder';
import { IconButton } from "@mui/material";
import { useState } from "react";
import PercentageFormat from "@/components/PercentageFormat";
import { useEffect } from "react";
import StockGraph from "../StockGraph";

/**
 * @param {String} symbol
 * @param {String} name
 * @param {Number} price
 * @param {Number} percent_change
 * @param {String} date
 * @param {Array<number>} historical_prices
 * @param {function} onClick
 * @param {String} size - "big" or "medium" or "mini"
 * @desc Medium sized stock widget, includes a price graph and a price change percentage
 */
const MediumStockWidget = (props) => {
    const { symbol, name, price, percent_change, marketCap, onClick, historical_data } = props;
    const [is_favourite, set_favourite] = useState(false);
    const [month_prices, set_month_prices] = useState(null);
    const [percent_change_month, set_percent_change_month] = useState(null);
    const [percent_change_ytd, set_percent_change_ytd] = useState(null);

    useEffect(() => {
        set_favourite(is_ticker_favourite(symbol));
        set_month_prices(get_month_prices(historical_data));
        set_percent_change_month(get_percent_change_month(historical_data));
        set_percent_change_ytd(get_percent_change_ytd(historical_data));
    }, [symbol]);

    return (
        <>
            <SoftPaper className={"container"} style={{ width: "40rem", height: "max-content" }} onClick={() => {
                if (!month_prices) return;
                onClick();
            }}>
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
                        <CustomSector>
                            <IconButton onClick={(e) => {
                                e.stopPropagation();
                                e.preventDefault();
                                e.nativeEvent.stopImmediatePropagation();
                            }}>
                                <AddIcon />
                            </IconButton>
                        </CustomSector>
                    </div>
                </div>
                <div className={"content"}>
                    <div className={"price"}>${price}</div>
                    <StockGraph timeset={"M"} symbol={symbol} />
                    <div className={"price-data"}>
                        <div className={"price-change"}>
                            <PercentageFormat percent_change={percent_change} timeset={"D"} />
                            <PercentageFormat percent_change={percent_change_month} timeset={"M"} />
                            <PercentageFormat percent_change={percent_change_ytd} timeset={"YTD"} />
                        </div>
                        <div className={"date"}>
                            {marketCap && <div className={"market-cap"}>MC: {format_currency_with_symbols(marketCap)}</div>}
                        </div>
                    </div>
                </div>
            </SoftPaper>
        </>
    );
};
export default MediumStockWidget;