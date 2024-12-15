<script lang="ts">
  import { getFullImageUrl } from "$lib/api/utils";
  import Button from "$lib/Button.svelte";
  import Image from "$lib/Image.svelte";
  import Spacer from "$lib/Spacer.svelte";
  import { faShoppingCart } from "@fortawesome/free-solid-svg-icons";
  import type { PageData } from "./$types";
  import { addToCart } from "$lib/cart/localCartApi";

  let { data }: { data: PageData } = $props();

  const stockStatus =
    data.product.quantity > 25
      ? "Good"
      : data.product.quantity <= 0
        ? "Empty"
        : "Limited";

  function onAddClick() {
    addToCart({ product: data.product, quantity: 1 });
  }
</script>

<div class="product-container">
  <div class="content-container-row">
    <div class="image-container">
      <Image
        src={getFullImageUrl(data.product.image_url)}
        alt="Image of {data.product.name}"
      />
    </div>

    <div class="content-column">
      <h2>{data.product.name}</h2>
      <Spacer center={false} />
      <p class="description">{data.product.description}</p>
      <Spacer --color="var(--primary-color)" --spacing="50px" --width="50%" />
      <div class="product-attributes">
        <h3 class="product-attributes-title">Additional Information</h3>
        {#each Object.entries(data.product.attributes) as [name, value]}
          <div class="product-attribute">
            <span class="product-attribute-name">{name}: </span>
            <span class="product-attribute-value">{value}</span>
            <br />
          </div>
        {/each}
      </div>
    </div>

    <div class="buy-section">
      <span class="product-price">${data.product.price}</span>
      <Spacer
        --color="var(--secondary-color)"
        --spacing="0px"
        --margin-top="-2px"
        --margin-bottom="3px"
      />
      <Button
        text="Buy"
        prefixIcon={faShoppingCart}
        onclick={onAddClick}
        --font-size="1.1em"
        --padding="7px"
        --border-radius="8px"
      />
      <span
        class="stock-value"
        class:good-stock={stockStatus === "Good"}
        class:limited-stock={stockStatus === "Limited"}
        class:empty-stock={stockStatus === "Empty"}
        >{data.product.quantity} in stock</span
      >
    </div>
  </div>
</div>

<style>
  .product-price {
    font-family: "Roboto Slab Variable";
    font-size: 1.8em;
    font-weight: 800;
  }

  .buy-section {
    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
    gap: 10px;
    max-width: 150px;
    padding: 15px 25px;
    margin: 75px auto;

    border-radius: 10px;
    background-color: var(--primary-color);
  }

  .stock-value.empty-stock {
    color: hsl(0, 46%, 36%);
  }

  .stock-value.limited-stock {
    color: hsl(60, 82%, 48%);
  }

  .stock-value.good-stock {
    color: hsl(120, 36%, 31%);
  }

  .product-attribute-name {
    font-weight: 800;
  }

  .product-attribute:last-child {
    border-bottom: var(--product-attribute-border-width) solid
      var(--product-attribute-border-color);
  }

  .product-attribute {
    --product-attribute-border-width: 1px;
    --product-attribute-border-color: var(--secondary-color);
    padding: 5px;
    border-top: var(--product-attribute-border-width) solid
      var(--product-attribute-border-color);
  }

  .product-attributes-title {
    margin-top: 5px;
  }

  .product-attributes {
    padding: 15px;
    background-color: var(--primary-color);
    border-radius: 10px;
  }

  .description {
    max-width: 750px;
  }

  .content-column {
    display: flex;
    flex-direction: column;
  }

  h2 {
    font-size: 2em;
    margin-top: 15px;
    margin-bottom: 0px;
  }

  .content-container-row {
    display: flex;
    flex-direction: row;
    gap: 50px;
    justify-content: center;
    align-items: start;
  }

  .product-container {
    margin: 50px auto;
    padding: 75px;
    padding-top: 50px;
    max-width: 1250px;
    width: fit-content;

    border-radius: 20px;
    background-color: var(--secondary-color);
  }
</style>
