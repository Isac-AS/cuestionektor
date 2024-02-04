use log::error;
use native_db::Database;

use crate::models::{
    questionnaire::Questionnaire,
    view_models::{BackendResponse, OperationResult},
};

#[tauri::command]
pub fn update_questionnaire_name(
    id: u64,
    new_name: &str,
    db: tauri::State<Database>,
) -> BackendResponse<String> {
    let rw = match db.rw_transaction() {
        Ok(rw_t) => rw_t,
        Err(err) => {
            error!("Failed to create rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    };

    let questionnaire: Questionnaire = match rw.get().primary(id) {
        Ok(Some(q)) => q,
        Ok(None) | Err(_) => {
            error!("Failed to read questionnaire.");
            return BackendResponse::new(
                OperationResult::Fail,
                String::from("Failed to read questionnaire"),
            );
        }
    };

    let mut updated_questionnaire = Questionnaire::new_from(&questionnaire);
    updated_questionnaire.name = new_name.to_string();

    match rw.update(questionnaire, updated_questionnaire) {
        Ok(()) => {}
        Err(err) => {
            error!("Failed to update questionnaire.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }

    match rw.commit() {
        Ok(()) => BackendResponse::new(
            OperationResult::Success,
            String::from("Questionnaire updated successfully"),
        ),
        Err(err) => {
            error!("Failed to commit rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }
}

#[tauri::command]
pub fn touch_questionnaire(id: u64, db: tauri::State<Database>) -> BackendResponse<String> {
    let rw = match db.rw_transaction() {
        Ok(rw_t) => rw_t,
        Err(err) => {
            error!("Failed to create rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    };

    let questionnaire: Questionnaire = match rw.get().primary(id) {
        Ok(Some(q)) => q,
        Ok(None) | Err(_) => {
            error!("Failed to read questionnaire.");
            return BackendResponse::new(
                OperationResult::Fail,
                String::from("Failed to read questionnaire"),
            );
        }
    };

    let updated_questionnaire = Questionnaire::new_from(&questionnaire);

    match rw.update(questionnaire, updated_questionnaire) {
        Ok(()) => {}
        Err(err) => {
            error!("Failed to update questionnaire.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }

    match rw.commit() {
        Ok(()) => BackendResponse::new(
            OperationResult::Success,
            String::from("Questionnaire update successfully"),
        ),
        Err(err) => {
            error!("Failed to commit rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }
}
