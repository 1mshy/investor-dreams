type YahooStockData = {
    chart: {
        result: Array<{
            meta: MetaData; // Define the structure of "meta" if needed
            timestamp: number[]; // Array of timestamps
            events?: Events; // Define more detailed structure if available
            indicators: {
                quote: Array<{
                    volume: number[]; // Array of volumes
                    open: number[]; // Array of opening prices
                    high: number[]; // Array of high prices
                    low: number[]; // Array of low prices
                    close: number[]; // Array of closing prices
                }>;
                adjclose: Array<{
                    adjclose: number[]; // Array of adjusted close prices
                }>;
            };
        }>;
    };
    error: null | object; // Error can be null or an object
};

type TotalStockData = {
    data: HistoricalData;
    meta: MetaData;
    events: Events;
}


type HistoricalData = {
    data: StockPoint[];
}


type StockPoint = {
    datetime: number; // Timestamp in ms
    volume: number; // Volume
    open: number; // Opening price
    high: number; // High price
    low: number; // Low price
    close: number; // Closing price
}


type Events = {
    dividends: {
        [key: string]: {
            amount: number;
            date: number;
        };
    };
    splits: {
        [key: string]: {
            date: number;
            numerator: number;
            denominator: number;
            splitRatio: string;
        };
    };
};
type MetaData = {
    currency: string;
    symbol: string;
    exchangeName: string;
    fullExchangeName: string;
    instrumentType: string;
    firstTradeDate: number;
    regularMarketTime: number;
    hasPrePostMarketData: boolean;
    gmtoffset: number;
    timezone: string;
    exchangeTimezoneName: string;
    regularMarketPrice: number;
    fiftyTwoWeekHigh: number;
    fiftyTwoWeekLow: number;
    regularMarketDayHigh: number;
    regularMarketDayLow: number;
    regularMarketVolume: number;
    longName: string;
    shortName: string;
    chartPreviousClose: number;
    priceHint: number;
    currentTradingPeriod: {
        pre: {
            timezone: string;
            start: number;
            end: number;
            gmtoffset: number;
        };
        regular: {
            timezone: string;
            start: number;
            end: number;
            gmtoffset: number;
        };
        post: {
            timezone: string;
            start: number;
            end: number;
            gmtoffset: number;
        };
    };
    dataGranularity: string;
    range: string;
    validRanges: string[];
};
