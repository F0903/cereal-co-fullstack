<script lang="ts">
  import { PUBLIC_BACKEND_URL } from "$env/static/public";
  import ProductCard from "$lib/ProductCard.svelte";
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();
</script>

<div class="products-container">
  {#each data.products as product}
    <ProductCard
      name={product.name}
      price="${product.price}"
      imageUrl="{PUBLIC_BACKEND_URL}{product.image_url}"
    />
  {/each}
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
