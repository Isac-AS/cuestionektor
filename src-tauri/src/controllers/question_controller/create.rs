use log::error;
use native_db::Database;

use crate::models::{
    questionnaire::Question,
    view_models::{BackendResponse, OperationResult},
};

#[tauri::command]
pub fn create_empty_question(
    questionnaire_id: i32,
    db: tauri::State<Database>,
) -> BackendResponse<String> {
    let rw = match db.rw_transaction() {
        Ok(rw_t) => rw_t,
        Err(err) => {
            error!("Failed to create rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    };

    let new_question = Question::new_empty(questionnaire_id);

    match rw.insert(new_question) {
        Ok(()) => {}
        Err(err) => {
            error!("Failed to create question.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }

    match rw.commit() {
        Ok(()) => BackendResponse::new(
            OperationResult::Success,
            String::from("Empty question created successfully"),
        ),
        Err(err) => {
            error!("Failed to commit rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }
}
