use log::error;

use crate::{
    json_db::JsonDB,
    models::{
        registered::RegisteredQuestionnaire,
        view_models::{BackendResponse, OperationResult},
    },
};

#[tauri::command]
fn delete_questionnaire(name: &str) -> BackendResponse<String> {
    let db = JsonDB::init("db").unwrap();
    // Read current registered questionnaire list
    let mut registered_questionnaires: Vec<RegisteredQuestionnaire> =
        match db.read("registered", "registered") {
            Ok(registered_questionnaires) => registered_questionnaires,
            Err(err) => {
                error!(
                    "Could not read registered document from database.\nError: {}",
                    err
                );
                return BackendResponse::new(
                    OperationResult::Fail,
                    String::from("Error reading questionnaires"),
                );
            }
        };

    registered_questionnaires.retain(|registered_q| registered_q.name != name.to_string());

    let db = JsonDB::init("db").unwrap();
    let db = match db.write("registered", "registered", registered_questionnaires) {
        Ok(database) => database,
        Err(err) => {
            error!("Could not write the updated registered questionnaires to db.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    };

    match db.delete("questionnaires", name) {
        Ok(_) => BackendResponse::new(
            OperationResult::Success,
            String::from("Removed questionnaire successfully"),
        ),
        Err(err) => {
            error!("Could not delete questionnaire file: {}", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }
}
