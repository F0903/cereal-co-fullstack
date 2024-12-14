<script lang="ts">
  import ProductCard from "$lib/ProductCard.svelte";
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();
</script>

<div class="products-container">
  {#await data.productsTask}
    <span>Loading products...</span>
  {:then products}
    {#each products as product}
      <ProductCard {product} />
    {/each}
  {/await}
</div>

<style>
  .products-container {
    padding: 50px;
    box-sizing: border-box;
    display: grid;
    grid-template-columns: repeat(
      auto-fill,
      minmax(225px, 1fr)
    ); /* Automatically adjust columns */
    gap: 20px; /* Space between cards */
    padding: 20px; /* Padding around the grid */
  }
</style>
