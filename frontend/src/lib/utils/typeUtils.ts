export class AssertionError extends Error {
    failureName?: string;

    constructor(message: string, failureName?: string) {
        super(message);
        this.name = "ApiError";
        this.failureName = failureName;
        Object.setPrototypeOf(this, AssertionError.prototype);
    }
}

export function assertNotNull<T>(
    value: T | null | undefined,
    errorMessage?: string,
    failureName?: string
): T {
    if (!value) {
        throw new AssertionError(
            errorMessage ?? "value was null.",
            failureName
        );
    }
    return value;
}
