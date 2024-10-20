import { get_five_year_prices, get_month_prices, get_percent_change_five_year, get_percent_change_month, get_percent_change_ten_year, get_percent_change_year, get_percent_change_ytd, get_ten_year_prices, get_year_prices, get_ytd_prices } from "@/app/funcs/historical_pricing";
import { get_all_news_bodies, get_whole_nasdaq_news_url } from "@/app/funcs/scraper";
import { generate_ollama_message, get_ticker_info, percentage_change } from "@/app/funcs/stock_api";
import { format_currency, format_number, format_percentage, unformat_number } from "@/app/funcs/tools";
import { MarketColouredBadge } from "@/app/mui/other";
import PriceGraph from "@/components/PriceGraph";
import { Button, CircularProgress } from "@mui/material";
import { open } from "@tauri-apps/plugin-shell";
import { useEffect, useState } from "react";
import ButtonPercentageFormat from "../ButtonPercentageFormat";
import PercentageFormat from "../PercentageFormat";

import "@/app/css/Widgets.css";

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
const BigStockWidget = (props) => {
    const { symbol, name, price, percent_change, date, historical_prices, marketCap, news, technicals, historical_data } = props;
    const [graph_prices, set_graph_prices] = useState(get_month_prices(historical_prices));
    const [ticker_info, set_ticker_info] = useState({});
    const [show_ollama_button, set_show_ollama_button] = useState(true);
    const [ollama_summary, set_ollama_summary] = useState("");

    useEffect(() => {
        get_ticker_info(symbol).then((info) => {
            set_ticker_info(info);
        });
    }, []);
    /**
     * @desc Generates a current summary using the news articles
     */
    const generate_summary = async () => {
        const bodies = await get_all_news_bodies(news, symbol)
        const prompt = `From the following information, find the most relevant details on the publicly traded company ${symbol}:\n${bodies}`
        const generated_summary = await generate_ollama_message(prompt)
        set_ollama_summary(generated_summary);
    }

    const percent_change_month = get_percent_change_month(historical_prices);
    const percent_change_ytd = get_percent_change_ytd(historical_prices);
    const percent_change_year = get_percent_change_year(historical_prices);
    const percent_change_five_year = get_percent_change_five_year(historical_prices);
    const percent_change_ten_year = get_percent_change_ten_year(historical_prices);

    const yesterday_price = historical_prices ? historical_prices[historical_prices.length - 2] : ""

    let unformatted_target = 0
    let unformatted_price = 0
    let price_target_change = 0
    let dividend_amount = 0
    // technicals may be null on first render
    if (technicals) {
        unformatted_target = unformat_number(technicals.OneYrTarget.value)
        unformatted_price = unformat_number(price);
        price_target_change = percentage_change(unformatted_target, unformatted_price)
        dividend_amount = unformat_number(technicals.AnnualizedDividend.value);
        if (dividend_amount === 0) {
            if (technicals.SpecialDividendAmount && technicals.SpecialDividendAmount.value !== "N/A") {
                dividend_amount = unformat_number(technicals.SpecialDividendAmount.value) * 4;
            }
        }
    }
    const dividend_yield = dividend_amount / unformatted_price * 100;

    return (
        <div className={"big"} data-tauri-drag-region
            onClick={(e) => {
                e.stopPropagation();
                e.preventDefault();
                e.nativeEvent.stopImmediatePropagation();
            }}
        >
            <div className={"head"}>
                <MarketColouredBadge >
                    <div className={"ticker_symbol"}>{symbol}</div>
                </MarketColouredBadge>

                <div className={"company_name"}>{name}</div>
            </div>
            <div className={"content"} >
                {technicals && <div className={"elements"}>
                    <div className={"data-element"}>
                        <div className={"info-title"}>{`${technicals.FiftTwoWeekHighLow.label}:`}</div>
                        <div className={"info-value"}>{`${technicals.FiftTwoWeekHighLow.value.replace("/", " / ")}`}</div>
                    </div>
                    <div className={"data-element"}>
                        <div className={"info-title"}>{`${technicals.TodayHighLow.label}:`}</div>
                        <div className={"info-value"}>{`${technicals.TodayHighLow.value.replace("/", " / ")}`}</div>
                    </div>
                    <div className={"data-element"}>
                        <div className={"info-title"}>{`(PE/FPE):`}</div>
                        <div className={"info-value"}>{`${technicals.PERatio.value} / ${technicals.ForwardPE1Yr.value}`}</div>
                    </div>
                    <div className={"data-element"}>
                        <div className={"info-title"}>{`(EPS/TTP):`}</div>
                        <div className={"info-value"}>{`${technicals.EarningsPerShare.value} (${format_percentage(unformat_number(technicals.EarningsPerShare.value) / unformatted_price * 100)})`}</div>
                    </div>
                    <div className={"data-element"}>
                        <div className={"info-title"}>{`${technicals.AnnualizedDividend.label}:`}</div>
                        <div className={"info-value"}>{`$${format_number(dividend_amount)} (${format_percentage(dividend_yield)})`}</div>
                    </div>
                    <div className={"data-element"}>
                        <div className={"info-title"}>{`${technicals.OneYrTarget.label}:`}</div>
                        <div className={"info-value"}>
                            <div className={"data-element"}>
                                <div>{`${technicals.OneYrTarget.value}`}</div>
                                (<PercentageFormat percent_change={price_target_change} />)
                            </div>
                        </div>
                    </div>

                </div>
                }
                <PriceGraph prices={graph_prices} size={"big"} historical_data={historical_data}/>
                {historical_prices && <div className={"price-data"}>
                    <div className={"price-change"}>
                        <ButtonPercentageFormat percent_change={percent_change} timeset={"D"} func={() => { set_graph_prices(get_month_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_month} timeset={"M"} func={() => { set_graph_prices(get_month_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_ytd} timeset={"YTD"} func={() => { set_graph_prices(get_ytd_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_year} timeset={"Y"} func={() => { set_graph_prices(get_year_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_five_year} timeset={"5Y"} func={() => { set_graph_prices(get_five_year_prices(historical_prices)) }} />
                        <ButtonPercentageFormat percent_change={percent_change_ten_year} timeset={"10Y"} func={() => { set_graph_prices(get_ten_year_prices(historical_prices)) }} />
                    </div>
                    <div className={"date"}>
                        {date}
                    </div>
                </div>}

            </div>
            <div className={"info"}>
                <div className={"info-section"}>
                    {ticker_info && <>
                        <h2>Info on {String(ticker_info.name).split(",")[0]}</h2>
                        <div>
                            <div className={"info-title"}>Exchange</div>
                            <div className={"info-value"}>{ticker_info.exchange}</div>
                            <div className={"info-title"}>Sector</div>
                            <div className={"info-value"}>{ticker_info.sector}</div>
                            <div className={"info-title"}>Industry</div>
                            <div className={"info-value"}>{`${ticker_info.industry} (${ticker_info.industry_group})`}</div>
                        </div>
                    </>}
                </div>
                <div className={"info-section"}>
                    {ticker_info &&
                        <div>
                            <div className={"info-title"}>Headquarters Location</div>
                            <div className={"info-value"}>{`${ticker_info.state},
                             ${ticker_info.city}, ${ticker_info.country}`}</div>
                            <div className={"info-title"}>Market</div>
                            <div className={"info-value"}>{ticker_info.market}</div>
                            <div className={"info-title"}>Website</div>
                            <div className={"info-value"} style={{ cursor: "pointer" }} onClick={async () => {
                                await open(ticker_info.website);
                            }}>
                                {ticker_info.website}
                            </div>
                        </div>}
                </div>
                <div className={"info-section"}>
                    <div className={"info-title"}>Yesterday</div>
                    <div className={"info-value"}>${yesterday_price}</div>
                    <div className={"info-title"}>Currently</div>
                    <div className={"price"}>${price}</div>
                    <div className={"info-title"}>Market Cap</div>
                    <div className={"price"}>{format_currency(marketCap)}</div>
                </div>

            </div>
            {ticker_info && <div className="summary" style={{ width: "100%" }}>
                <div className={"info-title"} >
                    {"Summary"}
                </div>
                <div className={""}>{ticker_info.summary}</div>
                <div className={"info-title"} >
                    {"News Headlines"}
                </div>
                {news && <div>
                    {news.map((article, index) => {
                        return <div className={"news-row"} key={index} style={{ cursor: "pointer" }} onClick={async () => {
                            await open(get_whole_nasdaq_news_url(article.url));
                        }}>
                                {article.title}
                        </div>
                    })}
                </div>}
                <a href="https://example.com" target="_blank">Example link</a>

                <div className={"info-title"} >
                    {"LLM generated summary"}
                </div>
                <div>
                    {!ollama_summary && (
                        <Button
                            onClick={async () => {
                                set_show_ollama_button(false);
                                await generate_summary();
                            }}
                            disabled={!show_ollama_button}>
                            {!show_ollama_button ? (<>
                                Generating...
                                <CircularProgress size={20} style={{ marginLeft: '10px' }} />
                            </>
                            ) : (
                                "Generate Summary"
                            )}
                        </Button>
                    )}
                    {ollama_summary}
                </div>
            </div>}
        </div>
    );
};

export default BigStockWidget;