<script lang="ts">
    import { goto } from "$app/navigation";
    import type { Product } from "../api/products";
    import { getFullImageUrl } from "../api/utils";
    import Button from "../generic/Button.svelte";
    import { addToCart } from "../cart/localCartApi";
    import ClickableImage from "../generic/ClickableImage.svelte";
    import Spacer from "../generic/Spacer.svelte";

    let { product }: { product: Product } = $props();

    function onImageClick() {
        goto(`/product/${product.id}`);
    }

    function onAddClick() {
        addToCart({ product, quantity: 1 });
    }
</script>

<div class="card">
    <ClickableImage
        src={getFullImageUrl(product.image_url)}
        alt="Image of {product.name}"
        onclick={onImageClick}
    />
    <h2 class="name">{product.name}</h2>
    <Spacer --color="hsl(0, 0%, 25%)" --margin-top="auto" />
    <div class="buy-container">
        <span class="price">${product.price}</span>
        <Button onclick={onAddClick}>Add</Button>
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
        padding-top: 5px;
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
        border-radius: 25px;
        box-sizing: border-box;
    }
</style>
