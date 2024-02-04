use log::error;

use crate::{
    json_db::JsonDB,
    models::{
        questionnaire::Questionnaire,
        view_models::{BackendResponse, OperationResult},
    },
    QUESTIONNAIRE_COLLECTION,
};

#[tauri::command]
pub fn read_questionnaire(document_name: &str) -> BackendResponse<Questionnaire> {
    let db = JsonDB::init("db").unwrap();
    match db.read(QUESTIONNAIRE_COLLECTION, document_name) {
        Ok(questionnaire) => BackendResponse::new(OperationResult::Success, questionnaire),
        Err(err) => {
            error!(
                "Could not read questionnaire with name: '{}'\nError: '{}'",
                document_name, err
            );
            BackendResponse::new(
                OperationResult::Fail,
                Questionnaire::new(vec![], "none".to_string()),
            )
        }
    }
}
