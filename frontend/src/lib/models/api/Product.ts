interface ProductAttributes {
  [key: string]: string;
}

// These models represent the "raw state (JSON)" of the object comming over the network.
export class Product {
  id!: string;
  name!: string;
  description!: string;
  manufacturer!: string;
  quantity!: string;
  price!: string;
  image_url!: string;
  attributes!: ProductAttributes;
  created_at!: string;
  updated_at!: string;
}
