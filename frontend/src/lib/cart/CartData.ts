import type { CartItem } from "$lib/models/CartProduct";

export type CartData = {
  items: CartItem[];
  expiryMillis: number;
};
