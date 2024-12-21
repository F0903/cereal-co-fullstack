import { browser } from "$app/environment";
import { getCurrentUser } from "$lib/api/auth";
import { error } from "@sveltejs/kit";
import type { PageLoad } from "./$types";

export const load: PageLoad = async (params) => {
    let currentUser = undefined;
    try {
        currentUser = await getCurrentUser();
    } catch {}

    // Make sure we are an admin, return an unauthorized error if not, but only if we are in the browser.
    if (browser && (currentUser === undefined || !currentUser.is_admin)) {
        return error(401, "Unauthorized");
    }
};
