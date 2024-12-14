import { PUBLIC_BACKEND_URL } from "$env/static/public";

export function getFullImageUrl(path: string): string {
  return PUBLIC_BACKEND_URL + path;
}
