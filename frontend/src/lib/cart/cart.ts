import type { CartItem } from "$lib/models/CartProduct";
import { dateOffset } from "$lib/utils/datetimeUtils";
import { DEFAULT_CART_EXPIRY_MINUTES } from "./localCartApi";
import type { CartData } from "./CartData";

export class Cart {
  data: CartData;

  constructor(data: CartData) {
    this.data = data;
  }

  static default(): Cart {
    return new Cart({
      items: [],
      expiryMillis: dateOffset(DEFAULT_CART_EXPIRY_MINUTES),
    });
  }

  addItem(item: CartItem) {
    this.data.items.push(item);
  }

  removeItem(id: number) {
    // This isn't exactly efficient, but I'm not sure there is a better way to do it in JS/TS.
    const itemIndex = this.data.items.findIndex((x) => x.product.id == id);
    this.data.items.splice(itemIndex, 1);
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
      (a, b) => a + b.product.price * b.quantity,
      0
    );
  }

  len(): number {
    if (!this.data.items) {
      return 0;
    }

    return this.data.items.length;
  }

  find(predicate: (item: CartItem) => boolean): CartItem | undefined {
    if (!this.data.items) return undefined;
    return this.data.items.find(predicate);
  }

  isEmpty = () => this.len() <= 0;
}
