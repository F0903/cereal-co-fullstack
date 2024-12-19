import { browser } from "$app/environment";
import { getCurrentUser } from "$lib/api/auth";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async (params) => {
    let currentUser = undefined;
    try {
        currentUser = browser ? await getCurrentUser() : undefined;
    } catch {}

    // Make sure we are an admin.
    if (currentUser === undefined || !currentUser.is_admin) {
        return error(401, "Unauthorized");
    }
};
