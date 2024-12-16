import type { LayoutServerLoad } from "./$types";

export const load: LayoutServerLoad = async ({ cookies }) => {
  const loggedIn = cookies.get("Auth") !== undefined;

  return { loggedIn };
};