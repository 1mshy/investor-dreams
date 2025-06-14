import {
    get_month_prices,
    get_percent_change_all,
    get_percent_change_five_year,
    get_percent_change_month,
    get_percent_change_ten_year,
    get_percent_change_year,
    get_percent_change_ytd
} from "@/app/funcs/historical_pricing";
import {get_all_news_bodies, get_whole_nasdaq_news_url} from "@/app/networking/scraper";
import {fetch_ticker_summary, generate_ollama_message, percentage_change} from "@/app/networking/stock_api";
import {to_tradingview_range, trim_title} from "@/app/funcs/tools";
import {format_currency, format_number_with_commas, format_percentage, unformat_number} from "@/app/funcs/formatting";
import {format_number} from "@/app/funcs/formatting";
import {MarketColouredBadge} from "@/app/mui/other";
import {SettingsContext} from '@/app/settings/SettingsContext';
import ButtonPercentageFormat from "@/components/ButtonPercentageFormat";
import PercentageFormat from "@/components/PercentageFormat";
import TradingViewPopup from "@/components/tradingview/TradingViewPopup";
import {Button} from "@mui/material";
import {open} from "@tauri-apps/plugin-shell";
import {useContext, useEffect, useState} from "react";

import "@/app/css/Widgets.css";
import {rsi_reading} from "@/app/funcs/algorithms";
import {fetch_common_subreddits, fetch_subreddit_posts} from "@/app/networking/reddit";
import {invoke} from "@tauri-apps/api/core";
import StockGraph from "../Graphing/StockGraph";
import {financials_link} from "@/app/funcs/formatting";

/**
 * @param {Object} props
 * @param {String} props.symbol
 * @param {String} props.name
 * @param {String} props.exchange
 * @param {Number} props.price
 * @param {Number} props.percent_change
 * @param {String} props.date
 * @param {Number[]} props.historical_prices
 * @param {HistoricalData} props.historical_data
 * @param {TotalStockData} props.total_stock_data
 * @param {String} props.size - "big" or "medium" or "mini"
 * @desc Popup on the screen, blocks all other elements to focus on this one.
 * It is large and includes the most detail out of all the stock widgets
 */
