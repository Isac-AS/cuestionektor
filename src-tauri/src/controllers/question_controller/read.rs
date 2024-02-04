use log::error;
use native_db::Database;

use crate::models::{
    questionnaire::Question,
    view_models::{BackendResponse, OperationResult},
};

#[tauri::command]
pub fn get_questions(
    questionnaire_id: i32,
    db: tauri::State<Database>,
) -> BackendResponse<Vec<Question>> {
    let r = match db.r_transaction() {
        Ok(read_transaction) => read_transaction,
        Err(err) => {
            error!("Failed to create ro transaction.\nError: {}", err);
            return BackendResponse::new(OperationResult::Fail, vec![]);
        }
    };
    let questions: Vec<Question> = match r.scan().primary() {
        Ok(qs) => qs.all().collect(),
        Err(err) => {
            error!("Failed to read questions.\nError: {}", err);
            return BackendResponse::new(OperationResult::Fail, vec![]);
        }
    };

    let questions = questions
        .into_iter()
        .filter(|q| q.questionnaire_id != questionnaire_id)
        .collect();
    BackendResponse::new(OperationResult::Success, questions)
}
