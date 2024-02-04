import { invoke } from "@tauri-apps/api";
import { BackendResponse } from "../models/view-models";
import { Question, Questionnaire } from "../models/questionnaire";

export async function parsePdf(filePath: string, name: string): Promise<BackendResponse<string>> {
    return await invoke<BackendResponse<string>>("upload_pdf", { uploadedFilePath: filePath, name: name });
}

export async function createQuestionnaire(questions: Question[], name: string): Promise<BackendResponse<string>> {
    return await invoke<BackendResponse<string>>("create_questionnaire", { questions: questions, name: name });
}

export async function getQuestionnaires(): Promise<BackendResponse<Questionnaire[]>> {
    return await invoke<BackendResponse<Questionnaire[]>>("get_questionnaires");
}

export async function getQuestionnaire(id: number): Promise<BackendResponse<Questionnaire>> {
    return await invoke<BackendResponse<Questionnaire>>("get_questionnaire", { id: id });
}

export async function updateQuestionnaireName(id: number, newName: string): Promise<BackendResponse<string>> {
    return await invoke<BackendResponse<string>>("update_questionnaire_name", { id: id, newName: newName });
}

export async function touchQuestionnaire(id: number): Promise<BackendResponse<string>> {
    return await invoke<BackendResponse<string>>("touch_questionnaire", { id: id });
}

export async function deleteQuestionnaire(id: number): Promise<BackendResponse<string>> {
    return await invoke<BackendResponse<string>>("delete_questionnaire", { id: id });
}