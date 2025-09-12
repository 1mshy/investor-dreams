import "@/app/css/Analysis.css";
import "@/app/css/Homepage.css";
import "@/app/css/Playground.css";
import { filter_tickers, filter_tickers_async, request_database, fetch_and_filter_stocks } from "@/app/funcs/analysis";
import { cache_is_valid, clear_cache, STOCK_CACHE } from "@/app/networking/cache";
import { get_all_nasdaq_info } from "@/app/networking/scraper";
import { save_dynamic_sector } from "@/app/funcs/sectors";
import { get_state } from "@/app/funcs/states";
import { SettingsContext } from "@/app/settings/SettingsContext";
import { fetch_widget_data, get_all_symbols, get_all_technical_data, get_all_technical_data_keys, get_cached_ticker_technicals, NASDAQ_NEWS, NASDAQ_TECHNICALS } from "@/app/networking/stock_api";
import { delay, is_complex_ticker } from "@/app/funcs/tools";
import { BackGroundPaper, theme } from "@/app/mui/theme";
import MarketCapSlider from "@/components/misc/MarketCapSlider";
import CustomSectorNamePopup from "@/components/popups/CustomSectorNamePopup";
import TableDownloadPopup from "@/components/popups/TableDownloadPopup";
import StockWidget from "@/components/widgets/StockWidget";
import { Button, Checkbox, FormControl, InputLabel, MenuItem, Select, Stack, TextField, ThemeProvider, Tooltip, Typography } from '@mui/material';
import localforage from "localforage";
import React, { Component } from "react";
import { Link } from "react-router-dom";
import { toast } from "react-toastify";

export default class Analysis extends Component {
    static contextType = SettingsContext;

    constructor(props) {
        super(props);

        this.state = {
            all_symbols: ["AAPL", "TSLA", "AMZN", "GOOGL", "MSFT"],
            downloadable_stores: [STOCK_CACHE, NASDAQ_TECHNICALS, NASDAQ_NEWS,],
            searched_symbols: new Set(),
            search_value: "",
            filtered_tickers: [],
            show_searching_options: false,
            is_loading: false,
            loading_status: "",
            loading_progress: 0,
            current_symbol: "",
            searching_options: {
                min_market_cap: 1_000_000_000,
                max_market_cap: 1_000_000_000_000_000,
                tickers_shown: 50,
                sort_by: "target_percent_difference",
                reverse: true,
            }
        }

        this.predictions = localforage.createInstance({
            "name": "predictions"
        });
        this.predict = this.predict.bind(this);
        this.fetch_all_data = this.fetch_all_data.bind(this);
        this.sort_all_tickers = this.sort_all_tickers.bind(this);
        this.toggle_searching_options = this.toggle_searching_options.bind(this);
        this.run_analysis = this.run_analysis.bind(this);
        this.load_existing_data = this.load_existing_data.bind(this);

        this.save_popup_ref = React.createRef(null);
    }

    openPopup() {
        this.save_popup_ref.current.open();
    };

    async componentDidMount() {
        try {
            const all_symbols = await get_all_symbols();
            this.setState({ all_symbols });
            
            // Auto-load existing data if available
            await this.load_existing_data();
        } catch (error) {
            console.error("Error in componentDidMount:", error);
            toast.error("Failed to load initial data");
        }
    }

    /**
     * Load and display analysis results from existing cached data
     */
    async load_existing_data() {
        const { searching_options } = this.state;
        this.setState({ 
            is_loading: true, 
            loading_status: "Loading existing data...",
            loading_progress: 0 
        });

        try {
            const filtered_results = await filter_tickers_async(searching_options);
            this.setState({ 
                filtered_tickers: filtered_results,
                is_loading: false,
                loading_status: filtered_results.length > 0 ? 
                    `Loaded ${filtered_results.length} stocks from cache` : 
                    "No cached data available - click 'Run Analysis' to fetch fresh data"
            });
        } catch (error) {
            console.error("Error loading existing data:", error);
            this.setState({ 
                is_loading: false, 
                loading_status: "Failed to load existing data",
                filtered_tickers: []
            });
        }
    }

    /**
     * Run a complete analysis with fresh data fetching
     */
    async run_analysis() {
        const { searching_options } = this.state;
        
        this.setState({ 
            is_loading: true, 
            loading_status: "Starting analysis...",
            loading_progress: 0,
            filtered_tickers: []
        });

        try {
            const results = await fetch_and_filter_stocks(
                searching_options,
                (symbol, progress) => {
                    this.setState({ 
                        current_symbol: symbol,
                        loading_progress: progress 
                    });
                },
                (status) => {
                    this.setState({ loading_status: status });
                }
            );

            this.setState({ 
                filtered_tickers: results,
                is_loading: false,
                loading_status: `Analysis complete! Found ${results.length} matching stocks.`,
                current_symbol: ""
            });

        } catch (error) {
            console.error("Error in run_analysis:", error);
            this.setState({ 
                is_loading: false,
                loading_status: "Analysis failed: " + error.message,
                filtered_tickers: []
            });
        }
    }
    /**
     * 
     * @param {Boolean} skip_cached - should ignore the cached data if any
     * @returns {Promise<void>}
     */
    async fetch_all_data(skip_cached = true) {
        // This method is now deprecated in favor of run_analysis()
        // Keeping for backwards compatibility
        console.warn("fetch_all_data is deprecated, use run_analysis() instead");
        
        if (!skip_cached) {
            await this.load_existing_data();
            return;
        }
        
        await this.run_analysis();
    }

