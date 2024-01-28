// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use model::{OperationResultStruct, OperationResult, PdfParsingFilters, Questionnaire, RegisteredQuestionnaire, RegisteredQuestionnaires};
mod pdf_parser;
mod json_handler;
mod model;

const QUESTIONNAIRE_DIRECTORY: &str = "./questionnaires/";
const REGISTERED_QUESTIONNAIRES_FILE_PATH: &str = "./questionnaires/registered_questionnaires.json";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn upload_pdf(uploaded_file_path: &str, name: &str) -> OperationResultStruct<String> {
    // Extract questions from pdf
    let questions = pdf_parser::parse_pdf(uploaded_file_path.to_string(), PdfParsingFilters::new());
    // Declare some later used variables
    let questionnaire_name: String = String::from(name);
    let questionnaire_file_path: String = format!("{}{}.json", QUESTIONNAIRE_DIRECTORY, name);

    // Create the elements that will be saved
    let new_questionnaire = Questionnaire::new(questions, questionnaire_name.clone(), questionnaire_file_path.clone());
    let new_registered_questionnaire = RegisteredQuestionnaire::new(
        questionnaire_name, 
        questionnaire_file_path.clone(),
        Some(String::from(uploaded_file_path)),
        false);
    
    // Read current registered questionnaire list
    let mut registered_questionnaires = match json_handler::read_json::<RegisteredQuestionnaires>(REGISTERED_QUESTIONNAIRES_FILE_PATH.to_string()) {
        Some(registered_questionnaire) => registered_questionnaire,
        None => return OperationResultStruct::new(OperationResult::Fail, Some(String::from("Error reading questionnaires")))
    };

    // Add new questionnaire
    registered_questionnaires.questionnaires.push(new_registered_questionnaire);


    // Write to registered questionnaires file
    match json_handler::write_to_json(registered_questionnaires, REGISTERED_QUESTIONNAIRES_FILE_PATH.to_string()) {
        Ok(()) => {},
        Err(err) => return OperationResultStruct::new(OperationResult::Fail, Some(err.to_string()))
    }
    
    // Write new questionnaire
    match json_handler::write_to_json(new_questionnaire, questionnaire_file_path) {
        Ok(()) => OperationResultStruct::new(OperationResult::Success, Some(String::from("Questionnaire created sucessfully"))),
        Err(err) => OperationResultStruct::new(OperationResult::Fail, Some(err.to_string()))
    }

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
        .plugin(tauri_plugin_persisted_scope::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            upload_pdf,
            get_registered_questionnaires,
            read_questionnaire,
            save_questionnaire
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
