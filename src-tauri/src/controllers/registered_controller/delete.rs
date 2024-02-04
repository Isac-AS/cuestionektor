use log::error;

use crate::{
    json_db::JsonDB,
    models::{
        registered::RegisteredQuestionnaire,
        view_models::{BackendResponse, OperationResult},
    },
    QUESTIONNAIRE_COLLECTION, REGISTERED_COLLECTION, REGISTERED_DOCUMENT,
};

use super::read::read_document_name;

#[tauri::command]
pub fn delete_questionnaire(name: &str) -> BackendResponse<String> {
    let questionnaire_document_name = match read_document_name(name) {
        Some(document_name) => document_name,
        None => {
            error!(
                "Could not extract document name from registered questionnaire: '{}'",
                name
            );
            return BackendResponse::new(
                OperationResult::Fail,
                String::from("Could not extract document name."),
            );
        }
    };

    match delete_registered(name) {
        Ok(_) => {}
        Err(err) => {
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }

    let db = JsonDB::init("db").unwrap();
    match db.delete(QUESTIONNAIRE_COLLECTION, &questionnaire_document_name) {
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

pub fn delete_registered(name: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Read current registered questionnaire list
    let mut registered_questionnaires: Vec<RegisteredQuestionnaire> =
        match JsonDB::init("db")?.read(REGISTERED_COLLECTION, REGISTERED_DOCUMENT) {
            Ok(registered_questionnaires) => registered_questionnaires,
            Err(err) => {
                error!(
                    "Could not read registered document from database.\nError: {}",
                    err
                );
                return Err(err);
            }
        };

    registered_questionnaires.retain(|registered_q| registered_q.name != name.to_string());

    match JsonDB::init("db")?.write(
        REGISTERED_COLLECTION,
        REGISTERED_DOCUMENT,
        registered_questionnaires,
    ) {
        Ok(database) => database,
        Err(err) => {
            error!(
                "Could not write the updated registered questionnaires to db.\nError: '{}'",
                err
            );
            return Err(err);
        }
    };
    Ok(())
}
