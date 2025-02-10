<script lang="ts">
    import Grid from "$lib/generic/Grid.svelte";
    import ProductCard from "$lib/products/ProductCard.svelte";
    import type { PageData } from "./$types";
    import type { Product } from "$lib/api/products";
    import { getContext } from "svelte";

    let { data }: { data: PageData } = $props();

    let searchData: { value: string } = getContext("search");

    // In a real world application you would filter the query instead.
    function filterProducts(products: Product[]) {
        const search = searchData.value;
        if (!search) return products;

        const filtered = products.filter((product) =>
            product.name.toLowerCase().includes(search.toLowerCase()),
        );

        return filtered;
    }
</script>

{#await data.productsTask}
    <div class="loading-container center-text">
        <span>Loading products...</span>
    </div>
{:then products}
    {@const filteredProducts = filterProducts(products)}
    <Grid>
        {#each filteredProducts as product}
            <ProductCard {product} />
        {/each}
    </Grid>
{:catch err}
    <div class="error-container center-text">
        <span>{err}</span>
    </div>
{/await}

<style>
    .loading-container {
        font-weight: 100;
        font-family: "Caveat Variable", cursive;
        font-size: 1.8em;
    }

    .error-container {
        font-weight: 400;
        font-size: 1.8em;
        color: var(--error-color);
    }

    .center-text {
        margin: auto;
        margin-top: 100px;

        text-align: center;
    }
</style>
