<script lang="ts">
    import Image from "$lib/generic/Image.svelte";
    import { getFullImageUrl } from "$lib/api/utils";
    import Button from "$lib/generic/Button.svelte";
    import { goto } from "$app/navigation";
    import Spacer from "$lib/generic/Spacer.svelte";
    import Counter from "$lib/generic/Counter.svelte";
    import { cart } from "./cartStore.svelte";
    import { CartItem } from "./LocalCart.svelte";

    async function onCheckoutClick() {
        await goto("/checkout");
    }
</script>

<div class="cart-view">
    <Spacer
        --margin-top="0px"
        --margin-bottom="15px"
        --width="100%"
        --color="var(--primary-color)"
    />
    <div class="cart-items">
        {#each cart.getItems() as item}
            <div class="cart-item">
                <Image
                    src={getFullImageUrl(item.product.image_url)}
                    alt="Image of {item.product.name}"
                    --width="50px"
                    --height="50px"
                    --border-radius="7px"
                />
                <h4 class="product-name">{item.product.name}</h4>
                <div class="counter-container">
                    <Counter
                        value={item.quantity}
                        max_value={item.product.quantity}
                        on_value_changed={(newVal) => {
                            cart.modifyItem(
                                item.product.id,
                                (itm: CartItem) => (itm.quantity = newVal),
                            );
                        }}
                        on_negative_value_callback={() => {
                            cart.removeItem(item.product.id);
                        }}
                        --background-color="var(--secondary-color)"
                        --seperator-color="var(--tertiary-color)"
                    />
                </div>

                <span class="price-text"
                    >${(item.product.price * item.quantity).toFixed(2)}</span
                >
            </div>
        {/each}
    </div>

    {#if !cart.isEmpty()}
        <div class="total-price-container">
            <span class="total-text">Total:</span>
            <span class="price-text">${cart.calcSum().toFixed(2)}</span>
        </div>
        <div class="checkout-button-container">
            <Button onclick={onCheckoutClick}>Checkout</Button>
        </div>
    {:else}
        <span style="text-align: center; width: 100%;"
            >it's a little empty up here...</span
        >
    {/if}
</div>

<style>
    .counter-container {
        float: right;
        justify-self: flex-end;
    }

    .price-text {
        font-family: "Roboto Slab Variable";
        font-weight: 700;
        width: 50px;
        text-align: center;
    }

    .total-price-container {
        padding-top: 15px;
        font-size: 1.5em;
        border-bottom: 2px solid var(--tertiary-color);
        width: fit-content;
    }

    .checkout-button-container {
        margin-top: 15px;
    }

    .cart-items {
        overflow-y: scroll;
        flex-shrink: 10;
    }

    .product-name {
        text-align: center;
        width: 90px;
    }

    .cart-item {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        align-items: center;

        padding: 10px 15px;

        background-color: var(--primary-color);
    }

    .cart-view {
        --cart-max-height: 50vh;

        padding: 25px;
        padding-top: 0px;
        background-color: var(--secondary-color);

        border-radius: 10px;
        border-top-left-radius: 0px;
        border-top-right-radius: 0px;

        box-shadow: 0px 0px 25px 10px rgba(0, 0, 0, 0.2);
        /* Allow shadow around but not above*/
        clip-path: inset(0px -50px -50px -50px);

        min-width: 400px;
        max-height: var(--cart-max-height);

        display: flex;
        flex-direction: column;
    }
</style>
