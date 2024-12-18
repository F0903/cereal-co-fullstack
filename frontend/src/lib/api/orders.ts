import { PUBLIC_BACKEND_URL } from "$env/static/public";
import { autofetch } from "./utils";

// All of these are used be sent as json, so it doesn't matter that the fields are strings, as they will be anyway.
export class OrderItem {
    product_id!: number;
    quantity!: number;
}

export class Order {
    shipping_name!: string;
    shipping_phone!: string;
    shipping_mail!: string;
    shipping_address!: string;
    total!: number;
    order_items!: OrderItem[];
}

export class OrderWithTimestamps {
    order_id!: number;
    order_form!: Order;
    created_at!: string;
    updated_at!: string;
}

// Returns the created orders id if successful
export async function addOrder(order: Order): Promise<number> {
    const resp = await autofetch(`${PUBLIC_BACKEND_URL}/api/v1/orders`, {
        method: "POST",
        body: JSON.stringify(order),
        headers: {
            "Content-Type": "application/json",
        },
    });

    const json: { id: number } = await resp.json();
    return json.id;
}

export async function getOrder(id: number): Promise<Order> {
    const resp = await autofetch(`${PUBLIC_BACKEND_URL}/api/v1/orders/${id}`);
    return resp.json();
}

export async function getOrdersByMail(
    mail: string
): Promise<OrderWithTimestamps[]> {
    const searchParams = new URLSearchParams();
    if (mail) searchParams.append("mail", mail);

    const resp = await autofetch(
        `${PUBLIC_BACKEND_URL}/api/v1/orders?${searchParams}`
    );
    return resp.json();
}
