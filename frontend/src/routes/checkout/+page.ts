import { browser } from "$app/environment";
import { goto } from "$app/navigation";
import { Cart } from "$lib/cart/Cart";
import { getCart } from "$lib/cart/localCartApi";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  // We can only get the cart (from localStorage) in a browser environment.
  const cart = browser ? getCart() : Cart.default();

  if (cart.isEmpty() && browser) {
    await goto("/");
  }

  return { cart };
};
