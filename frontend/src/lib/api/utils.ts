import { goto } from "$app/navigation";
import { PUBLIC_BACKEND_URL } from "$env/static/public";
import { ApiError, assertOk } from "./errors";

export function getFullImageUrl(path: string): string {
    return PUBLIC_BACKEND_URL + path;
}

// Includes credentials and asserts response.
export async function autofetch(
    input: RequestInfo | URL,
    init?: RequestInit
): Promise<Response> {
    if (!init) {
        init = {};
    }
    init.credentials = "include";

    const resp = await fetch(input, init);
    assertOk(resp);

    return resp;
}
