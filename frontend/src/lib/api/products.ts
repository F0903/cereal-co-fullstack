import { PUBLIC_BACKEND_URL } from "$env/static/public";
import { autofetch } from "./utils";

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

export async function getAllProducts(): Promise<Product[]> {
    const resp = await autofetch(`${PUBLIC_BACKEND_URL}/api/v1/products`);
    return resp.json();
}

export async function getSingleProduct(id: number): Promise<Product> {
    const resp = await autofetch(`${PUBLIC_BACKEND_URL}/api/v1/products/${id}`);
    return resp.json();
}

export async function updateSingleProduct(product: Product) {
    await autofetch(`${PUBLIC_BACKEND_URL}/api/v1/products/${product.id}`, {
        method: "PUT",
        body: JSON.stringify(product),
        headers: {
            "Content-Type": "application/json",
        },
    });
}

export async function addSingleProduct(product: Product) {
    await autofetch(`${PUBLIC_BACKEND_URL}/api/v1/products`, {
        method: "POST",
        body: JSON.stringify(product),
        headers: {
            "Content-Type": "application/json",
        },
    });
}

export async function deleteSingleProduct(id: number) {
    await autofetch(`${PUBLIC_BACKEND_URL}/api/v1/products/${id}`, {
        method: "DELETE",
    });
}
