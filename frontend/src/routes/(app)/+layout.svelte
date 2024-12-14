<script lang="ts">
  import { goto } from "$app/navigation";
  import Cart from "$lib/cart/CartView.svelte";
  import CartButton from "$lib/cart/CartButton.svelte";
  import { clickOutside } from "$lib/clickOutside.svelte";

  let { children } = $props();

  let cartVisible = $state(false);

  async function onHomeClick() {
    await goto("/");
  }
</script>

<div class="header-container">
  <header>
    <div
      class="logo"
      onclick={onHomeClick}
      onkeydown={onHomeClick}
      tabindex="0"
      role="button"
    >
      <h1>Cereal Co.</h1>
    </div>

    <div class="cart-button-container">
      <CartButton bind:cartVisible />
    </div>
  </header>

  <div
    class="cart-container"
    use:clickOutside
    onclickoutside={() => (cartVisible = false)}
  >
    <Cart bind:visible={cartVisible} />
  </div>
</div>

{@render children()}

<style>
  .header-container {
    position: sticky;
    top: 0;
    z-index: 99;
  }

  .cart-container {
    position: fixed;
    right: 15px;
    top: var(--header-height);
  }

  .cart-button-container {
    position: absolute;
    top: 50%;
    right: 10px;
    transform: translate(-50%, -50%);

    height: 45%;
    aspect-ratio: 1;
  }

  .logo {
    cursor: pointer;
    text-align: center;
  }

  h1 {
    user-select: none;
    text-align: center;
    font-family: "Caveat Variable", cursive;
    font-size: 2.6em;
    font-weight: 100;
    margin: 0px;

    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
  }

  header {
    --header-height: 75px;

    position: relative;

    width: 100vw;
    height: var(--header-height);
    box-sizing: border-box;

    background-color: var(--secondary-color);

    box-shadow: 0px 1px 25px 2px hsl(0, 0%, 10%);
  }
</style>
