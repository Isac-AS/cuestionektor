
export interface Answer {
    prefix: String,
    text: String,
    is_correct: boolean
}

export interface Question {
    question_number: number,
    heading: String,
    answers: Array<Answer>,
    topic: String,
    explanation: String
}

export interface Questionnaire {
    questions: Array<Question>,
    name: String,
    file_path: String
}

export interface RegisteredQuestionnaire {
    name: String,
    file_path: String,
    uploaded_file_path: Option<String>
}

export enum OperationResult {
    Success,
    Fail
}

export interface OperationResultStruct<T> {
    result: OperationResult,
    element: Option<T>
}

export interface Option<T> {
    Some: T,
    None: null
}