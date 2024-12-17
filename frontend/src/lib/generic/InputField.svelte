<script lang="ts">
    import { onMount } from "svelte";
    import type { FullAutoFill, HTMLInputTypeAttribute } from "svelte/elements";

    let {
        name,
        initial_value = undefined,
        input_type = "text",
        autocomplete = "off",
        has_input_error = false,
    }: {
        name: string;
        initial_value?: string;
        input_type?: HTMLInputTypeAttribute;
        autocomplete?: FullAutoFill;
        has_input_error?: boolean;
    } = $props();

    let input: HTMLInputElement;

    onMount(() => {
        if (initial_value) input.value = initial_value;
    });
</script>

<div class="input-field" class:error={has_input_error}>
    <label for={name}>{name}</label>
    <input type={input_type} {name} {autocomplete} bind:this={input} />
</div>

<style>
    input {
        border: 0;
        background-color: inherit;
        display: block;
        width: 100%;
        box-sizing: border-box;

        padding-top: 7px;

        font: inherit;
        color: inherit;
    }

    label {
        display: block;
        width: 100%;
        border-bottom: 2px solid
            var(--label-underline-color, var(--secondary-color));
        box-sizing: border-box;
        font-weight: var(--label-font-weight, 600);
        font-size: 1.15em;
        padding-bottom: 7px;
    }

    .input-field.error label,
    .input-field:has(input:invalid) {
        color: var(--error-color, hsl(0, 45%, 45%));
    }

    .input-field.error,
    .input-field:has(input:invalid) {
        border-left: 3px solid var(--error-color, hsl(0, 45%, 45%));
    }

    .input-field {
        color: var(--text-color, var(--tertiary-color));
        background-color: var(--bg-color, var(--primary-color));
        box-sizing: border-box;
        padding: 15px;
        border-radius: 5px;
        border: 2px dashed transparent;
    }
</style>
