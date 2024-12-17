import { browser } from "$app/environment";
import { getCurrentUser } from "$lib/api/auth";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async () => {
    // This will never succeed on SSR, so only run in browser.
    const currentUser = browser ? await getCurrentUser() : undefined;

    return { loggedIn: currentUser !== undefined, currentUser };
};
