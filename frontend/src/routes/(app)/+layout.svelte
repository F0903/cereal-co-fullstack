<script lang="ts">
  import { goto } from "$app/navigation";
  import Cart from "$lib/cart/CartView.svelte";
  import CartButton from "$lib/cart/CartButton.svelte";
  import { clickOutside } from "$lib/utils/clickOutside.svelte";
  import UserButton from "$lib/AuthButton.svelte";

  let { data, children } = $props();

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

    <div class="buttons-container">
      <div
        class="cart-button-container"
        use:clickOutside
        onclickoutside={() => (cartVisible = false)}
      >
        <CartButton bind:cartVisible />
        <div class="cart-container">
          <Cart bind:visible={cartVisible} />
        </div>
      </div>

      <div class="auth-button-container">
        <UserButton loggedIn={data.loggedIn} />
      </div>
    </div>
  </header>
</div>

{@render children()}

<style>
  .header-container {
    position: sticky;
    top: 0;
    z-index: 99;
  }

  .buttons-container {
    display: flex;
    flex-direction: row;
    height: 100%;
    gap: 25px;
    float: right;
    box-sizing: border-box;
    padding: 10px;
  }

  .cart-container {
    position: absolute;
    right: 35px;
    top: var(--header-height);
  }

  .cart-button-container {
    padding: 10px;
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
    padding: 0px 15px;
    box-sizing: border-box;

    background-color: var(--secondary-color);

    box-shadow: 0px 1px 25px 2px hsl(0, 0%, 10%);
  }
</style>
