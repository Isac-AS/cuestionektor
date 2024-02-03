use log::error;

use crate::{
    json_db::JsonDB,
    models::{
        parsing_utils::ParsingFilters,
        questionnaire::Questionnaire,
        registered::RegisteredQuestionnaire,
        view_models::{BackendResponse, OperationResult},
    },
    pdf_parser,
};

#[tauri::command]
pub fn upload_pdf(uploaded_file_path: &str, name: &str) -> BackendResponse<String> {
    // Extract questions from pdf
    let questions = pdf_parser::parse_pdf(uploaded_file_path.to_string(), ParsingFilters::new());

    // Create the elements that will be saved
    let new_questionnaire = Questionnaire::new(questions, name.to_string());
    let new_registered_questionnaire = RegisteredQuestionnaire::new(name.to_string());

    // Read current registered questionnaire list
    let db = JsonDB::init("db").unwrap();
    let mut registered_questionnaires: Vec<RegisteredQuestionnaire> =
        match db.read("registered", "registered") {
            Ok(registered_questionnaires) => registered_questionnaires,
            Err(err) => {
                error!(
                    "Could not read registered document from database.\nError: {}",
                    err
                );
                return BackendResponse::new(
                    OperationResult::Fail,
                    String::from("Error reading questionnaires"),
                );
            }
        };

    // Add new questionnaire
    registered_questionnaires.push(new_registered_questionnaire);

    // Write to registered questionnaires file
    let db = JsonDB::init("db").unwrap();
    let db = match db.write("registered", "registered", registered_questionnaires) {
        Ok(database) => database,
        Err(err) => {
            error!("Could not write the updated registered questionnaires to db.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    };

    // Write new questionnaire
    match db.write("questionnaires", name, new_questionnaire) {
        Ok(_) => BackendResponse::new(
            OperationResult::Success,
            String::from("Questionnaire created sucessfully"),
        ),
        Err(err) => {
            error!("Could not write questionnaire to db.\nError: {}", err);
            BackendResponse::new(OperationResult::Fail, err.to_string())
        }
    }
}
