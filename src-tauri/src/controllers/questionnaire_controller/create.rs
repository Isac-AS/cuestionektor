use log::error;
use native_db::Database;

use crate::{
    models::{
        parsing_utils::ParsingFilters,
        questionnaire::{Question, Questionnaire},
        view_models::{BackendResponse, OperationResult},
    },
    pdf_parser,
};

#[tauri::command(async)]
pub fn upload_pdf(
    uploaded_file_path: &str,
    name: &str,
    db: tauri::State<Database>,
) -> BackendResponse<String> {
    let questionnaire_id: i32 = rand::random();
    // Extract questions from pdf
    let questions = pdf_parser::parse_pdf(
        uploaded_file_path.to_string(),
        ParsingFilters::new(),
        questionnaire_id,
    );
    create_questionnaire(questions, name, db)
}

#[tauri::command]
pub fn create_questionnaire(
    questions: Vec<Question>,
    name: &str,
    db: tauri::State<Database>,
) -> BackendResponse<String> {
    let questionnaire = Questionnaire::new(name.to_string());

    let rw = match db.rw_transaction() {
        Ok(rw_t) => rw_t,
        Err(err) => {
            error!("Failed to create rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    };

    questions.iter().for_each(|question| {
        match rw.insert(Question::new_from(questionnaire.id, question)) {
            Ok(_) => {}
            Err(err) => {
                error!(
                    "Failed to insert question {}.\nError: '{}'",
                    question.question_number, err
                );
            }
        }
    });

    match rw.insert(questionnaire) {
        Ok(_) => {}
        Err(err) => {
            error!("Failed to insert questionnaire.\nError: '{}'", err);
        }
    };

    match rw.commit() {
        Ok(()) => BackendResponse::new(
            OperationResult::Success,
            "Questionnaire saved successfully".to_string(),
        ),
        Err(err) => {
            error!("Failed to commit rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }
}

#[tauri::command]
pub fn create_empty_questionnaire(
    name: &str,
    db: tauri::State<Database>,
) -> BackendResponse<String> {
    let rw = match db.rw_transaction() {
        Ok(rw_t) => rw_t,
        Err(err) => {
            error!("Failed to create rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    };

    let new_questionnaire = Questionnaire::new(name.to_string());
    let new_questionnaire_id = new_questionnaire.id.to_string();

    match rw.insert(new_questionnaire) {
        Ok(()) => {}
        Err(err) => {
            error!("Failed to create questionnaire.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }

    match rw.commit() {
        Ok(()) => BackendResponse::new(
            OperationResult::Success,
            new_questionnaire_id,
        ),
        Err(err) => {
            error!("Failed to commit rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }
}
