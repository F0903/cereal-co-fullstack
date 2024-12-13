import type { Product } from "../models/Product";

class Cart {
  expiryMillis!: number;
  products!: Product[];
}

const minuteToMillis = (minutes: number) => minutes * 60 * 1000;
const dateOffset = (minutes: number) => Date.now() + minuteToMillis(minutes);

export function getCart(expiresMinutes = 720): Cart {
  const rawCart = localStorage.getItem("cart");
  const cart: Cart =
    rawCart !== null
      ? JSON.parse(rawCart)
      : { expiryMillis: dateOffset(expiresMinutes), products: [] };

  if (!rawCart) {
    return cart;
  }

  if (Date.now() > cart.expiryMillis) {
    // Cart expired, clear it
    localStorage.removeItem("cart");
    return cart;
  }

  return cart;
}

export function addToCart(product: Product, expiresMinutes = 720) {
  const rawCart = localStorage.getItem("cart");
  const cart: Cart =
    rawCart !== null
      ? JSON.parse(rawCart)
      : { expiryMillis: dateOffset(expiresMinutes), products: [] };

  cart.products.push(product);
  cart.expiryMillis = dateOffset(expiresMinutes);

  localStorage.setItem("cart", JSON.stringify(cart));
}
