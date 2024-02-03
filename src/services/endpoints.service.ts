import { invoke } from "@tauri-apps/api";
import { OperationResultStruct, RegisteredQuestionnaire } from "../models";

export async function parsePdf(filePath: string, name: string): Promise<OperationResultStruct<String>> {
    return await invoke<OperationResultStruct<String>>("upload_pdf", { uploadedFilePath: filePath, name: name });
}

export async function updateRegisteredQuestionnaires(updatedQuestionnaires: RegisteredQuestionnaire[]): Promise<OperationResultStruct<String>> {
    return await invoke<OperationResultStruct<String>>("update_registered_questionnaires", { updatedQuestionnaires: updatedQuestionnaires });
}

export async function deleteQuestionnaire(updatedQuestionnaires: RegisteredQuestionnaire[], filePath: String): Promise<OperationResultStruct<String>> {
    return await invoke<OperationResultStruct<String>>("delete_questionnaire", { 
        updatedQuestionnaires: updatedQuestionnaires,
        filePath: filePath 
    });
}