    toggle_searching_options() {
        this.setState({ show_searching_options: !this.state.show_searching_options });
    }

    async sort_all_tickers() {
        // This method now uses the same logic as load_existing_data
        await this.load_existing_data();
    }
    predict(symbol) {
        this.predictions.setItem("AAPL", 180)
    }

    render() {
        const { all_symbols, search_value, searched_symbols, filtered_tickers, downloadable_stores,
            show_searching_options, searching_options, is_loading, loading_status, loading_progress, current_symbol } = this.state;
        const { settings } = this.context;
        const widgetSize = settings?.Analysis_Page?.settings?.default_widget_size?.value || 'small';

        return <ThemeProvider theme={theme}>
            <div className={"analysis-whole"}>
                <div className={"homepage-header"} data-tauri-drag-region>
                    <div className={"homepage-nav"} data-tauri-drag-region>
                        <Link to="/home" className={"homepage-navButton"}>Home</Link>
                    </div>
                </div>
                <div className="">
                    <div>
                        <h1>Stock Analysis</h1>
                        <div>
                            {`Total symbols available: ${all_symbols.length}`}
                        </div>
                        <div>
                            {`Matching stocks found: ${filtered_tickers.length}`}
                        </div>
                        {is_loading && (
                            <div style={{ marginTop: "10px" }}>
                                <div><strong>Status:</strong> {loading_status}</div>
                                {current_symbol && <div><strong>Processing:</strong> {current_symbol}</div>}
                                {loading_progress > 0 && (
                                    <div style={{ marginTop: "5px" }}>
                                        <div style={{ 
                                            width: "100%", 
                                            backgroundColor: "#e0e0e0", 
                                            borderRadius: "5px",
                                            height: "10px"
                                        }}>
                                            <div style={{ 
                                                width: `${loading_progress}%`, 
                                                backgroundColor: "#4caf50", 
                                                height: "10px",
                                                borderRadius: "5px",
                                                transition: "width 0.3s ease"
                                            }} />
                                        </div>
                                        <div style={{ textAlign: "center", fontSize: "12px", marginTop: "2px" }}>
                                            {loading_progress}%
                                        </div>
                                    </div>
                                )}
                            </div>
                        )}
                        {!is_loading && loading_status && (
                            <div style={{ marginTop: "10px", color: filtered_tickers.length > 0 ? "#4caf50" : "#ff9800" }}>
                                <strong>Status:</strong> {loading_status}
                            </div>
                        )}
                    </div>

                    <Stack spacing={2} direction={"row"}>
                        <Tooltip title={"Run complete analysis with current settings"}>
                            <Button 
                                variant="contained" 
                                onClick={this.run_analysis}
                                disabled={is_loading}
                                color="primary"
                            >
                                {is_loading ? "Running..." : "Run Analysis"}
                            </Button>
                        </Tooltip>
                        <Tooltip title={"Load results from cached data"}>
                            <Button 
                                onClick={this.load_existing_data}
                                disabled={is_loading}
                                variant="outlined"
                            >
                                Load Cached Data
                            </Button>
                        </Tooltip>
                        <Tooltip title={"Fetches all missing stock data (legacy)"}>
                            <Button onClick={() => {
                                this.fetch_all_data(true)
                            }} disabled={is_loading}>
                                Fetch Missing (Legacy)
                            </Button>
                        </Tooltip>
                        <Tooltip title={"EVERYTHING - requests all historical data"}>
                            <Button 
                                onClick={() => {
                                    request_database()
                                }}
                                disabled={is_loading}
                                color="warning"
                            >
                                Request All Historical
                            </Button>
                        </Tooltip>
                        <Button 
                            onClick={this.toggle_searching_options}
                            variant={show_searching_options ? "contained" : "outlined"}
                        >
                            {show_searching_options ? "Hide" : "Show"} Analysis Options
                        </Button>
                        <div style={{ flex: 1, paddingRight: "1rem" }}>
                            <Button onClick={clear_cache} style={{ float: "right" }} disabled={is_loading}>
                                Clear Cache
                            </Button>
                        </div>
                    </Stack>
                </div>
                {/* <PredictionPopup >
                    <h1>Click here to open popup</h1>
                </PredictionPopup> */}

                {show_searching_options && <BackGroundPaper style={{ padding: "1rem", minHeight: "4rem", flex: 1 }}>
                    <Stack spacing={2} direction={"row"}>
                        <Button 
                            onClick={this.load_existing_data}
                            disabled={is_loading}
                            variant="contained"
                        >
                            Apply Settings
                        </Button>
                        <div>
                            <Typography id="track-false-slider" gutterBottom>
                                Market Cap Range
                            </Typography>
                            <MarketCapSlider callback={(left, right) => {
                                this.setState({ searching_options: { ...searching_options, min_market_cap: left, max_market_cap: right } })
                            }} />
                        </div>
                        {/* <CurrencyTextField variant="standard" label="Min Market Cap" value={searching_options.min_market_cap} on_change={(value) => {
                            console.log(value)
                            if (isNaN(value) || value < 0) return;
                            this.setState({ searching_options: { ...searching_options, min_market_cap: Number(value) } })
                        }} />
                        <CurrencyTextField variant="standard" label="Max Market Cap" value={searching_options.max_market_cap} on_change={(value) => {
                            if (isNaN(value) || value < 0) return;
                            this.setState({ searching_options: { ...searching_options, max_market_cap: Number(value) } })
                        }} /> */}
                        <TextField variant="standard" label="Tickers Shown" value={searching_options.tickers_shown} onChange={(e) => {
                            const value = e.target.value;
                            if (isNaN(value) || value < 0) return;
                            this.setState({ searching_options: { ...searching_options, tickers_shown: Number(value) } })
                        }} />


                        <FormControl>
                            <InputLabel id="sort-select-label">Sort by</InputLabel>
                            <Select
                                labelId="sort-select-label"
                                id="sort-select"
                                value={searching_options.sort_by}
                                label="Sort by"
                                onChange={(e) => {
                                    this.setState({ searching_options: { ...searching_options, sort_by: e.target.value } })
                                }}
                            >
                                <MenuItem value={"target_percent_difference"}>Price Target Upside %</MenuItem>
                                <MenuItem value={"market_cap"}>Market Cap</MenuItem>
                                <MenuItem value={"pe_ratio"}>PE Ratio</MenuItem>
                                <MenuItem value={"forward_pe_ratio"}>Forward PE Ratio</MenuItem>
                                <MenuItem value={"divided_yield"}>Dividend Yield</MenuItem>
                                <MenuItem value={"percent_change"}>Daily % Change</MenuItem>
                                <MenuItem value={"net_change"}>Daily $ Change</MenuItem>
                            </Select>
                        </FormControl>

                        <FormControl>
                            <label style={{ fontSize: "14px", color: "#666" }}>
                                <Checkbox 
                                    checked={searching_options.reverse} 
                                    onChange={() => {
                                        this.setState({ searching_options: { ...searching_options, reverse: !searching_options.reverse } })
                                    }} 
                                />
                                {searching_options.sort_by === "target_percent_difference" ? "Lowest first" : "Reverse order"}
                            </label>
                        </FormControl>

                        <CustomSectorNamePopup ref={this.save_popup_ref} onSubmit={(user_input) => {
                            if (user_input && user_input !== "")
                                save_dynamic_sector(user_input, searching_options)
                        }} />

                        <div style={{ flex: 1 }}>
                            <TableDownloadPopup downloadable_stores={downloadable_stores}>
                                <Button style={{ float: "right" }}>
                                    Download Data
                                </Button>
                            </TableDownloadPopup>
                        </div>
                    </Stack>
                </BackGroundPaper>}
                <div className="widgets-container" data-tauri-drag-region style={{ height: "auto", flex: 7 }}>
                    {filtered_tickers.length === 0 && !is_loading ? (
                        <div style={{ textAlign: "center", padding: "2rem", color: "#666" }}>
                            <Typography variant="h6">No stocks found</Typography>
                            <Typography variant="body2" style={{ marginTop: "1rem" }}>
                                Try adjusting your search criteria or running a fresh analysis to fetch new data.
                            </Typography>
                        </div>
                    ) : (
                        <div>
                            {filtered_tickers.length > 0 && (
                                <div style={{ marginBottom: "1rem", padding: "1rem", backgroundColor: "#f5f5f5", borderRadius: "8px" }}>
                                    <Typography variant="h6">Analysis Results ({filtered_tickers.length} stocks)</Typography>
                                    <Typography variant="body2" style={{ marginTop: "0.5rem" }}>
                                        Sorted by: <strong>{searching_options.sort_by.replace(/_/g, ' ').toUpperCase()}</strong>
                                        {searching_options.sort_by === "target_percent_difference" && (
                                            <span> (shows potential upside based on analyst price targets)</span>
                                        )}
                                    </Typography>
                                </div>
                            )}
                            {filtered_tickers.map((data) => {
                                return <StockWidget symbol={data.symbol} size={widgetSize} key={data.symbol} />
                            })}
                        </div>
                    )}
                </div>
            </div>
        </ThemeProvider>
    }
}