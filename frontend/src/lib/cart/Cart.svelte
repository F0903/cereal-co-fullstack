<script lang="ts">
  import { onMount } from "svelte";
  import { getCart } from "./cart";
  import type { Product } from "../models/Product";
  import Image from "$lib/Image.svelte";
  import { getFullImageUrl } from "$lib/api";
  import Button from "$lib/Button.svelte";
  import { goto } from "$app/navigation";
  import Spacer from "$lib/Spacer.svelte";

  let { visible = $bindable(false) } = $props();

  let cartProducts: Product[] = $state([]);

  async function onCheckoutClick() {
    visible = false;
    await goto("/checkout");
  }

  onMount(() => {
    cartProducts = getCart().products;
  });
</script>

{#key visible}
  {#if visible}
    <div class="cart-view">
      <h3>Cart</h3>
      <Spacer --margin-top="0px" --margin-bottom="15px" --width="10%" />
      <div class="cart-items">
        {#each cartProducts as product}
          <div class="cart-item">
            <Image
              src={getFullImageUrl(product.image_url)}
              alt="Image of {product.name}"
              --width="50px"
              --height="50px"
              --border-radius="7px"
            />
            <h4>{product.name}</h4>
          </div>
        {/each}
      </div>
      <div class="checkout-button-container">
        <Button text="Checkout" onclick={onCheckoutClick} />
      </div>
    </div>
  {/if}
{/key}

<style>
  .checkout-button-container {
    margin-top: 15px;
  }

  .cart-items {
    overflow-y: scroll;
    flex-shrink: 10;
  }

  .cart-item:nth-child(2n) {
    background-color: var(--secondary-color);
  }

  .cart-item {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;

    padding: 10px 5px;

    background-color: var(--primary-color);
  }

  h3 {
    font-size: 1.7em;
    text-align: center;
    margin: 0px;
  }

  .cart-view {
    --cart-max-height: 50vh;

    padding: 25px;
    background-color: var(--secondary-color);

    border-radius: 10px;
    border-top-left-radius: 0px;
    border-top-right-radius: 0px;

    box-shadow: 0px 0px 25px 10px rgba(0, 0, 0, 0.2);
    /* Allow shadow around but not above*/
    clip-path: inset(0px -50px -50px -50px);

    min-height: 200px;
    height: var(--cart-max-height);

    display: flex;
    flex-direction: column;
  }
</style>
