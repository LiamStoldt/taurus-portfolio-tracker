// imports
use serde::{Deserialize, Serialize};
use std::fs;
use csv;

// buy-sell struct
#[derive(Debug, Deserialize, Serialize)]
pub struct BuySellRecord {
    pub ticker: String,
    pub quantity: u32,
    pub buy_date: chrono::NaiveDate,
    pub buy_price: f32,
    pub buy_brokerage: f32,
    pub sell_date: Option<chrono::NaiveDate>,
    pub sell_price: Option<f32>,
    pub sell_brokerage: Option<f32>
}

impl BuySellRecord {
    pub fn compute_buy_price(&self) -> f32 {
        (self.quantity as f32) * self.buy_price
    }

    pub fn compute_sell_price(&self) -> Result<f32, String> {
        match self.sell_price {
            Some(sell_price) => Ok((self.quantity as f32) * sell_price),
            None => Err("No sell price data".to_string())
        }
    }

    pub fn compute_profit_loss(&self) -> Result<f32, String> {
        // check brokerage
        let mut sell_brokerage = 0.0;
        if let Some(val) = self.sell_brokerage {
            sell_brokerage = val;
        }

        // get prices
        let sell_total = self.compute_sell_price();
        let buy_total = self.compute_buy_price();

        // check sell total, compute p/l
        match sell_total {
            Ok(sell_val) => Ok(sell_val - buy_total - sell_brokerage - self.buy_brokerage),
            Err(e) => Err(e)
        }
    }

    pub fn compute_held_days(&self) -> i64 {
        let mut latest_held_date = chrono::offset::Local::now()
            .date_naive();
        if let Some(sell_date) = self.sell_date {
            latest_held_date = sell_date;
        }
        (latest_held_date - self.buy_date).num_days()
    }

    pub fn write_buy_sell_record_to_file(&self, portfolio_name: String) -> Result<(), std::io::Error> {
        let csv_location = format!("./data/portfolios/{}/buy_sell.csv", portfolio_name);
        let file = fs::OpenOptions::new()
            .write(true)
            .create(true)
            .append(true)
            .open(csv_location)
            .unwrap();
        let mut wrt = csv::WriterBuilder::new()
            .has_headers(false)
            .from_writer(file);
        wrt.serialize(self)?;
        wrt.flush()?;
        Ok(())
    }
}


// directory management
#[tauri::command]
pub fn create_new_portfolio(portfolio_name: String) -> Result<(), std::io::Error> {
    fs::create_dir(format!("./data/portfolios/{}", portfolio_name))
}


// get buy sell data
#[tauri::command]
pub fn get_buy_sell_records(portfolio_name: String) -> Vec<BuySellRecord> {
    let csv_location = format!("./data/portfolios/{}/buy_sell.csv", portfolio_name);
    let file = fs::OpenOptions::new()
        .read(true)
        .open(csv_location)
        .unwrap();
    let mut reader = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(file);
    let res: Vec<BuySellRecord> = reader.deserialize()
        .into_iter()
        .map(|x| x.unwrap())
        .collect();
    res
}
