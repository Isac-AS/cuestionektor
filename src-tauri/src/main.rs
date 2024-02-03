// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use log::LevelFilter;
mod models;
mod controllers;
mod test;
mod json_db;
mod pdf_parser;

use controllers::registered_controller::create::upload_pdf;
use json_db::JsonDB;

fn main() {
    let _ = simple_logging::log_to_file("cuestionektor.log", LevelFilter::Trace);
    
    let db = JsonDB::init("db").unwrap();
    db.init_collections().unwrap();

    // tauri::Builder::default()
    //     .plugin(tauri_plugin_persisted_scope::init())
    //     .invoke_handler(tauri::generate_handler![
    //         upload_pdf,
    //     ])
    //     .run(tauri::generate_context!())
    //     .expect("error while running tauri application");
}
