<script lang="ts">
    import type { IconDefinition } from "@fortawesome/free-solid-svg-icons";
    import type { Snippet } from "svelte";
    import Fa from "svelte-fa";

    let {
        children,
        onclick,
        prefixIcon = undefined,
    }: {
        children: Snippet;
        onclick: () => void;
        prefixIcon?: IconDefinition;
    } = $props();
</script>

<div class="button" {onclick} onkeypress={onclick} role="button" tabindex="0">
    {#if prefixIcon}
        <Fa icon={prefixIcon} />
    {/if}
    {@render children()}
</div>

<style>
    .button:hover {
        cursor: pointer;
        filter: brightness(1.5);
    }

    .button {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
        gap: 5px;

        user-select: none;

        background-color: var(--background-color, var(--quaternary-color));
        color: hsl(0, 0%, 85%);
        font-family: "Roboto Flex Variable", sans-serif;
        font-size: var(--font-size, 1.1em);
        font-weight: 600;
        border-radius: var(--border-radius, 15px);
        padding: var(--padding, 10px);
        border: var(--border);
        height: var(--height);
        width: var(--width);

        box-sizing: border-box;
    }
</style>
