import { PUBLIC_BACKEND_URL } from "$env/static/public";

export function getFullImageUrl(path: string): string {
  return PUBLIC_BACKEND_URL + path;
}

export function fetchWithCreds(
  input: RequestInfo | URL,
  init?: RequestInit
): Promise<Response> {
  if (!init) {
    init = {};
  }
  init.credentials = "include";

  return fetch(input, init);
}
