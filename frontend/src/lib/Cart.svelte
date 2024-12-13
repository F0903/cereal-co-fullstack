<script lang="ts">
  import { onMount } from "svelte";
  import { getCart } from "./cart";
  import type { Product } from "./models/Product";

  let cartProducts: Product[] = $state([]);

  onMount(() => {
    cartProducts = getCart().products;
  });
</script>

<div class="cart-view">
  <h3>Cart</h3>
  {#each cartProducts as product}
    <div class="cart-item">
      <h3>{product.name}</h3>
    </div>
  {/each}
</div>

<style>
  h3 {
    font-size: 1.5em;
  }

  .cart-view {
    padding: 10px 50px;
    background-color: var(--secondary-color);
    border-radius: 10px;
    border-top-left-radius: 0px;
    border-top-right-radius: 0px;
    box-shadow: 0px 0px 25px 10px rgba(0, 0, 0, 0.2);
    /* Allow shadow around but not above*/
    clip-path: inset(0px -50px -50px -50px);
  }
</style>
