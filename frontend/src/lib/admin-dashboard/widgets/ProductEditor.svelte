<script lang="ts">
    import {
        deleteSingleProduct,
        getAllProducts,
        type Product,
        updateSingleProduct,
    } from "$lib/api/products";
    import Button from "$lib/generic/Button.svelte";
    import JsonTextEditor from "$lib/generic/JsonTextEditor.svelte";
    import Table from "$lib/generic/Table.svelte";
    import { cyrb53 } from "$lib/utils/hash";
    import {
        faSave,
        faTrashCan,
        faUndo,
    } from "@fortawesome/free-solid-svg-icons";
    import { onMount } from "svelte";
    import AddProductOverlay from "./AddProductOverlay.svelte";

    type EditableProduct = {
        product: Product;
        hash: number;
    };

    let changes = $state(false);
    let editables: EditableProduct[] = $state([]);
    let originalEditables: EditableProduct[] = [];
    let deletedEditables: EditableProduct[] = [];

    const stableStringify = (obj: any) =>
        JSON.stringify(obj, Object.keys(obj).sort());

    const hashObject = (obj: any) => cyrb53(stableStringify(obj));

    onMount(async () => {
        await resetProducts();
    });

    $effect(() => {
        editables;
        checkForChanges();
    });

    async function resetProducts() {
        const products = await getAllProducts();
        editables = [];
        products.forEach((product) => {
            editables.push({
                product,
                hash: hashObject(product),
            });
        });
        originalEditables = $state.snapshot(editables);
    }

    function checkForChanges() {
        for (const editable of editables) {
            const product = editable.product;
            const current_hash = hashObject(product);
            const changed = current_hash !== editable.hash;
            if (changed) {
                changes = true;
                return;
            }
        }

        changes = false;
    }

    async function updateChangedProducts() {
        for (const editable of editables) {
            const product = editable.product;
            const current_hash = hashObject(product);
            const changed = current_hash !== editable.hash;
            if (!changed) {
                continue;
            }

            await updateSingleProduct(product);
            editable.hash = hashObject(product); // Update hash
        }
    }

    async function deleteMarkedProducts() {
        for (const editable of deletedEditables) {
            await deleteSingleProduct(editable.product.id);
        }
    }

    async function submitChanges() {
        await Promise.all([updateChangedProducts(), deleteMarkedProducts()]);
        changes = false;
    }

    async function undoChanges() {
        editables = originalEditables;
        deletedEditables = [];
        changes = false;
    }

    async function deleteProduct(editable: EditableProduct) {
        editables.splice(
            editables.findIndex((x) => x.product.id === editable.product.id),
            1,
        );
        deletedEditables.push(editable);
        changes = true;
    }

    async function addProduct(product: Product) {
        let editable = {
            product,
            hash: hashObject(product),
        };
        editables.push(editable);
        changes = true;
    }
</script>

<!--TODO: ability to add elements-->
<!--TODO: full pop-up editor for description (like JSONEditor)-->
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
                <th class="date-header">Created At</th>
                <th class="date-header">Updated At</th>
                <th class="actions-header">Actions</th>
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
                            pattern="\d+"
                            bind:value={editable.product.price}
                        /></td
                    >
                    <td
                        ><input
                            type="text"
                            pattern="\d+"
                            bind:value={editable.product.quantity}
                        /></td
                    >
                    <td>
                        <JsonTextEditor
                            bind:jsonObject={editable.product.attributes}
                            --background-color="rgb(63, 63, 63)"
                        />
                    </td>
                    <td
                        ><span class="date">{editable.product.created_at}</span
                        ></td
                    >
                    <td
                        ><span class="date">{editable.product.updated_at}</span
                        ></td
                    >
                    <td class="actions-cell"
                        ><div class="actions">
                            <Button
                                --background-color="rgb(63, 63, 63)"
                                prefixIcon={faTrashCan}
                                onclick={() => deleteProduct(editable)}
                            />
                        </div></td
                    >
                </tr>
            {/each}
        </tbody>
    </Table>
    <div class="buttons">
        <Button
            disabled={!changes}
            prefixIcon={faSave}
            onclick={submitChanges}
            --background-color="var(--secondary-color)">Save Changes</Button
        >
        <Button
            disabled={!changes}
            prefixIcon={faUndo}
            onclick={undoChanges}
            --background-color="var(--secondary-color)">Undo Changes</Button
        >

        <AddProductOverlay onnewproduct={addProduct} />
    </div>
</div>

<style>
    :nth-last-child(-n + 1 of .date-header) {
        width: 200px;
    }

    .date-header {
        width: 150px;
    }

    .buttons {
        display: flex;
        flex-direction: row;
        justify-content: start;
        align-items: center;
        gap: 25px;
    }

    .actions-header {
        text-align: center;
    }

    .actions-cell {
        border-left: 1px solid hsl(0, 0%, 30%);
    }

    .actions {
        display: flex;
        flex-direction: row;
        justify-content: space-evenly;
        align-items: center;
    }

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
