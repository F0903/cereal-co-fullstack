<script lang="ts">
    import {
        getAllProducts,
        updateSingleProduct,
        type Product,
    } from "$lib/api/products";
    import Button from "$lib/generic/Button.svelte";
    import JsonTextEditor from "$lib/generic/JsonTextEditor.svelte";
    import Table from "$lib/generic/Table.svelte";
    import { cyrb53 } from "$lib/utils/hash";
    import { faSave } from "@fortawesome/free-solid-svg-icons";
    import { onMount } from "svelte";

    type EditableProduct = {
        product: Product;
        start_hash: number;
    };

    let editables: EditableProduct[] = $state([]);
    let changes = $state(false);

    onMount(async () => {
        const products = await getAllProducts();
        products.forEach((product) => {
            editables.push({
                product,
                start_hash: cyrb53(JSON.stringify(product)),
            });
        });
    });

    $effect(() => {
        checkForChanges();
    });

    function checkForChanges() {
        for (const editable of editables) {
            const product = editable.product;
            const current_hash = cyrb53(JSON.stringify(product));
            const changed = current_hash !== editable.start_hash;
            if (changed) {
                changes = true;
                return;
            }
        }
        changes = false;
    }

    async function submitChangedProducts() {
        for (const editable of editables) {
            const product = editable.product;
            const current_hash = cyrb53(JSON.stringify(product));
            const changed = current_hash !== editable.start_hash;
            if (!changed) {
                continue;
            }
            await updateSingleProduct(product);

            // Update hash
            editable.start_hash = current_hash;
        }
        changes = false;
    }
</script>

<div class="product-editor">
    <Table>
        <thead>
            <tr>
                <th>Id</th>
                <th>Image Path</th>
                <th>Name</th>
                <th>Description</th>
                <th>Manufacturer</th>
                <th>Price</th>
                <th>Quantity</th>
                <th>Attributes</th>
                <th>Created At</th>
                <th>Updated At</th>
            </tr>
        </thead>
        <tbody>
            {#each editables as editable}
                <tr>
                    <td>
                        {editable.product.id}
                    </td>
                    <td>
                        <input
                            class="very-wide-input"
                            type="text"
                            bind:value={editable.product.image_url}
                        />
                    </td>
                    <td
                        ><input
                            class="wide-input"
                            type="text"
                            bind:value={editable.product.name}
                        /></td
                    >
                    <td
                        ><input
                            class="very-wide-input"
                            type="text"
                            bind:value={editable.product.description}
                        /></td
                    >
                    <td
                        ><input
                            class="wide-input"
                            type="text "
                            bind:value={editable.product.manufacturer}
                        /></td
                    >
                    <td
                        ><input
                            type="text"
                            bind:value={editable.product.price}
                        /></td
                    >
                    <td
                        ><input
                            type="text"
                            bind:value={editable.product.quantity}
                        /></td
                    >
                    <td>
                        <JsonTextEditor
                            bind:jsonObject={editable.product.attributes}
                            --background-color="rgb(63, 63, 63)"
                        />
                    </td>
                    <td>{editable.product.created_at}</td>
                    <td>{editable.product.updated_at}</td>
                </tr>
            {/each}
        </tbody>
    </Table>
    <div class="buttons">
        <Button
            disabled={!changes}
            prefixIcon={faSave}
            onclick={submitChangedProducts}
            --background-color="var(--secondary-color)">Save Changes</Button
        >
    </div>
</div>

<style>
    .very-wide-input {
        min-width: 300px;
    }

    .wide-input {
        min-width: 200px;
    }

    input {
        width: 100%;
        background-color: rgb(63, 63, 63);
        border-radius: 10px;
        padding: 5px;
        border: 0;
        color: var(--tertiary-color);
    }

    .product-editor {
        width: 100%;
        height: 100%;

        display: flex;
        flex-direction: column;
    }
</style>
