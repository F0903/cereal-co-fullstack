<script lang="ts">
    import type { Snippet } from "svelte";

    let {
        selected_value = $bindable(),
        dropdown_content,
        name = "",
        placeholder = "",
    }: {
        selected_value: string;
        dropdown_content: Snippet;
        name?: string;
        placeholder?: string;
    } = $props();
</script>

<select {name} bind:value={selected_value}>
    <option value="" selected disabled>{placeholder}</option>
    {@render dropdown_content()}
</select>

<style>
    select {
        background-color: var(--bg-color, var(--tertiary-color));
        border: 5px solid var(--bg-color, var(--tertiary-color));
        border-radius: 5px;
        padding: 7px;
        color: var(--primary-text-color);
        font-weight: 600;
        font-size: var(--font-size, 0.95em);
    }
</style>
