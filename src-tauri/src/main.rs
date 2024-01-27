// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use model::{OperationResultStruct, OperationResult, PdfParsingFilters, Questionnaire, RegisteredQuestionnaire};
mod pdf_parser;
mod json_handler;
mod model;

const REGISTERED_QUESTIONNAIRES_FILE_PATH: &str = "questionnaires/registered_questionnaires.json";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn read_questionnaire(file_path: &str) -> OperationResultStruct<Questionnaire> {
    let read_attempt = json_handler::read_json(file_path.to_string()).unwrap();
    match read_attempt {
        Some(questionnaire) => OperationResultStruct::new(OperationResult::Success, Some(questionnaire)),
        None => OperationResultStruct::new(OperationResult::Fail, None)
    }
}

#[tauri::command]
fn save_questionnaire(questionnaire: Questionnaire) -> OperationResultStruct<String> {
    let save_attempt = json_handler::write_to_json(&questionnaire, questionnaire.file_path.clone());
    match save_attempt {
        Ok(()) => OperationResultStruct::new(OperationResult::Success, Some(String::from("Questionnaire save successfully"))),
        Err(err) => OperationResultStruct::new(OperationResult::Fail, Some(err.to_string()))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![read_questionnaire])
        .invoke_handler(tauri::generate_handler![save_questionnaire])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
