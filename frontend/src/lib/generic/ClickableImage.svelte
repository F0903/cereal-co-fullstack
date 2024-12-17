<script lang="ts">
    import Fa from "svelte-fa";
    import { faMagnifyingGlass } from "@fortawesome/free-solid-svg-icons";

    let { src, alt, onclick } = $props();
</script>

<div
    class="image-container"
    {onclick}
    onkeypress={onclick}
    role="button"
    tabindex="0"
>
    <img {src} {alt} />
    <Fa icon={faMagnifyingGlass} class="clickable-image-icon" />
</div>

<style>
    .image-container:hover :global(.clickable-image-icon) {
        opacity: 1;
    }

    :global(.clickable-image-icon) {
        opacity: 0;
        position: absolute;
        top: 50%;
        left: 50%;
        scale: 7;
        color: hsla(0, 0%, 30%, 0.7);

        transition-property: opacity;
        transition-duration: 150ms;
        transition-timing-function: ease-in;
    }

    .image-container:hover img {
        scale: 1.1;
        filter: blur(7px);
    }

    img {
        width: 100%;
        height: 100%;
        object-fit: cover;
        object-position: center;

        transition-property: scale, filter;
        transition-duration: 100ms;
        transition-timing-function: ease-in;
    }

    .image-container:hover {
        cursor: pointer;
    }

    .image-container {
        position: relative;
        width: 100%;
        height: 250px;
        border-radius: 15px;
        overflow: hidden;
        box-sizing: border-box;
    }
</style>
