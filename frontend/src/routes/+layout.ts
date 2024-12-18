import { browser } from "$app/environment";
import { getCurrentUser } from "$lib/api/auth";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async () => {
    // This will never succeed on SSR, so only run in browser.
    let currentUser = undefined;
    try {
        currentUser = browser ? await getCurrentUser() : undefined;
    } catch {}

    return { loggedIn: currentUser !== undefined, currentUser };
};
