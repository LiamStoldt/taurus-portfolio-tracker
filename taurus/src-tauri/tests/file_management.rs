// crates
use taurus::file_management;
use std::str::FromStr;



#[test]
fn write_buy_sell_to_csv() {
    let trade_1 = file_management::BuySellRecord {
        ticker: "DEMO".to_string(),
        quantity: 100,
        buy_date: chrono::NaiveDate::from_str("2024-01-01").unwrap(),
        buy_price: 10.0,
        buy_brokerage: 10.0,
        sell_date: None,
        sell_price: None,
        sell_brokerage: None
    };
    let trade_2 = file_management::BuySellRecord {
        ticker: "DEMO".to_string(),
        quantity: 100,
        buy_date: chrono::NaiveDate::from_str("2024-01-01").unwrap(),
        buy_price: 10.0,
        buy_brokerage: 10.0,
        sell_date: Some(chrono::NaiveDate::from_str("2024-02-01").unwrap()),
        sell_price: Some(12.0),
        sell_brokerage: Some(10.0)
    };
    let _res = trade_1.write_buy_sell_record_to_file("demo".to_string());
    let _res = trade_2.write_buy_sell_record_to_file("demo".to_string());
}

#[test]
fn check_trade_values() {
    let trade = file_management::BuySellRecord {
        ticker: "DEMO".to_string(),
        quantity: 100,
        buy_date: chrono::NaiveDate::from_str("2024-01-01").unwrap(),
        buy_price: 10.0,
        buy_brokerage: 10.0,
        sell_date: Some(chrono::NaiveDate::from_str("2024-02-01").unwrap()),
        sell_price: Some(12.0),
        sell_brokerage: Some(10.0)
    };
    assert_eq!(trade.compute_buy_price(), 1000.0);
    assert_eq!(trade.compute_sell_price(), Ok(1200.0));
    assert_eq!(trade.compute_profit_loss(), Ok(180.0));
    assert_eq!(trade.compute_held_days(), 31);
}

#[test]
fn check_file_deserialisation() {
    let trade = file_management::BuySellRecord {
        ticker: "DEMO".to_string(),
        quantity: 100,
        buy_date: chrono::NaiveDate::from_str("2024-01-01").unwrap(),
        buy_price: 10.0,
        buy_brokerage: 10.0,
        sell_date: Some(chrono::NaiveDate::from_str("2024-02-01").unwrap()),
        sell_price: Some(12.0),
        sell_brokerage: Some(10.0)
    };
    let _res = trade.write_buy_sell_record_to_file("demo".to_string());
    let _res = trade.write_buy_sell_record_to_file("demo".to_string());
    let _res = trade.write_buy_sell_record_to_file("demo".to_string());

    let res: Vec<file_management::BuySellRecord> = file_management::get_buy_sell_records("demo".to_string());
    assert!(res.len() > 2);
}
