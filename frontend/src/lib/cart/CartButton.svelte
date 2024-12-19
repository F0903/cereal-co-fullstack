<script lang="ts">
    import Button from "$lib/generic/Button.svelte";
    import { faShoppingCart } from "@fortawesome/free-solid-svg-icons";
    import { cart } from "./cartStore.svelte";
    import { browser } from "$app/environment";

    let { cartVisible = $bindable(false) } = $props();

    async function onShoppingCartClick() {
        cartVisible = !cartVisible;
    }
</script>

{#if browser}
    <Button
        prefixIcon={faShoppingCart}
        onclick={onShoppingCartClick}
        --padding="15px 20px"
        --background-color="var(--primary-color)"
        >${cart.getDisplayableTotalSum()}</Button
    >
{/if}
