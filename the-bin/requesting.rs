#[tauri::command]
pub async fn get_yahoo_crumb() -> Result<String, String> {
    // Create persistent client with cookies
    let client = match Client::builder()
        .cookie_store(true)
        .https_only(true)
        .user_agent("Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10_1) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/39.0.2171.95 Safari/537.36")
        .build() {
            Ok(client) => client,
            Err(_) => return Err("Error".to_string())
        };

    // 1. Get initial cookies
    match client.get("https://fc.yahoo.com").send().await {
        Ok(_) => {}
        Err(_) => return Err("Error".to_string()),
    };

    // 2. Get valid crumb with cookies
    let response = match client
        .get("https://query1.finance.yahoo.com/v1/test/getcrumb")
        .send()
        .await
    {
        Ok(response) => response,
        Err(_) => return Err("Error on response".to_string()),
    };

    match response.text().await {
        Ok(crumb) => Ok(crumb),
        Err(_) => Err("Error unwrapping".to_string()),
    }
}