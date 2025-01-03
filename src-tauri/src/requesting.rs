use reqwest::Client;
use serde_json::Value;
use std::{collections::HashMap, error::Error};
use tauri::command;

use crate::sensitive_constants::{CLIENT_ID, CLIENT_SECRET, REDDIT_API_PASSWORD, REDDIT_API_USERNAME};

#[tauri::command]
pub async fn fetch_reddit_access_token() -> Result<String, String> {
    let client = Client::new();
    let token_url = "https://www.reddit.com/api/v1/access_token";

    let mut params = HashMap::new();
    params.insert("grant_type", "password");
    params.insert("username", REDDIT_API_USERNAME);
    params.insert("password", REDDIT_API_PASSWORD);

    let response = client
        .post(token_url)
        .basic_auth(CLIENT_ID, Some(CLIENT_SECRET))
        .form(&params)
        .header("User-Agent", "PostmanRuntime/7.39.0")
        .header("Accept", "*/*")
        .send()
        .await
        .map_err(|e| format!("Failed to request access token: {}", e))?;

    let token_data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse token response: {}", e))?;

    println!("{}", token_data);

    // let access_token = token_data["access_token"]
    //     .as_str()
    //     .ok_or("Access token not found in response")?;

    Ok(token_data.to_string())
}

#[tauri::command]
pub async fn fetch_reddit_subreddit_posts(
    access_token: &str,
    subreddit: &str,
) -> Result<Value, String> {
    let client = Client::new();
    let subreddit_url = format!("https://oauth.reddit.com/r/{}/new", subreddit);

    let response = client
        .get(&subreddit_url)
        .bearer_auth(access_token)
        .header("User-Agent", "PostmanRuntime/7.39.0")
        .header("Accept", "*/*")
        .send()
        .await
        .map_err(|e| format!("Failed to fetch posts: {}", e))?;

    let posts_data: Value = response
        .json()
        .await
        .map_err(|e| format!("Failed to parse posts response: {}", e))?;

    Ok(posts_data)
}

/**
 * This function sends a GET request to the provided URL and returns the response text.
 */
async fn get_request(url: &str) -> Result<String, Box<dyn Error>> {
    let client = Client::new();
    let response = client
        .get(url)
        // Set headers to mimic a browser request
        .header("User-Agent", "PostmanRuntime/7.39.0")
        .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3;q=0.9")
        .header("Connection", "keep-alive")
        .send()
        .await?;
    let response_text = response.text().await?;
    Ok(response_text)
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
