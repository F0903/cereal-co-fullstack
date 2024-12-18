import { PUBLIC_BACKEND_URL } from "$env/static/public";
import { autofetch } from "./utils";

export class UserForm {
    mail!: string;
    password_plain!: string;
}

export class UserInfo {
    is_admin!: boolean;
    mail!: string;
    decorative_username!: string;
}

export async function signup(userForm: UserForm) {
    await autofetch(`${PUBLIC_BACKEND_URL}/api/v1/signup`, {
        method: "POST",
        body: JSON.stringify(userForm),
        headers: {
            "Content-Type": "application/json",
        },
    });
}

export async function login(userForm: UserForm) {
    await autofetch(`${PUBLIC_BACKEND_URL}/api/v1/login`, {
        method: "POST",
        body: JSON.stringify(userForm),
        headers: {
            "Content-Type": "application/json",
        },
    });
}

export async function logout() {
    await autofetch(`${PUBLIC_BACKEND_URL}/api/v1/logout`, {
        method: "POST",
        headers: {
            "Content-Type": "application/json",
        },
    });
}

export async function getCurrentUser(): Promise<UserInfo> {
    const resp = await autofetch(`${PUBLIC_BACKEND_URL}/api/v1/get_user`);
    return resp.json();
}
