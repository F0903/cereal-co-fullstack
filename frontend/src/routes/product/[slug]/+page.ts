import { getSingleProduct } from "$lib/api";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
  const id = parseInt(params.slug);
  const product = await getSingleProduct(id);
  console.log(product);
  return { product };
};