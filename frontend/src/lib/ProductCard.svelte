<script lang="ts">
  let { name, price, imageUrl } = $props();
</script>

<div class="card">
  <img src={imageUrl} alt="Image of {name}" />
  <h2 class="name">{name}</h2>
  <span class="price">{price}</span>
</div>

<style>
  .price {
    font-weight: 800;
  }

  .card {
    background-color: var(--secondary-color);
    width: fit-content;
    padding: 10px;
    border-radius: 15px;
  }
</style>
