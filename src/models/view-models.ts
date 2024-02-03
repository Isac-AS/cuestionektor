export enum OperationResult {
    Success,
    Fail,
}

export interface BackendResponse<T> {
    result: OperationResult,
    data: T,
}

