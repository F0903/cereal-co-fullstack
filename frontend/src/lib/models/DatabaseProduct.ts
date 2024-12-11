import type { Product } from "./Product";
import type { ProductDescriptor } from "./ProductDescriptor";

export type DatabaseProduct = {
    Descriptor: ProductDescriptor;
    Product: Product;
};
