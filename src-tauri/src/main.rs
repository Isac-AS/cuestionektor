// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use model::{OperationResultStruct, OperationResult, PdfParsingFilters, Questionnaire, RegisteredQuestionnaire, RegisteredQuestionnaires};
mod pdf_parser;
mod json_handler;
mod model;

const QUESTIONNAIRE_DIRECTORY: &str = "./questionnaires/";
const REGISTERED_QUESTIONNAIRES_FILE_PATH: &str = "questionnaires/registered_questionnaires.json";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn get_registered_questionnaires() -> OperationResultStruct<RegisteredQuestionnaires> {
    match json_handler::read_json(REGISTERED_QUESTIONNAIRES_FILE_PATH.to_string()) {
        Some(registered_questionnaire) => OperationResultStruct::new(OperationResult::Success, Some(registered_questionnaire)),
        None => OperationResultStruct::new(OperationResult::Fail, None)
    }
}

#[tauri::command]
fn read_questionnaire(file_path: &str) -> OperationResultStruct<Questionnaire> {
    match json_handler::read_json(file_path.to_string()) {
        Some(questionnaire) => OperationResultStruct::new(OperationResult::Success, Some(questionnaire)),
        None => OperationResultStruct::new(OperationResult::Fail, None)
    }
}

#[tauri::command]
fn save_questionnaire(questionnaire: Questionnaire) -> OperationResultStruct<String> {
    match json_handler::write_to_json(&questionnaire, questionnaire.file_path.clone()) {
        Ok(()) => OperationResultStruct::new(OperationResult::Success, Some(String::from("Questionnaire save successfully"))),
        Err(err) => OperationResultStruct::new(OperationResult::Fail, Some(err.to_string()))
    }
}

fn main() {
    json_handler::check_json_dir(&QUESTIONNAIRE_DIRECTORY, &REGISTERED_QUESTIONNAIRES_FILE_PATH);
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            greet,
            get_registered_questionnaires,
            read_questionnaire,
            save_questionnaire
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
