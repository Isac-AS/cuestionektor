import { invoke } from "@tauri-apps/api";
import { OperationResult, OperationResultStruct } from "../models";

function informAboutResult(result: OperationResult, sucess_message: string, error_message: string): boolean {
    let returnValue = result === OperationResult.Success;
    returnValue ? alert(sucess_message) : alert(error_message);
    return returnValue;
}

export async function parsePdf(filePath: string, name: string): Promise<boolean> {
    let uploadResult = await invoke<OperationResultStruct<String>>("upload_pdf", { uploadedFilePath: filePath, name: name});
    return informAboutResult(
        uploadResult.result,
        "Documento procesado correctamente.", 
        "Error al procesar el pdf.",
    );
}

