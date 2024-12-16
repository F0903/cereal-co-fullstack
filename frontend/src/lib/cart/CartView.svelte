<script lang="ts">
  import { onMount } from "svelte";
  import Image from "$lib/Image.svelte";
  import { getFullImageUrl } from "$lib/api/utils";
  import Button from "$lib/Button.svelte";
  import { goto } from "$app/navigation";
  import Spacer from "$lib/Spacer.svelte";
  import Counter from "$lib/Counter.svelte";
  import { getCart, modifyCartItem, removeCartItem } from "./localCartApi";
  import { Cart } from "./Cart";

  let { visible = $bindable(false) } = $props();

  let cart = $state(Cart.default());

  async function onCheckoutClick() {
    visible = false;
    await goto("/checkout");
  }

  function reloadCart() {
    cart = getCart();
  }

  $effect(() => {
    visible; // Run when visible changes.
    reloadCart();
  });
</script>

<div class="cart-view">
  <Spacer
    --margin-top="0px"
    --margin-bottom="15px"
    --width="100%"
    --color="var(--primary-color)"
  />
  <div class="cart-items">
    {#each cart.getItems() as item}
      <div class="cart-item">
        <Image
          src={getFullImageUrl(item.product.image_url)}
          alt="Image of {item.product.name}"
          --width="50px"
          --height="50px"
          --border-radius="7px"
        />
        <h4 class="product-name">{item.product.name}</h4>
        <Counter
          value={item.quantity}
          on_value_changed={(newVal) => {
            // This is extremely inefficient
            modifyCartItem(item.product.id, (itm) => (itm.quantity = newVal));
            reloadCart();
          }}
          on_negative_value_callback={() => {
            removeCartItem(item.product.id);
            reloadCart(); // Explicitly reloading the cart like this isn't exactly pretty. But I have not figured out a way to make this better with stores and the current cart API.
          }}
          --background-color="var(--secondary-color)"
          --seperator-color="var(--tertiary-color)"
        />
        <span class="price-text"
          >${(item.product.price * item.quantity).toFixed(2)}</span
        >
      </div>
    {/each}
  </div>

  {#if !cart.isEmpty()}
    <div class="total-price-container">
      <span class="total-text">Total:</span>
      <span class="price-text">${cart.calcSum().toFixed(2)}</span>
    </div>
    <div class="checkout-button-container">
      <Button text="Checkout" onclick={onCheckoutClick} />
    </div>
  {:else}
    <span style="text-align: center; width: 100%;"
      >it's a little empty up here...</span
    >
  {/if}
</div>

<style>
  .price-text {
    font-family: "Roboto Slab Variable";
    font-weight: 700;
  }

  .total-price-container {
    padding-top: 15px;
    font-size: 1.5em;
    border-bottom: 2px solid var(--tertiary-color);
    width: fit-content;
  }

  .checkout-button-container {
    margin-top: 15px;
  }

  .cart-items {
    overflow-y: scroll;
    flex-shrink: 10;
  }

  .product-name {
    text-align: center;
  }

  .cart-item {
    display: flex;
    flex-direction: row;
    justify-content: space-between;
    align-items: center;

    padding: 10px 15px;

    background-color: var(--primary-color);
  }

  .cart-view {
    --cart-max-height: 50vh;

    padding: 25px;
    padding-top: 0px;
    background-color: var(--secondary-color);

    border-radius: 10px;
    border-top-left-radius: 0px;
    border-top-right-radius: 0px;

    box-shadow: 0px 0px 25px 10px rgba(0, 0, 0, 0.2);
    /* Allow shadow around but not above*/
    clip-path: inset(0px -50px -50px -50px);

    min-width: 400px;
    max-height: var(--cart-max-height);

    display: flex;
    flex-direction: column;
  }
</style>
