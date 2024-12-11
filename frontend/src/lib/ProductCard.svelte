<script lang="ts">
    import { deleteSingleProduct, setSingleProduct } from "./api/products_api";
    import IconButton from "./IconButton.svelte";
    import type { DatabaseProduct } from "./models/DatabaseProduct";
    import { constructProductFromAttributeList } from "./models/Product";
    import ProductAttribute from "./ProductAttribute.svelte";
    import {
        faSave,
        faTrash,
        faEdit,
        faCancel,
    } from "@fortawesome/free-solid-svg-icons";

    let { db_product }: { db_product: DatabaseProduct } = $props();

    const descriptor = db_product.Descriptor;
    let product = db_product.Product;

    // Create attributes objects from each property on product.
    let attributes: Attribute[] = $state([]);
    let oldAttributes: Attribute[] = [];

    Object.entries(product).forEach((attribute) => {
        attributes.push({ Name: attribute[0], Value: attribute[1] });
    });

    let editable = $state(false);

    async function deleteSelf() {
        await deleteSingleProduct(descriptor.ID);
        htmlRoot.remove();
    }

    async function startEditing() {
        oldAttributes = $state.snapshot(attributes);
        console.log(oldAttributes);
        editable = true;
    }

    async function saveEdits() {
        editable = false;
        let newProduct = constructProductFromAttributeList(
            descriptor.Type,
            attributes,
        );
        await setSingleProduct(descriptor.ID, newProduct, descriptor.Type);
        product = newProduct;
    }

    async function discardEdits() {
        editable = false;

        // This just refuses to work, despite other values than this will actually work.
        attributes = oldAttributes;
    }

    let htmlRoot: HTMLElement;
</script>

<div class="product" bind:this={htmlRoot}>
    {#each attributes as _, i}
        <ProductAttribute bind:attribute={attributes[i]} {editable} />
    {/each}
    <div class="descriptor-section">
        <h2 class="descriptor-title">Metadata</h2>
        <span>ID: {descriptor.ID}</span>
        <span>Type: {descriptor.Type}</span>
        <span>Created at: {descriptor.CreatedAt}</span>
        <span>Last updated at: {descriptor.LastUpdatedAt}</span>
    </div>
    <div class="buttons">
        <IconButton
            fa_icon={faTrash}
            onclick={deleteSelf}
            size="50px"
            bg_color="var(--tertiary-color)"
        />
        {#if editable}
            <IconButton
                fa_icon={faSave}
                onclick={saveEdits}
                size="50px"
                bg_color="var(--tertiary-color)"
            />
            <IconButton
                fa_icon={faCancel}
                onclick={discardEdits}
                size="50px"
                bg_color="var(--tertiary-color)"
            />
        {:else}
            <IconButton
                fa_icon={faEdit}
                onclick={startEditing}
                size="50px"
                bg_color="var(--tertiary-color)"
            />
        {/if}
    </div>
</div>

<style>
    .buttons {
        margin-top: 25px;
        height: auto;
        place-self: center end;
        display: flex;
        flex-direction: row;
        gap: 15px;
    }

    .descriptor-title {
        text-align: center;
        font-size: 1.2em;
        margin: 5px;
        padding-bottom: 5px;
        border-bottom: 1px solid var(--secondary-color);
    }

    .descriptor-section {
        margin-top: 10px;
        display: flex;
        flex-direction: column;
        gap: 2px;
        font-size: 0.9em;

        background-color: var(--tertiary-color);
        padding: 10px;
        border-radius: 5px;
    }

    .product {
        display: flex;
        flex-direction: column;
        background-color: var(--secondary-color);
        width: fit-content;
        padding: 25px;
        border-radius: 20px;

        box-sizing: border-box;
    }
</style>
