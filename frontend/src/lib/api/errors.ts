export class ApiError extends Error {
    statusCode: number;

    constructor(message: string, statusCode: number) {
        super(message);
        this.name = "ApiError";
        this.statusCode = statusCode;
        Object.setPrototypeOf(this, ApiError.prototype);
    }
}

export function assertOk(resp: Response) {
    if (resp.ok) return;

    if (resp.status === 401) {
        throw new ApiError(`Request is not authorized`, resp.status);
    }
    throw new ApiError(
        `Response was not OK. Response was:\n${resp.statusText}`,
        resp.status
    );
}

export function isErrorUnauthorized(error: ApiError): boolean {
    return error.statusCode === 401;
}
