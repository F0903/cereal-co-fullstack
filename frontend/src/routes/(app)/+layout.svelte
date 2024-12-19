<script lang="ts">
    import { goto } from "$app/navigation";
    import Cart from "$lib/cart/CartView.svelte";
    import CartButton from "$lib/cart/CartButton.svelte";
    import { clickOutside } from "$lib/utils/clickOutside.svelte";
    import { fly } from "svelte/transition";
    import Header from "$lib/generic/Header.svelte";
    import AuthContainer from "$lib/auth/AuthContainer.svelte";

    let { data, children } = $props();

    let cartVisible = $state(false);

    let cartContainer: HTMLDivElement | undefined = $state(undefined);

    async function onHomeClick() {
        await goto("/");
    }
</script>

<Header>
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
        <AuthContainer loggedIn={data.loggedIn} user={data.currentUser!} />
        <div
            class="cart-container"
            use:clickOutside
            onclickoutside={() => (cartVisible = false)}
        >
            <CartButton bind:cartVisible />
            {#if cartVisible}
                <div
                    class="cart-view-container"
                    bind:this={cartContainer}
                    transition:fly={{
                        duration: 250,
                        y: -cartContainer.clientHeight,
                    }}
                >
                    <Cart />
                </div>
            {/if}
        </div>
    </div>
</Header>
{@render children()}

<style>
    .buttons-container {
        display: flex;
        flex-direction: row;
        gap: 25px;

        height: 100%;
        padding: 10px;
        float: right;
        box-sizing: border-box;
    }

    .cart-view-container {
        position: absolute;
        right: 35px;
        top: var(--header-height);
        z-index: -1;
    }

    .cart-container {
        display: contents;
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
</style>
