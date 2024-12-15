interface ProductAttributes {
  [key: string]: string;
}

// These models represent the "raw state (JSON)" of the object comming over the network.
//
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
