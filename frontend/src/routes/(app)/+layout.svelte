<script lang="ts">
    import { goto } from "$app/navigation";
    import Cart from "$lib/cart/CartView.svelte";
    import CartButton from "$lib/cart/CartButton.svelte";
    import { clickOutside } from "$lib/utils/clickOutside.svelte";
    import AuthButton from "$lib/user/AuthButton.svelte";
    import { fly, slide } from "svelte/transition";
    import UserContainer from "$lib/user/UserContainer.svelte";
    import Header from "$lib/generic/Header.svelte";

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
        <div
            class="cart-button-container"
            use:clickOutside
            onclickoutside={() => (cartVisible = false)}
        >
            <CartButton bind:cartVisible />
            {#if cartVisible}
                <div
                    class="cart-container"
                    bind:this={cartContainer}
                    transition:fly={{
                        duration: 250,
                        y: -cartContainer.clientHeight,
                    }}
                >
                    <Cart bind:visible={cartVisible} />
                </div>
            {/if}
        </div>

        <div class="auth-container">
            {#if data.loggedIn}
                <UserContainer user={data.currentUser!} />
            {/if}
            <AuthButton loggedIn={data.loggedIn} />
        </div>
    </div>
</Header>
{@render children()}

<style>
    .auth-container {
        width: 100%;
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
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
        z-index: -1;
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
</style>
