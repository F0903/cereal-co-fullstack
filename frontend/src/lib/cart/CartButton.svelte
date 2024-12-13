<script lang="ts">
  import { faShoppingCart } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";

  let { cartVisible = $bindable(false) } = $props();

  async function onShoppingCartClick() {
    cartVisible = !cartVisible;
  }
</script>

<div
  class="shopping-cart"
  class:visible={cartVisible}
  onclick={onShoppingCartClick}
  onkeydown={onShoppingCartClick}
  tabindex="0"
  role="button"
>
  <Fa icon={faShoppingCart} />
</div>

<style>
  .shopping-cart :global(svg) {
    width: 100%;
    height: 100%;
  }

  .shopping-cart.visible {
    color: var(--primary-color);
  }

  .shopping-cart {
    height: 40%;
    aspect-ratio: 1;

    cursor: pointer;
  }
</style>
