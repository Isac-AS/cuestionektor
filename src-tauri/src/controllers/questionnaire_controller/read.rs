use log::error;
use native_db::Database;

use crate::models::{
    questionnaire::Questionnaire,
    view_models::{BackendResponse, OperationResult},
};

#[tauri::command]
pub fn get_questionnaires(db: tauri::State<Database>) -> BackendResponse<Vec<Questionnaire>> {
    let r = match db.r_transaction() {
        Ok(read_transaction) => read_transaction,
        Err(err) => {
            error!("Failed to create ro transaction.\nError: {}", err);
            return BackendResponse::new(OperationResult::Fail, vec![]);
        }
    };

    let registered_questionnaires = match r.scan().primary() {
        Ok(questionnaires) => questionnaires.all().collect(),
        Err(err) => {
            error!("Failed to read questionnaires.\nError: {}", err);
            return BackendResponse::new(OperationResult::Fail, vec![]);
        }
    };
    BackendResponse::new(OperationResult::Success, registered_questionnaires)
}

#[tauri::command]
pub fn get_questionnaire(id: u64, db: tauri::State<Database>) -> BackendResponse<Questionnaire> {
    let r = match db.r_transaction() {
        Ok(read_transaction) => read_transaction,
        Err(err) => {
            error!("Failed to create ro transaction.\nError: {}", err);
            return BackendResponse::new(OperationResult::Fail, Questionnaire::new_empty());
        }
    };

    let questionnaire = match r.get().primary(id) {
        Ok(Some(q)) => q,
        Ok(None) => return BackendResponse::new(OperationResult::Fail, Questionnaire::new_empty()),
        Err(err) => {
            error!("Failed to read questionnaires.\nError: {}", err);
            return BackendResponse::new(OperationResult::Fail, Questionnaire::new_empty());
        }
    };
    BackendResponse::new(OperationResult::Success, questionnaire)
}
