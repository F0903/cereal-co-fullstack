<script lang="ts">
  import { getFullImageUrl } from "$lib/api/utils";
  import Button from "$lib/Button.svelte";
  import Image from "$lib/Image.svelte";
  import Spacer from "$lib/Spacer.svelte";
  import { faCheck } from "@fortawesome/free-solid-svg-icons";
  import type { PageData } from "./$types";
  import InputField from "$lib/InputField.svelte";

  let { data }: { data: PageData } = $props();

  let totalPrice = $derived.by(() => {
    const sum = data.cart.calcSum().toFixed(2);
    return sum;
  });

  async function onCheckoutClick() {}
</script>

<div class="checkout">
  <h1>Checkout</h1>
  <Spacer --margin-bottom="50px" --width="10%" />

  {#each data.cart.getItems() as item}
    <div class="item">
      <Image
        src={getFullImageUrl(item.product.image_url)}
        alt="Image of {item.product.name}"
        --width="50px"
        --height="50px"
        --border-radius="7px"
      />
      <h4>{item.product.name}</h4>
      <span>{item.quantity}</span>
      <span>{item.product.price}</span>
    </div>
  {/each}
  <Spacer --margin-bottom="50px" --width="10%" />

  <form class="shipping-form">
    <InputField name="Name" />
  </form>
  <div class="checkout-price-container">
    <span class="total-price">${totalPrice}</span>
    <Spacer --margin-bottom="5px" />
    <Button text="Purchase" prefixIcon={faCheck} onclick={onCheckoutClick} />
  </div>
</div>

<style>
  .total-price {
    font-size: 1.5em;
    font-weight: 700;
  }

  .checkout-price-container {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;

    background-color: var(--primary-color);

    padding: 15px;
    border-radius: 10px;
  }

  .item {
    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: space-evenly;
    width: 100%;
  }

  .checkout {
    padding: 50px;
    background-color: var(--secondary-color);
    margin-top: 100px;
    margin-left: auto;
    margin-right: auto;
    border-radius: 25px;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;

    max-width: 1000px;
  }
</style>
