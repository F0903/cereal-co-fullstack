<script lang="ts">
  import { getFullImageUrl } from "$lib/api/utils";
  import Button from "$lib/Button.svelte";
  import Image from "$lib/Image.svelte";
  import Spacer from "$lib/Spacer.svelte";
  import { faCheck } from "@fortawesome/free-solid-svg-icons";
  import type { PageData } from "./$types";

  let { data }: { data: PageData } = $props();

  async function onCheckoutClick() {}
</script>

<div class="checkout">
  <h1>Checkout</h1>
  <Spacer --margin-bottom="50px" --width="10%" />
  {#each data.products as product}
    <div class="item">
      <Image
        src={getFullImageUrl(product.image_url)}
        alt="Image of {product.name}"
        --width="50px"
        --height="50px"
        --border-radius="7px"
      />
      <h4>{product.name}</h4>
      <span>{product.id}</span>
      <span>{product.price}</span>
    </div>
  {/each}
  <div class="checkout-price-container">
    <span
      >{data.products.reduce<number>(
        (a, b) => a + parseFloat(b.price),
        0
      )}</span
    >
    <Button text="Checkout" prefixIcon={faCheck} onclick={onCheckoutClick} />
  </div>
</div>

<style>
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
