<script lang="ts">
    import type { IconDefinition } from "@fortawesome/free-solid-svg-icons";
    import type { Snippet } from "svelte";
    import Fa from "svelte-fa";

    let {
        children,
        onclick,
        disabled,
        hoverAnimation = true,
        prefixIcon = undefined,
    }: {
        children: Snippet;
        onclick: (event: Event) => void;
        disabled?: boolean;
        hoverAnimation?: boolean;
        prefixIcon?: IconDefinition;
    } = $props();
</script>

<button
    class="button"
    class:hover-animation={hoverAnimation}
    class:disabled
    {onclick}
    onkeypress={onclick}
>
    {#if prefixIcon}
        <Fa icon={prefixIcon} />
    {/if}
    {@render children()}
</button>

<style>
    .button.hover-animation:hover {
        scale: 1.06;
    }

    .button.hover-animation {
        transition-property: scale;
        transition-timing-function: cubic-bezier(0.45, 0.05, 0.55, 0.95);
        transition-duration: 50ms;
    }

    .button:hover {
        cursor: pointer;
        filter: brightness(1.5);
    }

    .button.disabled {
        filter: brightness(0.5);
        pointer-events: none;
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
        margin: var(--margin);
        border: var(--border);
        height: var(--height);
        width: var(--width);

        box-sizing: border-box;
    }
</style>
