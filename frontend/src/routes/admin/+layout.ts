import { browser } from "$app/environment";
import { error } from "@sveltejs/kit";
import type { LayoutLoad } from "./$types";

export const load: LayoutLoad = async (params) => {
    let { currentUser, loggedIn } = await params.parent();

    // Make sure we are an admin, return an unauthorized error if not, but only if we are in the browser.
    if (browser && (currentUser === undefined || !currentUser.is_admin)) {
        return error(401, "Unauthorized");
    }
};
