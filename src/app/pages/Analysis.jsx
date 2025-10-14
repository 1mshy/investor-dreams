import "@/app/css/Analysis.css";
import "@/app/css/Homepage.css";
import "@/app/css/Playground.css";
import { filter_tickers, filter_tickers_async, request_database, subscribeToBackgroundFetch, getBackgroundFetchState, stopBackgroundFetch, startBackgroundStockFetch, getCacheStatistics } from "@/app/funcs/analysis";
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
import { Button, Checkbox, FormControl, InputLabel, MenuItem, Select, Stack, TextField, ThemeProvider, Tooltip, Typography, LinearProgress, Box, Paper } from '@mui/material';
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
            is_filtering: false, // Only for filtering operation
            // Background fetch state
            background_fetch: {
                is_running: false,
                current_symbol: '',
                progress: 0,
                total: 0,
                status: 'idle',
            },
            // Cache statistics
            cache_stats: {
                total_symbols: 0,
                cached_symbols: 0,
                missing_symbols: 0,
                cache_percentage: 0,
            },
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
        this.start_background_fetch = this.start_background_fetch.bind(this);
        this.stop_background_fetch = this.stop_background_fetch.bind(this);
        this.update_cache_stats = this.update_cache_stats.bind(this);

        this.save_popup_ref = React.createRef(null);
        this.unsubscribe_background = null;
        this.stats_interval = null;
    }

    openPopup() {
        this.save_popup_ref.current.open();
    };

    async componentDidMount() {
        try {
            const all_symbols = await get_all_symbols();
            this.setState({ all_symbols });
            
            // Subscribe to background fetch updates
            this.unsubscribe_background = subscribeToBackgroundFetch((state) => {
                this.setState({ background_fetch: state });
                
                // Auto-refresh results when background fetch completes or updates
                if (!this.state.is_filtering) {
                    this.update_cache_stats();
                }
            });
            
            // Update cache statistics periodically
            this.update_cache_stats();
            this.stats_interval = setInterval(this.update_cache_stats, 5000); // Every 5 seconds
            
            // Auto-load existing data if available
            await this.run_analysis();
        } catch (error) {
            console.error("Error in componentDidMount:", error);
            toast.error("Failed to load initial data");
        }
    }

    componentWillUnmount() {
        // Cleanup subscription and interval
        if (this.unsubscribe_background) {
            this.unsubscribe_background();
        }
        if (this.stats_interval) {
            clearInterval(this.stats_interval);
        }
    }

    async update_cache_stats() {
        try {
            const stats = await getCacheStatistics();
            this.setState({ cache_stats: stats });
        } catch (error) {
            console.error("Error updating cache stats:", error);
        }
    }

    start_background_fetch() {
        startBackgroundStockFetch();
        toast.info("Background data collection started");
    }

    stop_background_fetch() {
        stopBackgroundFetch();
        toast.info("Background data collection stopped");
    }

    /**
     * Run analysis - filters and displays stocks based on current cache
     */
    async run_analysis() {
        this.setState({ is_filtering: true });

        try {
            const { searching_options } = this.state;
            const filtered_results = await filter_tickers_async(searching_options);
            
            this.setState({ 
                filtered_tickers: filtered_results,
                is_filtering: false,
            });

            if (filtered_results.length > 0) {
                toast.success(`Found ${filtered_results.length} matching stocks`);
            } else {
                toast.info("No stocks match your criteria");
            }
        } catch (error) {
            console.error("Error in run_analysis:", error);
            this.setState({ is_filtering: false });
            toast.error("Failed to analyze stocks: " + error.message);
        }
    }

    openPopup() {
        this.save_popup_ref.current.open();
    };

    /**
     * @deprecated - Background fetch handles data collection now
     */
    async fetch_all_data(skip_cached = true) {
        console.warn("fetch_all_data is deprecated, use background fetch instead");
        this.start_background_fetch();
    }

    toggle_searching_options() {
        this.setState({ show_searching_options: !this.state.show_searching_options });
    }

    async sort_all_tickers() {
        // This method now uses the same logic as run_analysis
        await this.run_analysis();
    }
    predict(symbol) {
        this.predictions.setItem("AAPL", 180)
    }

    render() {
        const { all_symbols, search_value, searched_symbols, filtered_tickers, downloadable_stores,
            show_searching_options, searching_options, is_filtering, background_fetch, cache_stats } = this.state;
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
                        
                        {/* Cache Statistics */}
                        <Paper elevation={1} style={{ marginTop: "10px", padding: "10px", backgroundColor: "#f5f5f5" }}>
                            <Typography variant="body2">
                                <strong>Data Cache:</strong> {cache_stats.cached_symbols} / {cache_stats.total_symbols} stocks ({cache_stats.cache_percentage}% complete)
                            </Typography>
                            {cache_stats.missing_symbols > 0 && (
                                <Typography variant="caption" color="textSecondary">
                                    {cache_stats.missing_symbols} stocks still need data
                                </Typography>
                            )}
                        </Paper>

                        <div style={{ marginTop: "10px" }}>
                            {`Matching stocks found: ${filtered_tickers.length}`}
                        </div>

                        {/* Background Fetch Status */}
                        {background_fetch.is_running && (
                            <Paper elevation={2} style={{ marginTop: "15px", padding: "15px", backgroundColor: "#e3f2fd" }}>
                                <Typography variant="h6" gutterBottom>Background Data Collection</Typography>
                                <Typography variant="body2" color="textSecondary">
                                    {background_fetch.status}
                                </Typography>
                                {background_fetch.current_symbol && (
                                    <Typography variant="body2" color="textSecondary">
                                        Processing: <strong>{background_fetch.current_symbol}</strong>
                                    </Typography>
                                )}
                                <Box sx={{ width: '100%', marginTop: 2 }}>
                                    <LinearProgress variant="determinate" value={background_fetch.progress} />
                                    <Typography variant="caption" display="block" align="center" sx={{ marginTop: 1 }}>
                                        {background_fetch.progress}% Complete
                                    </Typography>
                                </Box>
                                <Button 
                                    onClick={this.stop_background_fetch} 
                                    variant="outlined" 
                                    color="error" 
                                    size="small"
                                    style={{ marginTop: "10px" }}
                                >
                                    Stop Background Fetch
                                </Button>
                            </Paper>
                        )}
                    </div>

                    <Stack spacing={2} direction={"row"} style={{ marginTop: "15px" }}>
                        {!background_fetch.is_running ? (
                            <Tooltip title={"Start background data collection (ordered by market cap)"}>
                                <Button 
                                    variant="contained" 
                                    onClick={this.start_background_fetch}
                                    color="primary"
                                >
                                    Start Data Collection
                                </Button>
                            </Tooltip>
                        ) : (
                            <Tooltip title={"Background collection is running"}>
                                <Button 
                                    variant="outlined" 
                                    disabled
                                >
                                    Collecting Data...
                                </Button>
                            </Tooltip>
                        )}
                        
                        <Tooltip title={"Filter and display stocks with current settings"}>
                            <Button 
                                variant="contained" 
                                onClick={this.run_analysis}
                                disabled={is_filtering}
                                color="secondary"
                            >
                                {is_filtering ? "Filtering..." : "Run Analysis"}
                            </Button>
                        </Tooltip>
                        
                        <Tooltip title={"EVERYTHING - requests all historical data (legacy)"}>
                            <Button 
                                onClick={() => {
                                    request_database()
                                }}
                                disabled={background_fetch.is_running}
                                color="warning"
                            >
                                Request All Historical
                            </Button>
                        </Tooltip>
                        
                        <Button 
                            onClick={this.toggle_searching_options}
                            variant={show_searching_options ? "contained" : "outlined"}
                        >
                            {show_searching_options ? "Hide" : "Show"} Filter Options
                        </Button>
                        
                        <div style={{ flex: 1, paddingRight: "1rem" }}>
                            <Button onClick={clear_cache} style={{ float: "right" }} disabled={background_fetch.is_running}>
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
                            onClick={this.run_analysis}
                            disabled={is_filtering}
                            variant="contained"
                        >
                            Apply Filters
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
                    {filtered_tickers.length === 0 && !is_filtering && !background_fetch.is_running ? (
                        <div style={{ textAlign: "center", padding: "2rem", color: "#666" }}>
                            <Typography variant="h6">No stocks found</Typography>
                            <Typography variant="body2" style={{ marginTop: "1rem" }}>
                                {cache_stats.cached_symbols === 0 
                                    ? "Start data collection to begin fetching stock data."
                                    : "Try adjusting your filter criteria or wait for more data to be collected."}
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