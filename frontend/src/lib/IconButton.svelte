<script lang="ts">
    import Fa from "svelte-fa";
    let {
        fa_icon,
        onclick,
        size = "50px",
        padding = "13px",
        bg_color = "var(--secondary-color)",
        margin = "",
        custom_class = "",
        click_toggle_class = "",
    }: {
        fa_icon: any;
        onclick: (event: Event) => void;
        size?: string;
        padding?: string;
        bg_color?: string;
        margin?: string;
        custom_class?: string;
        click_toggle_class?: string;
    } = $props();

    let self: HTMLElement;

    function onClick(event: Event) {
        if (click_toggle_class) {
            self.classList.toggle(click_toggle_class);
        }
        onclick(event);
    }
</script>

<button
    bind:this={self}
    class="icon-button {custom_class}"
    onclick={onClick}
    style="width: {size}; padding: {padding}; margin: {margin}; background-color: {bg_color};"
>
    <Fa icon={fa_icon} style="width: 100%; height: 100%" />
</button>

<style>
    .icon-button:hover {
        filter: brightness(2.5);
    }

    .icon-button {
        background-color: var(--secondary-color);
        color: var(--primary-text-color);
        border: 0;
        margin: 0;

        box-sizing: border-box;

        aspect-ratio: 1/1;

        text-align: center;
        border-radius: 50%;
    }
</style>
