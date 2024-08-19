use reqwest::Client;
use std::error::Error;
use tauri::command;

/**
 * This function sends a GET request to the provided URL and returns the response text.
 * 
 */
async fn get_request(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(url)
        // Set headers to mimic a browser request
        .header(
            "User-Agent",
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:99.0) Gecko/20100101 Firefox/99.0",
        )
        // .header(
        //     "Accept",
        //     "text/html,application/json,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        // )
        .header("Connection", "keep-alive")
        .send()
        .await?;
    let response_text = response.text().await?;
    Ok(response_text)
}

/**
 * This function sends a GET request to the provided URL and returns the response text.
 * made specially for json requests
 */
async fn json_get_request(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(url)
        // Set headers to mimic a browser request
        .header(
            "User-Agent",
            "PostmanRuntime/7.39.0"
        )
        .header(
            "Accept",
            "application/json",
        )
        .header("Connection", "keep-alive")
        .send()
        .await?;
    let response_text = response.text().await?;
    Ok(response_text)
}

#[command]
pub async fn req_nasdaq_info() -> String {
    let url = "https://api.nasdaq.com/api/screener/stocks?tableonly=true&offset=0&download=true";
    let info = match json_get_request(url).await {
        Ok(response_text) => response_text,
        Err(e) => format!("Error: {}", e),
    };
    return info;
}

#[command]
pub async fn get_index_info() -> String {
    let url = "https://www.slickcharts.com/sp500";
    return match get_request(url).await {
        Ok(response_text) => response_text,
        Err(e) => format!("Error: {}", e),
    };
}



#[command]
pub async fn request_deep() -> String {
    let url = "https://api.nasdaq.com/api/quote/AAPL/chart?assetclass=stocks&fromdate=2000-08-16&todate=2024-08-16";
    return match get_request(url).await {
        Ok(response_text) => response_text,
        Err(e) => format!("Error: {}", e),
    };
}
    /*
    Good to know apis:
    https://api.nasdaq.com/api/quote/AAPL/summary?assetclass=stocks
    news letters: 
    https://www.nasdaq.com/api/news/topic/articlebysymbol?q=AAPL|STOCKS&offset=0&limit=10&fallback=true
    company profile: https://api.nasdaq.com/api/company/AAPL/company-profile
    All companies today on the nasdaq: https://api.nasdaq.com/api/screener/stocks?tableonly=true&offset=0&download=true
     */