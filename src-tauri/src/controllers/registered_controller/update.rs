use log::error;

use crate::{json_db::JsonDB, models::{registered::RegisteredQuestionnaire, view_models::{BackendResponse, OperationResult}}};


#[tauri::command]
fn update_registered_questionnaires(
    updated_questionnaires: Vec<RegisteredQuestionnaire>,
) -> BackendResponse<String> {
    let db = JsonDB::init("db").unwrap();
    
    match db.write("registered", "registered", updated_questionnaires) {
        Ok(_) => BackendResponse::new(OperationResult::Success, String::from("Questionnaires updated sucessfully.")),
        Err(err) => {
        error!("Could not write the updated registered questionnaires to db.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }
}