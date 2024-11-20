use reqwest::Client;
use std::error::Error;
use std::time::{Duration, UNIX_EPOCH};
use time::OffsetDateTime;
use tauri::command;
use yahoo_finance_api::{self as yahoo, time::macros::datetime};

/**
 * This function sends a GET request to the provided URL and returns the response text.
 */
async fn get_request(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(url)
        // Set headers to mimic a browser request
        .header("User-Agent", "PostmanRuntime/7.39.0")
        .header("Accept", "*/*")
        .header("Connection", "keep-alive")
        .send()
        .await?;
    let response_text = response.text().await?;
    Ok(response_text)
}
#[tauri::command]
pub async fn yahoo_testing() {
    let provider = yahoo::YahooConnector::new().unwrap();
    let start = datetime!(2020-1-1 0:00:00.00 UTC);
    let end = datetime!(2020-1-31 23:59:59.99 UTC);
    let response = match provider.get_latest_quotes("AMZN", "1d").await {
        Ok(res) => res,
        Err(e) => panic!("{}", e),
    };
    let quote = response.last_quote().unwrap();
    let time: OffsetDateTime =
        OffsetDateTime::from(UNIX_EPOCH + Duration::from_secs(quote.timestamp));
    println!("At {} quote price of Apple was {}", time, quote.close);
}

async fn reddit_request(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(url)
        // Set headers to mimic a browser request
        .header("User-Agent", "PostmanRuntime/7.39.0")
        .header("Accept", "*/*")
        .header("Connection", "keep-alive")
        .header("Authorization", "bearer eyJhbGciOiJSUzI1NiIsImtpZCI6IlNIQTI1NjpzS3dsMnlsV0VtMjVmcXhwTU40cWY4MXE2OWFFdWFyMnpLMUdhVGxjdWNZIiwidHlwIjoiSldUIn0.eyJzdWIiOiJ1c2VyIiwiZXhwIjoxNzI5NjQ5ODcxLjk2NjE4OCwiaWF0IjoxNzI5NTYzNDcxLjk2NjE4NywianRpIjoiUEFaTGtQVUp3emlSQXhwNlYxZmx4ZXJEU2daU0RBIiwiY2lkIjoiZDB1ZzN6UGdKNWdJZV9FNlpVTE9tUSIsImxpZCI6InQyXzE5NnZyd21pb3giLCJhaWQiOiJ0Ml8xOTZ2cndtaW94IiwibGNhIjoxNzI2ODQ0OTk0NDMzLCJzY3AiOiJlSnlLVnRKU2lnVUVBQURfX3dOekFTYyIsImZsbyI6OX0.B-c7B1ISgSlsTuCzcaNockK3Sr48MNFxL-eCGkTyNead_R0FCswp6tH5Sub7rwq15SK2A0KGf_cim8j-NFk9cL-unTUa5nM8PbfQJbeGCEgDiTua0S_lHtSN4fIhUvbkc5ftJCmBVVHgI3walX9EJx2UeiNOIf-kCAvolTZlZr6BEiKeTEwr3hfZ6lDhN1Lt6Ho0n5UVRSUfrR0Oc7Si4sr9UwMwy8150bHEzhbEiGXnbIgEucE0UiYym8fF1kvBqHrIU4xQloF3M8XJSYu3l45BTHEaLBvhEMTlDXOnDaXcTQAO0eqOOIEaODMn8gERnoo21iHQpgPb1ivx4SL6fw")
        .send()
        .await?;
    let response_text = response.text().await?;
    Ok(response_text)
}
/**
 * Get request that is open to the javascript to call
 */
#[command]
pub async fn get_request_api(url: String) -> Result<String, String> {
    match get_request(&url).await {
        Ok(body) => Ok(body),
        Err(e) => Err(format!("Failed to send GET request: {}", e)),
    }
}

#[command]
pub async fn reddit_request_api(url: String) -> Result<String, String> {
    match reddit_request(&url).await {
        Ok(body) => Ok(body),
        Err(e) => Err(format!("Failed to send GET request: {}", e)),
    }
}

#[command]
pub async fn req_nasdaq_info() -> String {
    let url = "https://api.nasdaq.com/api/screener/stocks?tableonly=true&offset=0&download=true";
    match get_request(url).await {
        Ok(response_text) => response_text,
        Err(e) => format!("Error: {}", e),
    }
}

#[command]
pub async fn get_all_static_ticker_info() -> String {
    let url = "https://www.slickcharts.com/sp500";
    match get_request(url).await {
        Ok(response_text) => response_text,
        Err(e) => format!("Error: {}", e),
    }
}

#[command]
pub async fn request_deep() -> String {
    let url = "https://api.nasdaq.com/api/quote/AAPL/chart?assetclass=stocks&fromdate=2000-08-16&todate=2024-08-16";
    match get_request(url).await {
        Ok(response_text) => response_text,
        Err(e) => format!("Error: {}", e),
    }
}
/*
Good to know apis:
https://api.nasdaq.com/api/quote/AAPL/summary?assetclass=stocks
news letters:
https://www.nasdaq.com/api/news/topic/articlebysymbol?q=AAPL|STOCKS&offset=0&limit=10&fallback=true
company profile: https://api.nasdaq.com/api/company/AAPL/company-profile
All companies today on the nasdaq: https://api.nasdaq.com/api/screener/stocks?tableonly=true&offset=0&download=true
 */
