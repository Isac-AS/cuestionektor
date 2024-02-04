// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use once_cell::sync::Lazy;
use log::LevelFilter;
mod models;
mod controllers;
mod pdf_parser;

use controllers::questionnaire_controller::create::{ upload_pdf, create_questionnaire };
use controllers::questionnaire_controller::delete::delete_questionnaire;
use controllers::questionnaire_controller::read::{ get_questionnaire, get_questionnaires };
use controllers::questionnaire_controller::update::{ update_questionnaire_name, touch_questionnaire };
use controllers::question_controller::delete::delete_question;
use controllers::question_controller::read::get_questions;
use controllers::question_controller::update::update_question;
use tauri::Manager;

static DATABASE_BUILDER: Lazy<native_db::DatabaseBuilder> = Lazy::new(|| {
    let mut builder = native_db::DatabaseBuilder::new();
    builder
        .define::<models::questionnaire::Questionnaire>()
        .expect("failed to define model Questionnaire");
    builder
        .define::<models::questionnaire::Question>()
        .expect("failed to define model Question");
    builder
});

fn main() {
    let _ = simple_logging::log_to_file("cuestionektor.log", LevelFilter::Trace);
    
    let db = DATABASE_BUILDER.create("./cuestionektorDB");

    tauri::Builder::default()
        .setup(move |app| {
            let app_handle = app.handle();
            app_handle.manage(db);
            Ok(())
        })
        .plugin(tauri_plugin_persisted_scope::init())
        .invoke_handler(tauri::generate_handler![
            upload_pdf,
            create_questionnaire,
            delete_questionnaire,
            get_questionnaire,
            get_questionnaires,
            update_questionnaire_name,
            touch_questionnaire,
            delete_question,
            get_questions,
            update_question
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
