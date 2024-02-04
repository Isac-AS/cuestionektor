export enum OperationResult {
    Success = "Success",
    Fail = "Fail",
}

export interface BackendResponse<T> {
    result: OperationResult,
    data: T,
}

