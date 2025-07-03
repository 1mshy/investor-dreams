import { get_month_prices, get_percent_change_month } from "@/app/funcs/historical_pricing";
import { format_currency_with_symbols } from "@/app/funcs/formatting";
import { SoftPaper } from "@/app/mui/theme";
import StockGraph from "../Graphing/StockGraph";
import PercentageFormat from "../PercentageFormat";
import { useContext } from 'react';
import { SettingsContext } from '@/app/settings/SettingsContext';

const SmallStockWidget = ({
    symbol,
    name,
    price,
    percent_change,
    onClick,
    historical_prices,
    historical_data,
}) => {
    const { settings } = useContext(SettingsContext);
    const smallSettings = settings.Small_Stock_Widget.settings;
    const percent_change_month = historical_data ? get_percent_change_month(historical_data) : NaN;

    return (
        <>
            <SoftPaper className={"container"} onClick={() => {
                if (!historical_prices || !name) return;
                onClick();
            }}>
                <div className={"widget-header"}>
                    <div className={"small-widget-names"}>
                        <div className={"ticker_symbol"}>{symbol || "Unknown"}</div>
                        {smallSettings.show_name.value && (
                            <div className={"small_company_name"}>{name || "Unknown"}</div>
                        )}
                    </div>
                    <div style={{ background: "inherit" }}>
                        <div className={"price-change"} style={{ background: "inherit" }}>
                            {smallSettings.show_day.value && percent_change && (
                                <PercentageFormat percent_change={percent_change} timeset={"D"} />
                            )}
                            {smallSettings.show_month.value && percent_change_month && (
                                <PercentageFormat percent_change={percent_change_month} timeset={"M"} />
                            )}
                        </div>
                        <div className={"smaller-price"}>{format_currency_with_symbols(price)}</div>
                    </div>
                </div>
                <div className={"content"}>
                    {smallSettings.show_graph.value && (
                        <StockGraph timeset={"M"} symbol={symbol} size={"full"} />
                    )}
                </div>
            </SoftPaper>
        </>
    );
};

export default SmallStockWidget;