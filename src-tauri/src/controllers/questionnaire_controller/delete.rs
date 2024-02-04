use log::error;
use native_db::Database;

use crate::{
    controllers::question_controller::read::get_questions,
    models::{
        questionnaire::{Question, Questionnaire},
        view_models::{BackendResponse, OperationResult},
    },
};

#[tauri::command]
pub fn delete_questionnaire(id: i32, db: tauri::State<Database>) -> BackendResponse<String> {
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

    let questions_retrieve = get_questions(questionnaire.id, db.clone());
    let questions = match questions_retrieve.result {
        OperationResult::Fail => {
            return BackendResponse::new(
                OperationResult::Fail,
                "Failed to retrieve questions".to_string(),
            )
        }
        OperationResult::Success => questions_retrieve.data,
    };

    questions.iter().for_each(|q| match rw.remove(Question::new_from(questionnaire.id, q)) {
        Ok(()) => {}
        Err(err) => {
            error!(
                "Failed to remove question: '{}'.\nError: '{}'",
                q.question_number, err
            );
        }
    });

    match rw.remove(questionnaire) {
        Ok(()) => {}
        Err(err) => {
            error!("Failed to remove questionnaire.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }

    match rw.commit() {
        Ok(()) => BackendResponse::new(
            OperationResult::Success,
            String::from("Questionnaire deleted successfully"),
        ),
        Err(err) => {
            error!("Failed to commit rw transaction.\nError: '{}'", err);
            return BackendResponse::new(OperationResult::Fail, err.to_string());
        }
    }
}
