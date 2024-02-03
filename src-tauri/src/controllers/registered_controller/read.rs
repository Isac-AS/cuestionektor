use log::error;

use crate::{
    json_db::JsonDB,
    models::{
        registered::RegisteredQuestionnaire,
        view_models::{BackendResponse, OperationResult},
    },
};

#[tauri::command]
fn get_registered_questionnaires() -> BackendResponse<Vec<RegisteredQuestionnaire>> {
    let db = JsonDB::init("db").unwrap();
    // Read current registered questionnaire list
    let registered_questionnaires: Vec<RegisteredQuestionnaire> =
        match db.read("registered", "registered") {
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
