use log::error;
use native_db::Database;

use crate::models::{
    questionnaire::Question,
    view_models::{BackendResponse, OperationResult},
};

#[tauri::command]
pub fn update_question(
    id: i32,
    question_to_update: Question,
    db: tauri::State<Database>,
) -> BackendResponse<String> {
    let rw = match db.rw_transaction() {
        Ok(rw_t) => rw_t,
        Err(err) => {
            error!("Failed to create rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    };

    let original_question: Question = match rw.get().primary(id) {
        Ok(Some(q)) => q,
        Ok(None) => {
            error!("Failed to read question. get().primary() succeeded but got None.");
            return BackendResponse::new(
                OperationResult::Fail,
                String::from("Failed to read question. Got None."),
            );
        }
        Err(err) => {
            error!("Failed to read question.\nError: {}", err);
            return BackendResponse::new(
                OperationResult::Fail,
                String::from("Failed to read question. Database error."),
            );
        }
    };

    let updated_question =
        Question::new_from(original_question.questionnaire_id, &question_to_update);

    match rw.update(original_question, updated_question) {
        Ok(()) => {}
        Err(err) => {
            error!("Failed to update question.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }

    match rw.commit() {
        Ok(()) => BackendResponse::new(
            OperationResult::Success,
            String::from("Question updated successfully"),
        ),
        Err(err) => {
            error!("Failed to commit rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }
}
