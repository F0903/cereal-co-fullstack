class OrderItem {
  product_id!: number;
  quantity!: number;
}

class Order {
  shipping_name!: string;
  shipping_phone!: string;
  shipping_mail!: string;
  shipping_address!: string;
  total!: number;
  order_items!: OrderItem[];
}
