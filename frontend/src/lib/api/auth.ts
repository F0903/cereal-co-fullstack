import { PUBLIC_BACKEND_URL } from "$env/static/public";
import { autofetch } from "./utils";

export class ChangePasswordForm {
    old_password_plain!: string;
    new_password_plain!: string;
}

export class UserSignupForm {
    mail!: string;
    password_plain!: string;
    name!: string;
    address?: string;
    phone?: string;
}

export class UserLoginForm {
    mail!: string;
    password_plain!: string;
}

export class UserInfo {
    is_admin!: boolean;
    mail!: string;
    name!: string;
    address?: string;
    phone?: string;
}

export async function signup(userForm: UserSignupForm) {
    await autofetch(`${PUBLIC_BACKEND_URL}/api/v1/signup`, {
        method: "POST",
        body: JSON.stringify(userForm),
        headers: {
            "Content-Type": "application/json",
        },
    });
}

export async function login(userForm: UserLoginForm) {
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

export async function changePassword(passwordsForm: ChangePasswordForm) {
    await autofetch(`${PUBLIC_BACKEND_URL}/api/v1/change_password`, {
        method: "POST",
        body: JSON.stringify(passwordsForm),
        headers: {
            "Content-Type": "application/json",
        },
    });
}
