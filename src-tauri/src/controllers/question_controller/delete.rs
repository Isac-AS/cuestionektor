use log::error;
use native_db::Database;

use crate::models::{
    questionnaire::Question,
    view_models::{BackendResponse, OperationResult},
};

#[tauri::command]
pub fn delete_question(id: i32, db: tauri::State<Database>) -> BackendResponse<String> {
    let rw = match db.rw_transaction() {
        Ok(rw_t) => rw_t,
        Err(err) => {
            error!("Failed to create rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    };

    let question: Question = match rw.get().primary(id) {
        Ok(Some(q)) => q,
        Ok(None) | Err(_) => {
            error!("Failed to read question.");
            return BackendResponse::new(
                OperationResult::Fail,
                String::from("Failed to read questionnaire"),
            );
        }
    };
    
    match rw.remove(question) {
        Ok(()) => {}
        Err(err) => {
            error!("Failed to remove question.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }

    match rw.commit() {
        Ok(()) => BackendResponse::new(
            OperationResult::Success,
            String::from("Question deleted successfully"),
        ),
        Err(err) => {
            error!("Failed to commit rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }
}
