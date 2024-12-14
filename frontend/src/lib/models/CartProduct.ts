import type { Product } from "./api/Product";

export class CartItem {
  product!: Product;
  quantity!: number;
}
