
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
    uploaded_file_path: String,
    last_opened: String
}

export interface RegisteredQuestionnaires {
    questionnaires: Array<RegisteredQuestionnaire>,
}

export enum OperationResult {
    Success = 'Success',
    Fail = 'Fail'
}

export interface OperationResultStruct<T> {
    result: OperationResult,
    element: T
}
