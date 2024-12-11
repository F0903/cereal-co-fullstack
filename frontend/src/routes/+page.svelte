<script lang="ts">
  import ProductCard from "$lib/ProductCard.svelte";
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();
</script>

<div class="products-container">
  {#each data.products as product}
    <ProductCard name={product.name} price="${product.price}" imageUrl="" />
  {/each}
</div>

<style>
  .products-container {
    padding: 50px;
    box-sizing: border-box;
  }
</style>
