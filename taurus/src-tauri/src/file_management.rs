// imports
use serde::{Deserialize, Serialize};
use std::fs;
use chrono;
use csv;

// csv management
#[derive(Debug, Deserialize, Serialize)]
struct BuySellRecord {
    ticker: String,
    quantity: u32,
    buy_date: chrono::NaiveDate,
    buy_price: f32,
    buy_brokerage: f32,
    sell_date: Option<chrono::NaiveDate>,
    sell_price: Option<f32>,
    sell_brokerage: Option<f32>
}

impl BuySellRecord {
    
}

fn create_new_data_files(portfolio_name: String) -> Result<(), std::io::Error> {
    // file names
    let buy_file_loc = format!("./data/portfolios/{}/buy_sell.csv", portfolio_name);
    let divi_file_loc = format!("./data/portfolios/{}/dividends.csv", portfolio_name);
}

// directory management
#[tauri::command]
pub fn create_new_portfolio(portfolio_name: String) -> Result<(), std::io::Error> {
    let res = fs::create_dir(format!("./data/portfolios/{}", portfolio_name));
    return res
}
