import { getAllProducts } from "$lib/api/products";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
    return { productsTask: getAllProducts() };
};
