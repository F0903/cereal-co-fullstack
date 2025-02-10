<script lang="ts">
    import {
        addSingleProduct,
        deleteSingleProduct,
        getAllProducts,
        getSingleProduct,
        ProductForm,
        updateSingleProduct,
        type Product,
    } from "$lib/api/products";
    import Button from "$lib/generic/Button.svelte";
    import JsonTextEditor from "$lib/generic/JsonTextEditor.svelte";
    import Table from "$lib/generic/Table.svelte";
    import { cyrb53 } from "$lib/utils/hash";
    import {
        faAdd,
        faSave,
        faTrashCan,
        faUndo,
    } from "@fortawesome/free-solid-svg-icons";
    import { onMount, tick } from "svelte";

    type EditableProduct = {
        new_product: boolean;
        product: Product | ProductForm;
        hash: number;
    };

    let changes = $state(false);
    let editables: EditableProduct[] = $state([]);
    let originalEditables: EditableProduct[] = [];
    let deletedEditables: EditableProduct[] = [];

    let table_div: HTMLDivElement | undefined = $state();

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

    async function addProduct() {
        const product: ProductForm = {
            name: "",
            description: "",
            manufacturer: "",
            price: 0,
            quantity: 0,
            image_url: "",
            attributes: {},
        };
        editables.push({
            product,
            hash: hashObject(product),
            new_product: true,
        });
        changes = true;

        await tick(); // Wait for the DOM to update so we can correctly scroll all the way down.
        table_div!.scroll({
            top: table_div!.scrollHeight,
            behavior: "smooth",
        });
    }

    async function resetProducts() {
        const products = await getAllProducts();
        editables = [];
        products.forEach((product) => {
            editables.push({
                product,
                hash: hashObject(product),
                new_product: false,
            });
        });

        // Grab a snapshot of the current editables to restore later for undo.
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

            if (editable.new_product) {
                console.log("Adding new product", $state.snapshot(product));
                const inserted_id = await addSingleProduct(product);
                const full_product = await getSingleProduct(inserted_id);
                editable.product = full_product;
                editable.new_product = false;
                console.log($state.snapshot(editable));
            } else {
                // product is guaranteed to be of type Product here.
                await updateSingleProduct(product as Product);
            }

            editable.hash = hashObject(product); // Update hash
        }
    }

    async function deleteMarkedProducts() {
        for (const editable of deletedEditables) {
            if (editable.new_product) {
                console.log("Marked product was a new product. Skipping...");
                continue;
            }

            console.log(
                "Marked product was not a new product. Sending delete...",
            );
            const product = editable.product as Product;
            await deleteSingleProduct(product.id);
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

    async function onProductDelete(editable: EditableProduct) {
        console.log("Deleting product", $state.snapshot(editable));

        editables.splice(
            editables.findIndex((x) => hashObject(x) === hashObject(editable)),
            1,
        );

        if (editable.new_product) {
            console.log(
                "Product to delete is a new product.",
                $state.snapshot(editable),
            );
            return;
        }
        deletedEditables.push(editable);
        changes = true;
    }
</script>

<!--NOTE: this editor was quickly thrown together the day before the project was due, it is in desperate need of a refactor-->
<!--TODO: full pop-up editor for description (like JSONEditor)-->
<div class="product-editor">
    <Table bind:self={table_div}>
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
                        {#if editable.new_product}
                            N/A
                        {:else}
                            {(editable.product as Product).id}
                        {/if}
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
                            type="number"
                            bind:value={editable.product.price}
                        /></td
                    >
                    <td
                        ><input
                            type="number"
                            bind:value={editable.product.quantity}
                        /></td
                    >
                    <td>
                        <JsonTextEditor
                            bind:jsonObject={editable.product.attributes}
                            --background-color="rgb(63, 63, 63)"
                        />
                    </td>
                    <td>
                        {#if editable.new_product}
                            N/A
                        {:else}
                            <span class="date"
                                >{(editable.product as Product)
                                    .created_at}</span
                            >
                        {/if}
                    </td>
                    <td>
                        {#if editable.new_product}
                            N/A
                        {:else}
                            <span class="date"
                                >{(editable.product as Product)
                                    .updated_at}</span
                            >
                        {/if}</td
                    >
                    <td class="actions-cell"
                        ><div class="actions">
                            <Button
                                --background-color="rgb(63, 63, 63)"
                                prefixIcon={faTrashCan}
                                onclick={() => onProductDelete(editable)}
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
        <Button
            prefixIcon={faAdd}
            onclick={addProduct}
            --background-color="var(--secondary-color)">Add Product</Button
        >
    </div>
</div>

<style>
    /* Chrome, Safari, Edge, Opera */
    input::-webkit-outer-spin-button,
    input::-webkit-inner-spin-button {
        -webkit-appearance: none;
        margin: 0;
    }

    /* Firefox */
    input[type="number"] {
        -moz-appearance: textfield;
        appearance: textfield;
    }

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
