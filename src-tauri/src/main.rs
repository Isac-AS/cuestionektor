// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;

use log::{debug, error, info, LevelFilter};
use model::{
    OperationResult, OperationResultStruct, PdfParsingFilters, Questionnaire,
    RegisteredQuestionnaire, RegisteredQuestionnaires,
};
mod json_handler;
mod model;
mod pdf_parser;

const QUESTIONNAIRE_DIRECTORY: &str = "./questionnaires/";
const REGISTERED_QUESTIONNAIRES_FILE_PATH: &str = "./questionnaires/registered_questionnaires.json";

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn upload_pdf(uploaded_file_path: &str, name: &str) -> OperationResultStruct<String> {
    debug!("Entered pdf upload func. Proceeding to parse pdf");
    // Extract questions from pdf
    let questions = pdf_parser::parse_pdf(uploaded_file_path.to_string(), PdfParsingFilters::new());
    debug!("Pdf parsed successfully.");
    // Declare some later used variables
    let questionnaire_name: String = String::from(name);
    let questionnaire_file_path: String = format!("{}{}.json", QUESTIONNAIRE_DIRECTORY, name);

    // Create the elements that will be saved
    let new_questionnaire = Questionnaire::new(
        questions,
        questionnaire_name.clone(),
        questionnaire_file_path.clone(),
    );
    let new_registered_questionnaire = RegisteredQuestionnaire::new(
        questionnaire_name,
        questionnaire_file_path.clone(),
        Some(String::from(uploaded_file_path)),
    );

    debug!("Reading current registered questionnaire list.");
    // Read current registered questionnaire list
    let mut registered_questionnaires = match json_handler::read_json::<RegisteredQuestionnaires>(
        REGISTERED_QUESTIONNAIRES_FILE_PATH.to_string(),
    ) {
        Some(registered_questionnaire) => registered_questionnaire,
        None => {
            return OperationResultStruct::new(
                OperationResult::Fail,
                Some(String::from("Error reading questionnaires")),
            )
        }
    };

    debug!("Current registered questionnaire list read sucessfully.");
    // Add new questionnaire
    registered_questionnaires
        .questionnaires
        .push(new_registered_questionnaire);

    debug!("Writing new questionnaire to registered questionnaire file.");
    // Write to registered questionnaires file
    match json_handler::write_to_json(
        registered_questionnaires,
        REGISTERED_QUESTIONNAIRES_FILE_PATH.to_string(),
    ) {
        Ok(()) => {}
        Err(err) => {
            return OperationResultStruct::new(OperationResult::Fail, Some(err.to_string()))
        }
    }
    debug!("Added new questionnaire to registered questionnaires successfully.");

    debug!("Writing parsed questionnaire to file.");
    // Write new questionnaire
    match json_handler::write_to_json(new_questionnaire, questionnaire_file_path) {
        Ok(()) => OperationResultStruct::new(
            OperationResult::Success,
            Some(String::from("Questionnaire created sucessfully")),
        ),
        Err(err) => OperationResultStruct::new(OperationResult::Fail, Some(err.to_string())),
    }
}

#[tauri::command]
fn get_registered_questionnaires() -> OperationResultStruct<RegisteredQuestionnaires> {
    match json_handler::read_json(REGISTERED_QUESTIONNAIRES_FILE_PATH.to_string()) {
        Some(registered_questionnaire) => {
            OperationResultStruct::new(OperationResult::Success, Some(registered_questionnaire))
        }
        None => OperationResultStruct::new(OperationResult::Fail, None),
    }
}

