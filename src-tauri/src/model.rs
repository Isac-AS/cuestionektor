pub struct Answer {
    prefix: String,
    text: String,
    is_correct: bool
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

pub struct Question {
    question_number: u32,
    heading: String,
    answers: Vec<Answer>,
    topic: String,
    explanation: String
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
}