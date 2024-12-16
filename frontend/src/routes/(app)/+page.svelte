<script lang="ts">
  import Grid from "$lib/Grid.svelte";
  import ProductCard from "$lib/ProductCard.svelte";
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();
</script>

<Grid>
  {#await data.productsTask}
    <span>Loading products...</span>
  {:then products}
    {#each products as product}
      <ProductCard {product} />
    {/each}
  {/await}
</Grid>

<style>
</style>
