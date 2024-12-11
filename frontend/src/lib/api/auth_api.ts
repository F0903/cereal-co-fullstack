import { PUBLIC_BACKEND_URL } from "$env/static/public";
import type { User } from "$lib/models/User";
import { getCookie } from "$lib/utils/cookie_utils";

export class InvalidCredsError extends Error {}

export async function login(user: User) {
    const resp = await fetch(`${PUBLIC_BACKEND_URL}/api/login`, {
        method: "POST",
        body: JSON.stringify(user),
        headers: {
            "Content-Type": "application/json",
        },
        credentials: "include",
    });

    if (!resp.ok) {
        switch (resp.status) {
            case 401:
                throw new InvalidCredsError(`Invalid username or password`);
            default:
                throw new Error(
                    `Response was not OK. Response was:\n${resp.statusText}`
                );
        }
    }
}

export async function logout() {
    const resp = await fetch(`${PUBLIC_BACKEND_URL}/api/logout`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
            // We need a CSRF token for state changing methods.
            "X-CSRF-TOKEN": getCookie("csrf_access_token"),
        },
        credentials: "include",
    });

    if (!resp.ok) {
        switch (resp.status) {
            case 401:
                throw new InvalidCredsError(`Invalid username or password`);
            default:
                throw new Error(
                    `Response was not OK. Response was:\n${resp.statusText}`
                );
        }
    }
}
