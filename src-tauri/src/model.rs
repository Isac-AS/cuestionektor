use serde::{Deserialize, Serialize};
use tauri::regex::Regex;
use tauri::regex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Answer {
    pub prefix: String,
    pub text: String,
    pub is_correct: bool
}

impl Answer {
    pub fn new(prefix: String, text: String, is_correct: bool) -> Answer{
        Answer {
            prefix,
            text,
            is_correct
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Question {
    pub question_number: u32,
    pub heading: String,
    pub answers: Vec<Answer>,
    pub topic: String,
    pub explanation: String
}

impl Question {
    pub fn new(question_number: u32, heading: String, answers: Vec<Answer>, topic: String, explanation: String) -> Question {
        Question {
            question_number,
            heading,
            answers,
            topic,
            explanation
         }
    }

    pub fn new_empty() -> Question {
        Question {
            question_number: 0,
            heading: "".to_string(),
            answers: Vec::new(),
            topic: "".to_string(),
            explanation: "".to_string()
         }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Questionnaire {
    pub questions: Vec<Question>,
    pub name: String,
    pub file_path: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredQuestionnaire {
    pub name: String,
    pub file_path: String,
    pub uploaded_file_path: Option<String>,
    pub was_opened: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisteredQuestionnaires {
    pub questionnaires: Vec<RegisteredQuestionnaire>,
}

impl RegisteredQuestionnaires {
    pub fn new_empty() -> RegisteredQuestionnaires{
        RegisteredQuestionnaires {
            questionnaires: Vec::new()
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum OperationResult {
    Success,
    Fail
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OperationResultStruct<T> {
    pub result: OperationResult,
    pub element: Option<T>
}

impl<T> OperationResultStruct<T> {
    pub fn new(result: OperationResult, element: Option<T>) -> OperationResultStruct<T> {
        OperationResultStruct {
            result,
            element
        }
    }
}

pub struct PdfParsingFilters {
    pub heading_re: Regex,
    pub possible_answer_re: Regex,
    pub answer_re: Regex
}

impl PdfParsingFilters {
    pub fn new() -> PdfParsingFilters {
        PdfParsingFilters {
            heading_re: Regex::new(r"^\d{1,3}\.").unwrap(),
            possible_answer_re: Regex::new(r"^(A|B|C|D)\)").unwrap(),
            answer_re: Regex::new(r"^Respuesta Correcta").unwrap()
        }
    }
    
    pub fn new_with_strings(heading_re: String, possible_answer_re: String, answer_re: String) -> PdfParsingFilters{
        PdfParsingFilters {
            heading_re: Regex::new(format!(r#"{}"#, regex::escape(&heading_re)).as_str()).unwrap(),
            possible_answer_re: Regex::new(format!(r#"{}"#, regex::escape(&possible_answer_re)).as_str()).unwrap(),
            answer_re: Regex::new(format!(r#"{}"#, regex::escape(&answer_re)).as_str()).unwrap()
        }
    }
}