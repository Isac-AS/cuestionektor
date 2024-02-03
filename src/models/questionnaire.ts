export interface Answer {
    prefix: String,
    text: String,
    is_correct: boolean
}

export interface Question {
    question_number: number,
    heading: String,
    answers: Answer[],
    topic: String,
    explanation: String
}

export interface Questionnaire {
    questions: Question[],
    name: String,
}