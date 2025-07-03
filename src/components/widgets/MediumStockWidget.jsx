import { is_ticker_favourite, toggle_favourite } from "@/app/funcs/favourites";
import { get_month_prices, get_percent_change_month, get_percent_change_ytd } from "@/app/funcs/historical_pricing";
import { format_currency_with_symbols } from "@/app/funcs/formatting";
import { format_number } from "@/app/funcs/formatting";
import { SoftPaper } from "@/app/mui/theme";
import PercentageFormat from "@/components/PercentageFormat";
import CustomSector from "@/components/popups/CustomSector";
import AddIcon from '@mui/icons-material/Add';
import FavoriteIcon from '@mui/icons-material/Favorite';
import FavoriteBorderIcon from '@mui/icons-material/FavoriteBorder';
import { IconButton } from "@mui/material";
import { useEffect, useState } from "react";
import StockGraph from "../Graphing/StockGraph";
import { invoke } from "@tauri-apps/api/core";
import { user_settings } from "@/app/settings/settings";

const MediumStockWidgetSettings = user_settings.Medium_Stock_Widget.settings; // shallow copy

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
    const [month_prices, set_month_prices] = useState(NaN);
    const [percent_change_month, set_percent_change_month] = useState(NaN);
    const [percent_change_ytd, set_percent_change_ytd] = useState(NaN);
    const [rsi, set_rsi] = useState("");

    useEffect(() => {
        set_favourite(is_ticker_favourite(symbol));
        if (historical_data && historical_data.length > 0) {
            set_month_prices(get_month_prices(historical_data));
            set_percent_change_month(get_percent_change_month(historical_data));
            set_percent_change_ytd(get_percent_change_ytd(historical_data));
            const old_first_historical_data = historical_data.slice().reverse();
            if (MediumStockWidgetSettings.show_rsi.value) {
                invoke("rsi", {
                    prices: old_first_historical_data.map(data => Number(data.close)), period: 14,
                }).then((rsi_values) => {
                    set_rsi(format_number(rsi_values[rsi_values.length - 1]));
                }).catch(() => set_rsi("Unknown"));
            }
        } else {
            set_month_prices(NaN);
            set_percent_change_month(NaN);
            set_percent_change_ytd(NaN);
            set_rsi("Unknown");
        }
    }, [symbol, historical_data]);

    useEffect(() => {
        if (!historical_data) return;

    }, [historical_data]);

    return (
        <>
            <SoftPaper className={"container"} style={{ width: "40rem", height: "max-content" }} onClick={() => {
                if (month_prices === NaN) return; // Check for NaN explicitly
                onClick();
            }}>
                <div className={"widget-header"}>
                    <div>
                        <div className={"ticker_symbol"}>{symbol || "Unknown"}</div>
                        <div className={"company_name"}>{name || "Unknown"}</div>
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
                    <div className={"price"}>${price || "Unknown"}</div>
                    <StockGraph timeset={"M"} symbol={symbol} />
                    <div className={"price-data"}>
                        <div className={"price-change"}>
                            {MediumStockWidgetSettings.show_day.value && <PercentageFormat percent_change={percent_change} timeset={"D"} />}
                            {MediumStockWidgetSettings.show_month.value && <PercentageFormat percent_change={percent_change_month} timeset={"M"} />}
                            {MediumStockWidgetSettings.show_ytd.value && <PercentageFormat percent_change={percent_change_ytd} timeset={"YTD"} />}
                        </div>
                        <div className={"date"}>
                            {MediumStockWidgetSettings.show_market_cap.value && marketCap && <div className={"market-cap"}>MC: {format_currency_with_symbols(marketCap)}</div>}
                            {MediumStockWidgetSettings.show_rsi.value && rsi && <div className={"rsi"}>RSI: {rsi}</div>}
                        </div>
                    </div>
                </div>
            </SoftPaper>
        </>
    );
};
export default MediumStockWidget;