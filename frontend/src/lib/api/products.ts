import { PUBLIC_BACKEND_URL } from "$env/static/public";
import { assertOk } from "./errors";
import { fetchWithCreds } from "./utils";

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
    const resp = await fetchWithCreds(`${PUBLIC_BACKEND_URL}/api/v1/products`);

    assertOk(resp);

    return resp.json();
}

export async function getSingleProduct(id: number): Promise<Product> {
    const resp = await fetchWithCreds(
        `${PUBLIC_BACKEND_URL}/api/v1/products/${id}`
    );

    assertOk(resp);

    return resp.json();
}

export async function setSingleProduct(id: number, product: Product) {
    const resp = await fetchWithCreds(
        `${PUBLIC_BACKEND_URL}/api/v1/products/${id}`,
        {
            method: "PUT",
            body: JSON.stringify(product),
            headers: {
                "Content-Type": "application/json",
            },
        }
    );

    assertOk(resp);
}

export async function addSingleProduct(product: Product) {
    const resp = await fetchWithCreds(`${PUBLIC_BACKEND_URL}/api/v1/products`, {
        method: "POST",
        body: JSON.stringify(product),
        headers: {
            "Content-Type": "application/json",
        },
    });

    assertOk(resp);
}

export async function deleteSingleProduct(id: number) {
    const resp = await fetchWithCreds(
        `${PUBLIC_BACKEND_URL}/api/v1/products/${id}`,
        {
            method: "DELETE",
        }
    );

    assertOk(resp);
}
