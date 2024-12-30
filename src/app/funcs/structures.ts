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

/**
 * Stock point array from oldest to newest
 */
type HistoricalData = StockPoint[];


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

type SubredditData = RedditPost[];

type RedditPost = {
    all_awardings: any[]; // Array with unspecified structure
    allow_live_comments: boolean;
    approved_at_utc: number | null;
    approved_by: string | null;
    archived: boolean;
    author: string;
    author_flair_background_color: string | null;
    author_flair_css_class: string | null;
    author_flair_richtext: Array<{ e: string; t: string }>;
    author_flair_template_id: string | null;
    author_flair_text: string | null;
    author_flair_text_color: string | null;
    author_flair_type: string;
    author_fullname: string;
    author_is_blocked: boolean;
    author_patreon_flair: boolean;
    author_premium: boolean;
    awarders: any[]; // Array with unspecified structure
    banned_at_utc: number | null;
    banned_by: string | null;
    can_gild: boolean;
    can_mod_post: boolean;
    category: string | null;
    clicked: boolean;
    content_categories: string[] | null;
    contest_mode: boolean;
    created: number;
    created_utc: number;
    discussion_type: string | null;
    distinguished: string | null;
    domain: string;
    downs: number;
    edited: boolean | number; // `false` or timestamp
    gilded: number;
    gildings: Record<string, number>; // Object with key-value pairs
    hidden: boolean;
    hide_score: boolean;
    id: string;
    is_created_from_ads_ui: boolean;
    is_crosspostable: boolean;
    is_meta: boolean;
    is_original_content: boolean;
    is_reddit_media_domain: boolean;
    is_robot_indexable: boolean;
    is_self: boolean;
    is_video: boolean;
    likes: boolean | null;
    link_flair_background_color: string | null;
    link_flair_css_class: string | null;
    link_flair_richtext: Array<{ e: string; t: string }>;
    link_flair_template_id: string | null;
    link_flair_text: string | null;
    link_flair_text_color: string | null;
    link_flair_type: string;
    locked: boolean;
    media: any | null; // Media object with unspecified structure
    media_embed: Record<string, any>; // Embed details as key-value pairs
    media_only: boolean;
    mod_note: string | null;
    mod_reason_by: string | null;
    mod_reason_title: string | null;
    mod_reports: any[];
    name: string;
    no_follow: boolean;
    num_comments: number;
    num_crossposts: number;
    num_reports: number | null;
    over_18: boolean;
    permalink: string;
    pinned: boolean;
    post_hint: string | null;
    preview: {
        enabled: boolean;
        images: Array<{
            source: { url: string; width: number; height: number };
            resolutions: Array<{ url: string; width: number; height: number }>;
            variants: Record<string, any>;
            id: string;
        }>;
    } | null;
    pwls: number | null;
    quarantine: boolean;
    removal_reason: string | null;
    removed_by: string | null;
    removed_by_category: string | null;
    report_reasons: string[] | null;
    saved: boolean;
    score: number;
    secure_media: any | null;
    secure_media_embed: Record<string, any>;
    selftext: string;
    selftext_html: string | null;
    send_replies: boolean;
    spoiler: boolean;
    stickied: boolean;
    subreddit: string;
    subreddit_id: string;
    subreddit_name_prefixed: string;
    subreddit_subscribers: number;
    subreddit_type: string;
    suggested_sort: string | null;
    thumbnail: string | null;
    thumbnail_height: number | null;
    thumbnail_width: number | null;
    title: string;
    top_awarded_type: string | null;
    total_awards_received: number;
    treatment_tags: string[];
    ups: number;
    upvote_ratio: number;
    url: string;
    url_overridden_by_dest: string | null;
    user_reports: any[];
    view_count: number | null;
    visited: boolean;
    wls: number | null;
};


type GraphDataset = {
    label: string;
    data: number[];
    backgroundColor: string;
    borderColor: string;
    fill: boolean;
    pointRadius: number;
    pointHoverRadius: number;
    pointHitRadius: number;
    pointBackgroundColor: string;
    pointBorderColor: string;
    pointHoverBackgroundColor: string;
    pointHoverBorderColor: string;
    lineTension: number;
    borderWidth: number;
    hidden: boolean;
}