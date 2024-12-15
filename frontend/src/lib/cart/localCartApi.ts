import type { CartItem } from "$lib/models/CartProduct";
import { dateOffset } from "$lib/utils/datetimeUtils";
import { Cart } from "./Cart";

export const DEFAULT_CART_EXPIRY_MINUTES = 720;
const CART_LOCAL_STORAGE_NAME = "cart_data";

function readCart(): Cart {
  const cartData = localStorage.getItem(CART_LOCAL_STORAGE_NAME);
  return cartData !== null ? new Cart(JSON.parse(cartData)) : Cart.default();
}

function writeCart(cart: Cart) {
  localStorage.setItem(CART_LOCAL_STORAGE_NAME, JSON.stringify(cart.data));
}

export function clearCart() {
  localStorage.removeItem(CART_LOCAL_STORAGE_NAME);
}

export function getCart(): Cart {
  const cart = readCart();

  if (cart.isExpired()) {
    // Cart expired, clear it
    clearCart();
    return cart;
  }

  return cart;
}

export function addToCart(
  item: CartItem,
  expiresMinutes = DEFAULT_CART_EXPIRY_MINUTES
) {
  const cart = readCart();

  const existingProduct = cart.find((x) => x.product.id == item.product.id);

  // If the product is already in the cart, we can just increment the quantity
  if (existingProduct) {
    console.log("existing product in cart, incrementing quantity...");
    existingProduct.quantity += 1;
  } else {
    console.log("adding new product to cart...");
    cart.addItem(item);
  }

  cart.setExpiry(dateOffset(expiresMinutes));

  writeCart(cart);
}

export function removeCartItem(id: number, cart: Cart = readCart()) {
  cart.removeItem(id);
  writeCart(cart);
}

export function modifyCartItem(
  id: number,
  modifier: (item: CartItem) => void,
  cart: Cart = readCart()
) {
  for (const item of cart.data.items) {
    if (item.product.id != id) continue;
    modifier(item);
  }

  writeCart(cart);
}
