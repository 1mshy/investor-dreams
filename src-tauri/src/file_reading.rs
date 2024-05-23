use std::fs::File;
use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::path::Path;
use tauri::{command, State};
use crate::TickerInfoMap;

#[derive(Clone)]
pub(crate) struct TickerSymbolData {
    full_name: String,
    exchange: String,
}

impl TickerSymbolData {
    fn new(full_name: &str, exchange: &str) -> TickerSymbolData {
        TickerSymbolData {
            full_name: full_name.to_string(),
            exchange: exchange.to_string(),
        }
    }
}

// #[command]
pub fn get_company_info(ticker_symbol: &str, state: State<'_, TickerInfoMap>) -> Result<TickerSymbolData, String> {
    let map_read_guard = state.read().expect("Failed to read lock the map");

    match map_read_guard.get(ticker_symbol) {
        Some(ticker) => Ok(ticker.clone()), // Correctly clone the data to match the expected type
        None => Err(format!("Ticker symbol {} not found", ticker_symbol))
    }
}
#[command]
pub fn get_company_name(ticker_symbol:&str, state: State<'_, TickerInfoMap>) -> String {
    let symbol_data = get_company_info(ticker_symbol, state);
    return match symbol_data {
        Ok(data) => data.full_name,
        Err(_) => String::new()
    }
}
#[command]
pub fn get_company_exchange(ticker_symbol:&str, state: State<'_, TickerInfoMap>) -> String {
    let symbol_data = get_company_info(ticker_symbol, state);
    return match symbol_data {
        Ok(data) => data.exchange,
        Err(_) => String::new()
    }
}


#[command]
pub fn read_csv(state: State<'_, TickerInfoMap>) {
    let path = Path::new("data/tickers_names_exchanges.csv");
    let file = File::open(path).expect("Unable to open file");
    let reader = BufReader::new(file);

    let mut map: HashMap<String, TickerSymbolData> = HashMap::new();

    for (index, line) in reader.lines().enumerate() {
        let line = line.expect("Unable to read line");

        // Optionally skip the header line
        if index == 0 {
            continue; // Skip the header row
        }

        let split: Vec<&str> = line.split(',').map(|s| s.trim()).collect(); // Trim each field to remove whitespace
        if split.len() >= 3 { // Ensure there are enough fields
            let ticker_data = TickerSymbolData::new(split[1], split[2]);
            map.insert(split[0].to_string(), ticker_data);
        } else {
            eprintln!("Skipping invalid line: {}", line);
        }
    }
    // Replace the global state map with the new one
    let mut state_map = state.write().unwrap();
    *state_map = map;
    // println!("{}", state_map.get("AAPL").unwrap().full_name)
}