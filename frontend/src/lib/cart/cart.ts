import type { CartItem } from "$lib/models/CartProduct";

const DEFAULT_EXPIRY_MINUTES = 720;

type CartData = {
  items: CartItem[];
  expiryMillis: number;
};

export class Cart {
  data: CartData;

  constructor(data: CartData) {
    this.data = data;
  }

  static default(): Cart {
    return new Cart({
      items: [],
      expiryMillis: dateOffset(DEFAULT_EXPIRY_MINUTES),
    });
  }

  addItem(item: CartItem) {
    this.data.items.push(item);
  }

  setExpiry(expiryMillis: number) {
    this.data.expiryMillis = expiryMillis;
  }

  isExpired(): boolean {
    return Date.now() > this.data.expiryMillis;
  }

  getItems(): CartItem[] {
    return this.data.items;
  }

  calcSum(): number {
    return this.data.items.reduce<number>(
      (a, b) => a + parseFloat(b.product.price) * b.quantity,
      0
    );
  }

  len = () => this.data.items.length;
  isEmpty = () => this.data.items.length <= 0;
}

const minuteToMillis = (minutes: number) => minutes * 60 * 1000;
const dateOffset = (minutes: number) => Date.now() + minuteToMillis(minutes);

function readCart(expiresMinutes = DEFAULT_EXPIRY_MINUTES): Cart {
  const rawCart = localStorage.getItem("cart");
  return rawCart !== null ? new Cart(JSON.parse(rawCart)) : Cart.default();
}

export function getCart(expiresMinutes = DEFAULT_EXPIRY_MINUTES): Cart {
  const cart = readCart(expiresMinutes);
  console.log(cart);

  if (cart.isExpired()) {
    // Cart expired, clear it
    localStorage.removeItem("cart");
    return cart;
  }

  return cart;
}

export function addToCart(
  item: CartItem,
  expiresMinutes = DEFAULT_EXPIRY_MINUTES
) {
  const cart = readCart(expiresMinutes);

  let existingProduct = cart.data.items.find(
    (x) => x.product.id == item.product.id
  );

  // If the product is already in the cart, we can just increment the quantity
  if (existingProduct) {
    console.log("existing product in cart, incrementing quantity...");
    existingProduct.quantity += 1;
  } else {
    console.log("adding new product to cart...");
    cart.addItem(item);
  }

  cart.setExpiry(dateOffset(expiresMinutes));

  localStorage.setItem("cart", JSON.stringify(cart));
}
