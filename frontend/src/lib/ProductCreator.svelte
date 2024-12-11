<script lang="ts">
    import { addSingleProduct } from "./api/products_api";
    import {
        getAttributesOfProductType,
        product_types,
        Product,
    } from "./models/Product";
    import { faPlus, faArrowRight } from "@fortawesome/free-solid-svg-icons";
    import IconButton from "./IconButton.svelte";
    import Dropdown from "./Dropdown.svelte";
    import {
        findParentWithTag,
        getObjectFromFormData,
    } from "./utils/dom_utils";
    import InputField from "./InputField.svelte";
    import ErrorBox from "./ErrorBox.svelte";

    let { addProductCallback }: { addProductCallback: () => Promise<void> } =
        $props();

    let error: Error | null = $state(null);
    let creatorExpanded = $state(false);
    let selectedType = $state("");

    function onCreatorButtonClick() {
        creatorExpanded = !creatorExpanded;
    }

    async function submit(event: Event) {
        event.preventDefault();

        let form = findParentWithTag("form", event.target as HTMLElement);
        const product = getObjectFromFormData<Product>(form as HTMLFormElement);

        try {
            await addSingleProduct(product, selectedType);
            await addProductCallback();
        } catch (err) {
            error = err as Error;
        }
    }
</script>

<div class="creator-container" class:expanded={creatorExpanded}>
    <IconButton
        custom_class="creator-button"
        click_toggle_class="enabled"
        margin="40px"
        size="65px"
        fa_icon={faPlus}
        onclick={onCreatorButtonClick}
    />
    {#if creatorExpanded}
        <div class="creator">
            <h2 class="creator-title">Create Product</h2>

            <div class="creator-form-wrapper">
                {#snippet dropdown_content()}
                    {#each product_types as productType}
                        <option value={productType}>{productType}</option>
                    {/each}
                {/snippet}
                <Dropdown
                    name="Type"
                    placeholder="Select Type"
                    {dropdown_content}
                    bind:selected_value={selectedType}
                />

                <form class="creator-form">
                    {#if selectedType}
                        <div class="creator-form-inputs">
                            {#each getAttributesOfProductType(selectedType) as attr_name}
                                <InputField
                                    name={attr_name}
                                    has_input_error={error != null}
                                />
                            {/each}
                        </div>
                        <IconButton
                            fa_icon={faArrowRight}
                            onclick={submit}
                            margin="5px"
                            bg_color="var(--tertiary-color)"
                        />
                    {/if}
                </form>

                {#if error}
                    <ErrorBox message={error.message} />
                {/if}
            </div>
        </div>
    {/if}
</div>

<style>
    .creator-form-wrapper {
        display: flex;
        flex-direction: column;
        align-items: center;
        gap: 10px;
    }

    .creator-title {
        text-align: center;
    }

    .creator {
        display: flex;
        flex-direction: column;
        gap: 10px;

        padding: 15px;
        width: 300px;

        border-radius: 7px;
        background-color: var(--secondary-color);

        margin-bottom: 25px;
    }

    /* Custom class for the creator button */
    :global(.creator-button.enabled) {
        rotate: 45deg;
    }

    .creator-form {
        display: flex;
        flex-direction: column;
        gap: 5px;
        justify-content: center;
        align-items: center;
        width: 100%;
    }

    .creator-form-inputs {
        display: flex;
        flex-direction: column;
        gap: 5px;
        width: 100%;
        padding: 5px;
        box-sizing: border-box;
    }

    .creator-container {
        margin: 10px;
        display: flex;
        flex-direction: column;
        align-items: center;
    }
</style>
