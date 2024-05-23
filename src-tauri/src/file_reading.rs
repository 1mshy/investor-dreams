use std::io::{BufRead, BufReader};
use tauri::{command, State};
use crate::csv_data::{TICKER_MAP};

#[derive(Clone)]
pub(crate) struct TickerSymbolData {
    full_name: String,
    exchange: String,
}

impl TickerSymbolData {
    pub fn new(full_name: &str, exchange: &str) -> TickerSymbolData {
        TickerSymbolData {
            full_name: full_name.to_string(),
            exchange: exchange.to_string(),
        }
    }
}

// #[command]
pub fn get_company_info(ticker_symbol: &str) -> Result<TickerSymbolData, String> {

    match TICKER_MAP.get(ticker_symbol) {
        Some(ticker) => Ok(ticker.clone()), // Correctly clone the data to match the expected type
        None => Err(format!("Ticker symbol {} not found", ticker_symbol))
    }
}

#[command]
pub fn get_company_name(ticker_symbol: &str) -> String {
    let symbol_data = get_company_info(ticker_symbol);
    return match symbol_data {
        Ok(data) => data.full_name,
        Err(_) => String::new()
    };
}

#[command]
pub fn get_company_exchange(ticker_symbol: &str) -> String {
    let symbol_data = get_company_info(ticker_symbol);
    return match symbol_data {
        Ok(data) => data.exchange,
        Err(_) => String::new()
    };
}