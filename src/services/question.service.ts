import { invoke } from "@tauri-apps/api";
import { BackendResponse } from "../models/view-models";
import { Question } from "../models/questionnaire";

export async function getQuestions(questionnaireId: number): Promise<BackendResponse<Question[]>> {
    return await invoke<BackendResponse<Question[]>>("get_questions", { questionnaireId: questionnaireId });
}

export async function updateQuestion(id: number, question: Question): Promise<BackendResponse<string>> {
    return await invoke<BackendResponse<string>>("update_question", { id: id, questionToUpdate: question });
}

export async function deleteQuestion(id: number): Promise<BackendResponse<string>> {
    return await invoke<BackendResponse<string>>("delete_question", { id: id });
}