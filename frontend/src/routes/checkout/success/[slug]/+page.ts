import { getOrder } from "$lib/api/orders";
import type { PageLoad } from "./$types";

export const load: PageLoad = async ({ params }) => {
    const id = parseInt(params.slug);
    const order = await getOrder(id);
    return { order };
};
