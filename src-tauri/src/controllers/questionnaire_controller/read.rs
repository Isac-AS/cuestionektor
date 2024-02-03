use log::error;

use crate::{
    json_db::JsonDB,
    models::{
        questionnaire::Questionnaire,
        view_models::{BackendResponse, OperationResult},
    },
};

#[tauri::command]
fn read_questionnaire(name: &str) -> BackendResponse<Questionnaire> {
    let db = JsonDB::init("db").unwrap();
    match db.read("questionnaires", name) {
        Ok(questionnaire) => BackendResponse::new(OperationResult::Success, questionnaire),
        Err(err) => {
            error!(
                "Could not read questionnaire with name: '{}'\nError: '{}'",
                name, err
            );
            BackendResponse::new(
                OperationResult::Fail,
                Questionnaire::new(vec![], "none".to_string()),
            )
        }
    }
}
