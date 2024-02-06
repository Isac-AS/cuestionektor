export interface Answer {
    prefix: string,
    text: string,
    is_correct: boolean
}

export interface Question {
    id: number,
    questionnaire_id: number,
    question_number: number,
    heading: string,
    answers: Answer[],
    topic: string,
    explanation: string,
    answeredCorrectly?: boolean,
}

export interface Questionnaire {
    id: number,
    name: string,
    last_accessed: string,
}

export enum AnswerState {
    Correct,
    Incorrect,
    Unanswered,
    All
}