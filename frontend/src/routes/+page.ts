import { getAllProducts } from "$lib/api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async () => {
  const products = await getAllProducts();

  return { products };
};
