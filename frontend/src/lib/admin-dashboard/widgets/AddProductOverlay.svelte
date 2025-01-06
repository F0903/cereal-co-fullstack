<script lang="ts">
    import type { Product, ProductContent } from "$lib/api/products";
    import BlurredOverlay from "$lib/generic/BlurredOverlay.svelte";
    import Button from "$lib/generic/Button.svelte";
    import InputField from "$lib/generic/InputField.svelte";
    import JsonTextEditor from "$lib/generic/JsonTextEditor.svelte";
    import { faAdd, faClose } from "@fortawesome/free-solid-svg-icons";

    let { onnewproduct }: { onnewproduct: (product: ProductContent) => void } =
        $props();

    let visible = $state(false);

    let productName = $state(undefined);
    let productDescription = $state(undefined);
    let productManufacturer = $state(undefined);
    let productQuantity = $state(undefined);
    let productPrice = $state(undefined);
    let productImageURL = $state(undefined);
    let productAttributes = $state(undefined);

    function constructProduct() {
        if (productName == null) throw new Error("Product name is required");
        if (productDescription == null)
            throw new Error("Product description is required");
        if (productManufacturer == null)
            throw new Error("Product manufacturer is required");
        if (productQuantity == null)
            throw new Error("Product quantity is required");
        if (productPrice == null) throw new Error("Product price is required");
        if (productImageURL == null)
            throw new Error("Product image URL is required");
        if (productAttributes == null)
            throw new Error("Product attributes are required");

        let product: ProductContent = {
            name: productName,
            description: productDescription,
            manufacturer: productManufacturer,
            quantity: productQuantity,
            price: productPrice,
            image_url: productImageURL,
            attributes: productAttributes,
        };
    }
</script>

{#if !visible}
    <Button
        prefixIcon={faAdd}
        onclick={() => (visible = !visible)}
        --background-color="var(--secondary-color)"
        >Add Product
    </Button>
{:else}
    <BlurredOverlay>
        <div class="add-product-overlay">
            <div class="product-constructor">
                <InputField name="Name" />
                <InputField name="Description" />
                <InputField name="Manufacturer" />
                <InputField name="Quantity" />
                <InputField name="Price" />
                <InputField name="Image URL" />
                <JsonTextEditor
                    bind:jsonObject={productAttributes}
                    buttonText="Edit Attributes"
                />
                <input type="text" placeholder="Name" />
                <input type="text" placeholder="Description" />
                <input type="number" placeholder="Product Price" />
                <input type="text" placeholder="Product Image URL" />
                <Button
                    onclick={constructProduct}
                    --background-color="var(--secondary-color)"
                    >Add Product
                </Button>
            </div>
            <Button
                prefixIcon={faClose}
                onclick={() => (visible = !visible)}
                --background-color="var(--secondary-color)"
                >Close
            </Button>
        </div>
    </BlurredOverlay>
{/if}

<style>
    .product-constructor {
        display: flex;
        flex-direction: column;
        gap: 10px;
    }
</style>
