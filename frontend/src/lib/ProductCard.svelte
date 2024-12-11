<script lang="ts">
  import Button from "./Button.svelte";
  import ClickableImage from "./ClickableImage.svelte";
  import Image from "./Image.svelte";
  import Spacer from "./Spacer.svelte";

  let { name, price, imageUrl } = $props();
</script>

<div class="card">
  <ClickableImage src={imageUrl} alt="Image of {name}" />
  <h2 class="name">{name}</h2>
  <Spacer />
  <div class="buy-container">
    <span class="price">{price}</span>
    <Button text="Add" />
  </div>
</div>

<style>
  .buy-container {
    width: 100%;
    display: flex;
    flex-direction: row;
    justify-content: center;
    align-items: center;
    gap: 15px;
    justify-self: end;
    align-self: end;
    margin-top: auto;
  }

  .name {
    font-weight: 600;
    margin-bottom: 0px;
    text-align: center;
  }

  .price {
    font-family: "Roboto Slab Variable";
    font-weight: 800;
    font-size: 1.2em;
    padding: 10px;
    border-radius: 15px;
    background-color: var(--primary-color);
  }

  .card {
    display: flex;
    flex-direction: column;
    background-color: var(--secondary-color);
    width: 100%;
    padding: 10px;
    border-radius: 15px;
    box-sizing: border-box;
  }
</style>
