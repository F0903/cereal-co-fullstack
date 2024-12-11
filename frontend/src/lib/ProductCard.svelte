<script lang="ts">
  import Image from "./Image.svelte";

  let { name, price, imageUrl } = $props();
</script>

<div class="card">
  <Image src={imageUrl} alt="Image of {name}" />
  <h2 class="name">{name}</h2>
  <span class="price">{price}</span>
</div>

<style>
  .name {
    font-weight: 600;
  }

  .price {
    font-weight: 800;
    font-size: 1.2em;
  }

  .card {
    background-color: var(--secondary-color);
    width: 100%;
    padding: 10px;
    border-radius: 15px;
    box-sizing: border-box;
  }
</style>
