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
pub struct Question {
    pub question_number: u32,
    pub heading: String,
    pub answers: Vec<Answer>,
    pub topic: String,
    pub explanation: String,
}

impl Question {
    pub fn new(
        question_number: u32,
        heading: String,
        answers: Vec<Answer>,
        topic: String,
        explanation: String,
    ) -> Question {
        Question {
            question_number,
            heading,
            answers,
            topic,
            explanation,
        }
    }

    pub fn new_empty() -> Question {
        Question {
            question_number: 0,
            heading: "".to_string(),
            answers: Vec::new(),
            topic: "".to_string(),
            explanation: "".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Questionnaire {
    pub questions: Vec<Question>,
    pub name: String,
}

impl Questionnaire {
    pub fn new(questions: Vec<Question>, name: String) -> Questionnaire {
        Questionnaire { questions, name }
    }
}
