import { invoke } from "@tauri-apps/api";
import { OperationResultStruct } from "../models";

export async function parsePdf(filePath: string, name: string): Promise<OperationResultStruct<String>> {
    return await invoke<OperationResultStruct<String>>("upload_pdf", { uploadedFilePath: filePath, name: name});
}

