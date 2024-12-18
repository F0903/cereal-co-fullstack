<script lang="ts">
    import type { OrderWithTimestamps } from "$lib/api/orders";
    import { getSingleProduct } from "$lib/api/products";
    import { getFullImageUrl } from "$lib/api/utils";
    import Image from "$lib/generic/Image.svelte";

    import Table from "$lib/generic/Table.svelte";

    let { order }: { order: OrderWithTimestamps } = $props();
</script>

<Table>
    <thead>
        <tr>
            <th>Image</th>
            <th>Name</th>
            <th>Quantity</th>
            <th>Price</th>
            <th>Total</th>
        </tr>
    </thead>
    <tbody>
        {#each order.order_form.order_items as item}
            {@const productTask = getSingleProduct(item.product_id)}
            {#await productTask}
                <tr>
                    <td>
                        <span class="product-loading-text">Loading...</span>
                    </td>
                </tr>
            {:then product}
                <tr>
                    <td>
                        <Image
                            src={getFullImageUrl(product.image_url)}
                            alt="Image of {product.name}"
                            --width="50px"
                            --height="50px"
                            --border-radius="7px"
                        />
                    </td>
                    <td>{product.name}</td>
                    <td>{item.quantity}</td>
                    <td>${product.price}</td>
                    <td>${(item.quantity * product.price).toFixed(2)}</td>
                </tr>
            {/await}
        {/each}
    </tbody>
</Table>
