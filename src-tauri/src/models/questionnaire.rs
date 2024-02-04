use chrono::{DateTime, Local};
use native_db::{native_db, InnerKeyValue};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub prefix: String,
    pub text: String,
    pub is_correct: bool,
}

impl Answer {
    pub fn new(prefix: String, text: String, is_correct: bool) -> Answer {
        Answer {
            prefix,
            text,
            is_correct,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[native_model(id = 2, version = 1)]
#[native_db]
pub struct Question {
    #[primary_key]
    pub id: i32,
    pub questionnaire_id: i32,
    pub question_number: u32,
    pub heading: String,
    pub answers: Vec<Answer>,
    pub topic: String,
    pub explanation: String,
}

impl Question {
    pub fn new_from(questionnaire_id: i32, question: &Question) -> Question {
        Question {
            id: rand::random(),
            questionnaire_id,
            question_number: question.question_number,
            heading: question.heading.clone(),
            answers: question.answers.clone(),
            topic: question.topic.clone(),
            explanation: question.explanation.clone(),
        }
    }

    pub fn new_empty(questionnaire_id: i32) -> Question {
        Question {
            id: rand::random(),
            questionnaire_id,
            question_number: 0,
            heading: "".to_string(),
            answers: Vec::new(),
            topic: "".to_string(),
            explanation: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct Questionnaire {
    #[primary_key]
    pub id: i32,
    pub name: String,
    pub last_accessed: String,
}

impl Questionnaire {
    pub fn new(name: String) -> Questionnaire {
        let current_local: DateTime<Local> = Local::now();
        let formatted_date_local = current_local.format("%Y-%m-%d %H:%M:%S").to_string();
        Questionnaire {
            id: rand::random(),
            name,
            last_accessed: formatted_date_local,
        }
    }

    pub fn new_from(questionnaire: &Questionnaire) -> Questionnaire {
        let current_local: DateTime<Local> = Local::now();
        let formatted_date_local = current_local.format("%Y-%m-%d %H:%M:%S").to_string();
        Questionnaire {
            id: questionnaire.id,
            name: questionnaire.name.clone(),
            last_accessed: formatted_date_local,
        }
    }

    pub fn new_empty() -> Questionnaire {
        Questionnaire {
            id: 0,
            name: "str".to_string(),
            last_accessed: "0".to_string(),
        }
    }
}