#[tauri::command]
fn update_registered_questionnaires(
    updated_questionnaires: Vec<RegisteredQuestionnaire>,
) -> OperationResultStruct<String> {
    // Read current registered questionnaire list
    let mut registered_questionnaires = match json_handler::read_json::<RegisteredQuestionnaires>(
        REGISTERED_QUESTIONNAIRES_FILE_PATH.to_string(),
    ) {
        Some(registered_questionnaire) => registered_questionnaire,
        None => {
            return OperationResultStruct::new(
                OperationResult::Fail,
                Some(String::from("Error reading questionnaires")),
            )
        }
    };
    println!("\n\nUpdated questionnaires:\n{:?}\n\nRegistered questionnaires:\n{:?}", updated_questionnaires, registered_questionnaires);
    registered_questionnaires.questionnaires = updated_questionnaires;
    println!("\nUpdated registered questionnaires:\n{:?}", registered_questionnaires);
    match json_handler::write_to_json(
        registered_questionnaires,
        REGISTERED_QUESTIONNAIRES_FILE_PATH.to_string(),
    ) {
        Ok(()) => OperationResultStruct::new(OperationResult::Success, Some(String::from("Saved questionnaires successfully"))),
        Err(err) => {
            return OperationResultStruct::new(OperationResult::Fail, Some(err.to_string()))
        }
    }
}

#[tauri::command]
fn delete_questionnaire(
    updated_questionnaires: Vec<RegisteredQuestionnaire>,
    file_path: &str
) -> OperationResultStruct<String> {
    // Read current registered questionnaire list
    let mut registered_questionnaires = match json_handler::read_json::<RegisteredQuestionnaires>(
        REGISTERED_QUESTIONNAIRES_FILE_PATH.to_string(),
    ) {
        Some(registered_questionnaire) => registered_questionnaire,
        None => {
            return OperationResultStruct::new(
                OperationResult::Fail,
                Some(String::from("Error reading questionnaires")),
            )
        }
    };
    registered_questionnaires.questionnaires = updated_questionnaires;
    print!("{:?}", registered_questionnaires);

    let _ = match json_handler::write_to_json(
        registered_questionnaires,
        REGISTERED_QUESTIONNAIRES_FILE_PATH.to_string(),
    ) {
        Ok(()) => info!("Saved questionnaire list successfully proceding to delete."),
        Err(err) => {
            return OperationResultStruct::new(OperationResult::Fail, Some(err.to_string()))
        }
    };

    debug!("About to remove: {}", file_path);
    match fs::remove_file(file_path) {
        Ok(_) => {
            debug!("File: '{}' deleted successfully!", file_path);
            OperationResultStruct::new(OperationResult::Success, Some(String::from("Removed questionnaire successfully")))
        },
        Err(err) => {
            error!("Could not delete questionnaire file: {}", err);
            return OperationResultStruct::new(OperationResult::Fail, Some(err.to_string()))
        }
    }

}

#[tauri::command]
fn read_questionnaire(file_path: &str) -> OperationResultStruct<Questionnaire> {
    match json_handler::read_json(file_path.to_string()) {
        Some(questionnaire) => {
            OperationResultStruct::new(OperationResult::Success, Some(questionnaire))
        }
        None => OperationResultStruct::new(OperationResult::Fail, None),
    }
}

#[tauri::command]
fn save_questionnaire(questionnaire: Questionnaire) -> OperationResultStruct<String> {
    match json_handler::write_to_json(&questionnaire, questionnaire.file_path.clone()) {
        Ok(()) => OperationResultStruct::new(
            OperationResult::Success,
            Some(String::from("Questionnaire saved successfully")),
        ),
        Err(err) => OperationResultStruct::new(OperationResult::Fail, Some(err.to_string())),
    }
}

fn main() {
    let _ = simple_logging::log_to_file("cuestionektor.log", LevelFilter::Trace);
    json_handler::check_json_dir(
        &QUESTIONNAIRE_DIRECTORY,
        &REGISTERED_QUESTIONNAIRES_FILE_PATH,
    );
    tauri::Builder::default()
        .plugin(tauri_plugin_persisted_scope::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            upload_pdf,
            get_registered_questionnaires,
            update_registered_questionnaires,
            delete_questionnaire,
            read_questionnaire,
            save_questionnaire
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
