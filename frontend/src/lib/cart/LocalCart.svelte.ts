import type { Product } from "$lib/api/products";
import { dateOffset } from "$lib/utils/datetimeUtils";

const CART_LOCAL_STORAGE_NAME = "cart_data";
export const DEFAULT_CART_EXPIRY_MINUTES = 720;

export class CartItem {
    product!: Product;
    quantity!: number;
}

export type CartData = {
    items: CartItem[];
    expiryMillis: number;
};

export class LocalCart implements LocalCart {
    data: CartData = $state({
        items: [],
        expiryMillis: dateOffset(DEFAULT_CART_EXPIRY_MINUTES),
    });

    constructor(data?: CartData) {
        if (data) this.data = data;
    }

    static getPersisted(): LocalCart {
        const cartData = localStorage.getItem(CART_LOCAL_STORAGE_NAME);
        const cart =
            cartData !== null
                ? new LocalCart(JSON.parse(cartData))
                : new LocalCart();

        if (cart.isExpired()) {
            // Cart expired, clear it
            LocalCart.clearPersisted();
            return cart;
        }

        return cart;
    }

    static clearPersisted() {
        localStorage.removeItem(CART_LOCAL_STORAGE_NAME);
    }

    persist() {
        localStorage.setItem(
            CART_LOCAL_STORAGE_NAME,
            JSON.stringify(this.data)
        );
    }

    addItem(item: CartItem) {
        const existingProduct = this.find(
            (x) => x.product.id == item.product.id
        );

        // If the product is already in the cart, we can just increment the quantity
        if (existingProduct) {
            existingProduct.quantity += 1;
        } else {
            this.data.items.push(item);
        }

        this.setExpiry(dateOffset(DEFAULT_CART_EXPIRY_MINUTES));
        this.persist();
    }

    removeItem(id: number) {
        // This isn't exactly efficient, but I'm not sure there is a better way to do it in JS/TS.
        const itemIndex = this.data.items.findIndex((x) => x.product.id == id);
        this.data.items.splice(itemIndex, 1);
        this.persist();
    }

    clear() {
        this.data.items = [];
        LocalCart.clearPersisted();
    }

    modifyItem(id: number, modifier: (item: CartItem) => void) {
        for (const item of this.data.items) {
            if (item.product.id != id) continue;
            modifier(item);
        }
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

    getDisplayableTotalSum(): string {
        return this.calcSum().toFixed(2);
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
