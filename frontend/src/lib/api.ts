import { PUBLIC_BACKEND_URL } from "$env/static/public";
//import { getCookie } from "$lib/utils/cookie_utils";
import { Product } from "./models/Product";

export async function getAllProducts(): Promise<Product[]> {
  const resp = await fetch(`${PUBLIC_BACKEND_URL}/api/v1/products`, {
    //credentials: "include",
  });
  if (!resp.ok) {
    throw new Error(`Response was not OK. Response was:\n${resp.statusText}`);
  }

  const products = await resp.json();
  let productModels: Product[] = [];
  products.forEach((productModel: Product) => {
    productModels.push(productModel);
  });

  return productModels;
}

export async function getSingleProduct(id: number): Promise<Product> {
  const resp = await fetch(`${PUBLIC_BACKEND_URL}/api/v1/products/${id}`, {
    credentials: "include",
  });
  if (!resp.ok) {
    throw new Error(`Response was not OK. Response was:\n${resp.statusText}`);
  }

  // The API always wraps it in an array.
  const products = await resp.json();
  const product = products[0];

  return product;
}

export async function setSingleProduct(id: number, product: Product) {
  const resp = await fetch(`${PUBLIC_BACKEND_URL}/api/v1/products/${id}`, {
    method: "PUT",
    body: JSON.stringify(product),
    /* headers: {
      "Content-Type": "application/json",
      // We need a CSRF token for state changing methods.
      "X-CSRF-TOKEN": getCookie("csrf_access_token"),
    },
    credentials: "include", */
  });
  if (!resp.ok) {
    throw new Error(`Response was not OK. Response was:\n${resp.statusText}`);
  }
}

export async function addSingleProduct(product: Product) {
  const resp = await fetch(`${PUBLIC_BACKEND_URL}/api/v1/products`, {
    method: "POST",
    body: JSON.stringify(product),
    /* headers: {
      "Content-Type": "application/json",
      // We need a CSRF token for state changing methods.
      "X-CSRF-TOKEN": getCookie("csrf_access_token"),
    },
    credentials: "include", */
  });
  if (!resp.ok) {
    throw new Error(`Response was not OK. Response was:\n${resp.statusText}`);
  }
}

export async function deleteSingleProduct(id: number) {
  const resp = await fetch(`${PUBLIC_BACKEND_URL}/api/v1/products/${id}`, {
    method: "DELETE",
    /* headers: {
      // We need a CSRF token for state changing methods.
      "X-CSRF-TOKEN": getCookie("csrf_access_token"),
    },
    credentials: "include", */
  });
  if (!resp.ok) {
    throw new Error(`Response was not OK. Response was:\n${resp.statusText}`);
  }
}