const BigStockWidget = (props) => {
    const {settings} = useContext(SettingsContext);
    const bigSettings = settings.Big_Stock_Widget.settings;
    const {
        symbol,
        name,
        price,
        percent_change,
        historical_prices,
        marketCap,
        news,
        technicals,
        historical_data,
        total_stock_data
    } = props;
    const [graph_prices, set_graph_prices] = useState(get_month_prices(historical_data));
    const [ticker_info, set_ticker_info] = useState("");
    const [show_ollama_button, set_show_ollama_button] = useState(true);
    const [ollama_summary, set_ollama_summary] = useState("");
    const [trading_view_popup, set_trading_view_popup] = useState(false);
    const [rsi, set_rsi] = useState(0);
    const [rsi_values, set_rsi_values] = useState([]);
    const [today_high_low, set_today_high_low] = useState("");
    const [forcasted_rsi, set_forcasted_rsi] = useState(0);
    const [forcasted_rsi_days, set_forcasted_rsi_days] = useState(10);
    const [subreddit_data, set_subreddit_data] = useState([]);
    const [common_subreddit_data, set_common_subreddit_data] = useState([]);
    const [exchange, set_exchange] = useState("");
    /**
     * @desc The time set for the graph
     * @type {BigStockWidgetRange}
     */
    const [timeset, set_timeset] = useState("M");

    useEffect(() => {
        const complex_operations = async () => {
            const today_high = format_number(historical_data[0].high);
            const today_low = format_number(historical_data[0].low);
            set_today_high_low(`${today_high} / ${today_low}`);

            const old_first_historical_data = historical_data.slice().reverse();

            invoke("rsi", {
                prices: old_first_historical_data.map(data => Number(data.close)), period: 14,
            }).then((rsi_values) => {
                set_rsi_values(rsi_values);
                set_rsi(format_number(rsi_values[rsi_values.length - 1]));
            });

            invoke("monte_carlo_rsi", {
                prices: old_first_historical_data.map(data => Number(data.close)), numSimulations: 1000,
                forecastDays: forcasted_rsi_days,
                period: 14,
            }).then((forcasted_rsi) => {
                set_forcasted_rsi(format_number(forcasted_rsi[0]));
            });


            const ticker_info = await fetch_ticker_summary(symbol);
            set_ticker_info(ticker_info);
            set_exchange(ticker_info.quoteType.exchange);

            const subreddit_data = await fetch_subreddit_posts(symbol);
            set_subreddit_data(subreddit_data);

            const common_subreddit_data = await fetch_common_subreddits();
            console.log(common_subreddit_data)
            let usefull_subreddits = [];
            for (let subreddit of Object.keys(common_subreddit_data)) {
                for (let post of common_subreddit_data[subreddit]) {
                    if (post.title.toLowerCase().includes(symbol.toLowerCase())) {
                        usefull_subreddits.push(post);
                    }
                }
            }
            set_common_subreddit_data(usefull_subreddits);

        }
        complex_operations();
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

    const percent_change_month = get_percent_change_month(historical_data);
    const percent_change_ytd = get_percent_change_ytd(historical_data);
    const percent_change_year = get_percent_change_year(historical_data);
    const percent_change_five_year = get_percent_change_five_year(historical_data);
    const percent_change_ten_year = get_percent_change_ten_year(historical_data);
    const percent_change_all = get_percent_change_all(historical_data);

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
        dividend_amount = unformat_number(technicals.AnnualizedDividend?.value);
        if (dividend_amount === 0) {
            if (technicals.SpecialDividendAmount && technicals.SpecialDividendAmount.value !== "N/A") {
                dividend_amount = unformat_number(technicals.SpecialDividendAmount.value) * 4;
            }
        }
    }

    const dividend_yield = dividend_amount / unformatted_price * 100;

    const rsi_dataset = {
        data: rsi_values,
        label: 'RSI',
        fill: false,
        borderColor: 'rgb(75, 192, 192)',
        tension: 0.1
    }

    const single_name = String(name).split(" ")[0];

    console.log(ticker_info)

    return (
        <div className={"big"} data-tauri-drag-region
             onClick={(e) => {
                 e.stopPropagation();
                 e.preventDefault();
                 e.nativeEvent.stopImmediatePropagation();
             }}
        >
            <div className={"head"} data-tauri-drag-region>
                <MarketColouredBadge>
                    <div className={"ticker_symbol"}>{single_name}</div>
                </MarketColouredBadge>
                <div className={"company_name"}>{name}</div>
            </div>
            <div className={"content"}>
                {bigSettings.show_technicals.value && technicals && <div className={"elements"}>
                    {technicals.FiftTwoWeekHighLow && <div className={"data-element"}>
                        <div className={"info-title"}>{`${technicals.FiftTwoWeekHighLow.label}:`}</div>
                        <div
                            className={"info-value"}>{`${technicals.FiftTwoWeekHighLow.value.replace("/", " / ")}`}</div>
                    </div>}
                    <div className={"data-element"}>
                        <div className={"info-title"}>{`Today's High/Low:`}</div>
                        <div className={"info-value"}>{`${today_high_low}`}</div>
                    </div>
                    {technicals.PERatio && <div className={"data-element"}>
                        <div className={"info-title"}>{`(PE/FPE):`}</div>
                        <div
                            className={"info-value"}>{`${technicals.PERatio.value} / ${technicals.ForwardPE1Yr.value}`}</div>
                    </div>}

                    {technicals.EarningsPerShare && <div className={"data-element"}>
                        <div className={"info-title"}>{`(EPS/TTP):`}</div>
                        <div
                            className={"info-value"}>{`${technicals.EarningsPerShare.value} (${format_percentage(unformat_number(technicals.EarningsPerShare.value) / unformatted_price * 100)})`}</div>
                    </div>}
                    {technicals.AnnualizedDividend && <div className={"data-element"}>
                        <div className={"info-title"}>{`${technicals.AnnualizedDividend?.label}:`}</div>
                        <div
                            className={"info-value"}>{`$${format_number(dividend_amount)} ${isNaN(dividend_amount) ? '' : `(${format_percentage(dividend_yield)})`}`}</div>
                    </div>}
                    <div className={"data-element"}>
                        <div className={"info-title"}>{`RSI:`}</div>
                        <div className={"info-value"}>{`${rsi} (${rsi_reading(rsi)})`}</div>
                    </div>
                    <div className={"data-element"}>
                        <div className={"info-title"}>{`Forcasted RSI (${forcasted_rsi_days} days):`}</div>
                        <div className={"info-value"}>{`${forcasted_rsi} (${rsi_reading(forcasted_rsi)})`}</div>
                    </div>
                    {technicals.OneYrTarget && <div className={"data-element"}>
                        <div className={"info-title"}>{`${technicals.OneYrTarget.label}:`}</div>
                        <div className={"info-value"}>
                            <div className={"data-element"}>
                                <div>{`${technicals.OneYrTarget.value}`}</div>
                                (<PercentageFormat percent_change={price_target_change}/>)
                            </div>
                        </div>
                    </div>}
                    <Button onClick={() => {
                        set_trading_view_popup(!trading_view_popup);
                    }}>View on tradingview</Button>

                </div>
                }
                <StockGraph symbol={symbol} size={"big"} timeset={timeset}/>
                {/* <IndicatorGraph symbol={symbol} size={"big"} timeset={timeset} indicators={["rsi"]}/> */}
                {/* <CombinedGraph symbol={symbol} size={"big"} timeset={timeset}/> */}
                {/* <PriceGraph prices={rsi_values.slice(-30)} /> */}
                <TradingViewPopup {...props} range={to_tradingview_range(timeset)} open={trading_view_popup}
                                  onClick={() => {
                                      set_trading_view_popup(false)
                                  }}/>
                {historical_data && <div className={"price-data"}>
                    <div className={"price-change"}>
                        <ButtonPercentageFormat variant={timeset == "D" ? "outlined" : ""}
                                                percent_change={percent_change} timeset={"D"} func={() => {
                            set_timeset("D")
                        }}/>
                        <ButtonPercentageFormat variant={timeset == "M" ? "outlined" : ""}
                                                percent_change={percent_change_month} timeset={"M"} func={() => {
                            set_timeset("M")
                        }}/>
                        <ButtonPercentageFormat variant={timeset == "YTD" ? "outlined" : ""}
                                                percent_change={percent_change_ytd} timeset={"YTD"} func={() => {
                            set_timeset("YTD")
                        }}/>
                        <ButtonPercentageFormat variant={timeset == "Y" ? "outlined" : ""}
                                                percent_change={percent_change_year} timeset={"Y"} func={() => {
                            set_timeset("Y")
                        }}/>
                        <ButtonPercentageFormat variant={timeset == "5Y" ? "outlined" : ""}
                                                percent_change={percent_change_five_year} timeset={"5Y"} func={() => {
                            set_timeset("5Y")
                        }}/>
                        <ButtonPercentageFormat variant={timeset == "10Y" ? "outlined" : ""}
                                                percent_change={percent_change_ten_year} timeset={"10Y"} func={() => {
                            set_timeset("10Y")
                        }}/>
                        <ButtonPercentageFormat variant={timeset == "ALL" ? "outlined" : ""}
                                                percent_change={percent_change_all} timeset={"ALL"} func={() => {
                            set_timeset("ALL")
                        }}/>
                    </div>
                    <div className={"date"}>
                        Updated {new Date(historical_data[0].datetime).toLocaleDateString()}
                    </div>
                </div>}

            </div>
            {ticker_info && <div className={"info"}>
                <div className={"info-section"}>
                    {ticker_info.assetProfile && <>
                        <h1> {symbol}</h1>
                        <div>
                            <div className={"info-title"}>Sector</div>
                            <div className={"info-value"}>{ticker_info.assetProfile.sector}</div>
                            <div className={"info-title"}>Industry</div>
                            <div className={"info-value"}>{`${ticker_info.assetProfile.industry}`}</div>
                        </div>
                    </>}
                </div>
                <div className={"info-section"}>
                    {ticker_info.assetProfile &&
                        <div>
                            <div className={"info-title"}>Headquarters Location</div>
                            <div className={"info-value"}>
                                {`${ticker_info.assetProfile.city}, ${ticker_info.assetProfile.state}, ${ticker_info.assetProfile.country}`}
                            </div>
                            <div className={"info-title"}>Website</div>
                            <div className={"info-value"} style={{cursor: "pointer"}} onClick={async () => {
                                await open(ticker_info.assetProfile.website);
                            }}>
                                {ticker_info.assetProfile.website}
                            </div>
                        </div>}
                </div>
                <div className={"info-section"}>
                    <div className={"info-title"}>Yesterday</div>
                    <div className={"info-value"}>${format_number_with_commas(yesterday_price)}</div>
                    <div className={"info-title"}>Currently</div>
                    <div className={"price"}>${format_number_with_commas(price)}</div>
                    <div className={"info-title"}>Market Cap</div>
                    <div className={"price"}>{format_currency(marketCap)}</div>
                </div>

            </div>}
            {bigSettings.show_financial_data.value && ticker_info &&
                <div className={"info-section"} style={{overflowX: 'auto'}}>
                    <div className={"info-title"}>Financial Data</div>
                    <div className={"financial-data-grid"}>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Current Ratio</div>
                            <div className={"info-value"}>{ticker_info.financialData.currentRatio.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Debt to Equity</div>
                            <div className={"info-value"}>{ticker_info.financialData.debtToEquity.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Earnings Growth</div>
                            <div className={"info-value"}>{ticker_info.financialData.earningsGrowth.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>EBITDA</div>
                            <div className={"info-value"}>{ticker_info.financialData.ebitda.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>EBITDA Margins</div>
                            <div className={"info-value"}>{ticker_info.financialData.ebitdaMargins.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Gross Margins</div>
                            <div className={"info-value"}>{ticker_info.financialData.grossMargins.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Operating Margins</div>
                            <div className={"info-value"}>{ticker_info.financialData.operatingMargins.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Profit Margins</div>
                            <div className={"info-value"}>{ticker_info.financialData.profitMargins.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Quick Ratio</div>
                            <div className={"info-value"}>{ticker_info.financialData.quickRatio.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Return on Assets</div>
                            <div className={"info-value"}>{ticker_info.financialData.returnOnAssets.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Return on Equity</div>
                            <div className={"info-value"}>{ticker_info.financialData.returnOnEquity.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Revenue Growth</div>
                            <div className={"info-value"}>{ticker_info.financialData.revenueGrowth.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Revenue Per Share</div>
                            <div className={"info-value"}>{ticker_info.financialData.revenuePerShare.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Total Cash</div>
                            <div className={"info-value"}>{ticker_info.financialData.totalCash.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Total Cash Per Share</div>
                            <div className={"info-value"}>{ticker_info.financialData.totalCashPerShare.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Total Debt</div>
                            <div className={"info-value"}>{ticker_info.financialData.totalDebt.fmt}</div>
                        </div>
                        <div className={"data-row"}>
                            <div className={"info-label"}>Total Revenue</div>
                            <div className={"info-value"}>{ticker_info.financialData.totalRevenue.fmt}</div>
                        </div>
                    </div>
                </div>}
            <div>

                <div className={"info-title"}>Links:</div>
                <div className={"info-value"} style={{cursor: "pointer", width: "fit-content"}} onClick={async () => {
                    await open(financials_link(symbol, exchange))
                }}>View Historical Finances of {single_name} on the {exchange} exchange
                </div>
            </div>
            {bigSettings.show_company_info.value && ticker_info && <div className="summary" style={{width: "100%"}}>
                <div className={"info-title"}>
                    {"Summary"}
                </div>
                <div className={""}>
                    {ticker_info.assetProfile?.longBusinessSummary}
                </div>
                {bigSettings.show_reddit_data.value && subreddit_data && <>
                    <div className={"info-title"}>
                        {"Reddit Headlines"}
                    </div>
                    <div className="reddit-news">
                        {subreddit_data.map((post, index) => {
                            return <div className={"news-row"} key={index} style={{cursor: "pointer"}}
                                        onClick={async () => {
                                            await open(post.url);
                                        }}>
                                {post.title}
                            </div>
                        })}
                    </div>
                    {common_subreddit_data && <div className="common-reddit">
                        {common_subreddit_data.map((post, index) => {
                            return <div className={"news-row"} key={index} style={{cursor: "pointer"}}
                                        onClick={async () => {
                                            await open(post.url);
                                        }}>
                                {post.title}
                            </div>
                        })}
                    </div>}
                </>}


                {bigSettings.show_news.value && news && <>
                    <div className={"info-title"}>
                        {"News Headlines"}
                    </div>
                    <div>
                        {news.map((article, index) => {
                            return <div className={"news-row"} key={index} style={{cursor: "pointer"}}
                                        onClick={async () => {
                                            await open(get_whole_nasdaq_news_url(article.url));
                                        }}>
                                {trim_title(article.title)}
                            </div>
                        })}

                    </div>
                </>}

                {/* <div className={"info-title"} >
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
                </div> */}
            </div>}
        </div>
    );
};

export default BigStockWidget;