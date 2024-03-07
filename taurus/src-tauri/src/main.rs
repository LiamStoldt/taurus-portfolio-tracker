// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// local modules
mod file_management;
mod finance_computation;
mod yfinance_interactions;

// crates
use std::fs;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // check data directories
    let check_dir = fs::create_dir("./data/");
    match check_dir {
        Ok(()) => (),
        Err(_e) => ()
    }
    let check_dir = fs::create_dir("./data/portfolios/");
    match check_dir {
        Ok(()) => (),
        Err(_e) => ()
    }

    // build tauri
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            file_management::create_new_portfolio,
            file_management::get_buy_sell_records
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
