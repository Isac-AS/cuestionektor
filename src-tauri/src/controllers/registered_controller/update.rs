use log::error;

use crate::{json_db::JsonDB, models::{registered::RegisteredQuestionnaire, view_models::{BackendResponse, OperationResult}}, REGISTERED_COLLECTION, REGISTERED_DOCUMENT};


#[tauri::command]
pub fn update_registered(
    updated_questionnaires: Vec<RegisteredQuestionnaire>,
) -> BackendResponse<String> {
    let db = JsonDB::init("db").unwrap();
    
    match db.write(REGISTERED_COLLECTION, REGISTERED_DOCUMENT, updated_questionnaires) {
        Ok(_) => BackendResponse::new(OperationResult::Success, String::from("Questionnaires updated sucessfully.")),
        Err(err) => {
        error!("Could not write the updated registered questionnaires to db.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }
}