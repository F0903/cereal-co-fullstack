interface ProductAttributes {
  [key: string]: string;
}
export class Product {
  id!: number;
  name!: string;
  description!: string;
  manufacturer!: string;
  quantity!: number;
  price!: number;
  image_url!: string;
  attributes!: ProductAttributes;
  created_at!: string;
  updated_at!: string;
}
