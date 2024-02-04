use log::error;

use crate::{
    json_db::JsonDB, models::{
        registered::RegisteredQuestionnaire,
        view_models::{BackendResponse, OperationResult},
    }, REGISTERED_COLLECTION, REGISTERED_DOCUMENT
};

#[tauri::command]
pub fn read_registered() -> BackendResponse<Vec<RegisteredQuestionnaire>> {
    let db = JsonDB::init("db").unwrap();
    // Read current registered questionnaire list
    let registered_questionnaires: Vec<RegisteredQuestionnaire> =
        match db.read(REGISTERED_COLLECTION, REGISTERED_DOCUMENT) {
            Ok(registered_questionnaires) => registered_questionnaires,
            Err(err) => {
                error!(
                    "Could not read registered document from database.\nError: {}",
                    err
                );
                return BackendResponse::new(OperationResult::Fail, vec![]);
            }
        };
    BackendResponse::new(OperationResult::Success, registered_questionnaires)
}

pub fn read_document_name(name: &str) -> Option<String> {
    let db = JsonDB::init("db").unwrap();
    let registered_questionnaires: Vec<RegisteredQuestionnaire> =
        match db.read(REGISTERED_COLLECTION, REGISTERED_DOCUMENT) {
            Ok(registered_questionnaires) => registered_questionnaires,
            Err(err) => {
                error!(
                    "Could not read registered document from database.\nError: {}",
                    err
                );
                return None;
            }
        };
    
    match registered_questionnaires.iter().find(|q| q.name == name) {
        Some(registered) => Some(registered.document_name.clone()),
        None => None
    }
}