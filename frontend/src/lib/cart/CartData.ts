import type { CartItem } from "./Cart";

export type CartData = {
  items: CartItem[];
  expiryMillis: number;
};
