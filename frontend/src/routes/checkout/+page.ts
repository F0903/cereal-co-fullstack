import { browser } from "$app/environment";
import { getCart } from "$lib/cart/cart";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  // We can only get the cart (from localStorage) in a browser environment.
  const products = browser ? getCart().products : [];
  return { products };
};
