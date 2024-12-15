// All of these are used be sent as json, so it doesn't matter that the fields are strings, as they will be anyway.
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
