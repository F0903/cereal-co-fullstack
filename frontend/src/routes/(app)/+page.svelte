<script lang="ts">
    import Grid from "$lib/generic/Grid.svelte";
    import ProductCard from "$lib/products/ProductCard.svelte";
    import type { PageData } from "./$types";

    let { data }: { data: PageData } = $props();
</script>

{#await data.productsTask}
    <div class="loading-container">
        <span>Loading products...</span>
    </div>
{:then products}
    <Grid>
        {#each products as product}
            <ProductCard {product} />
        {/each}
    </Grid>
{/await}

<style>
    .loading-container {
        margin: auto;
        margin-top: 100px;

        text-align: center;
        font-weight: 100;
        font-family: "Caveat Variable", cursive;
        font-size: 1.8em;
    }
</style>
