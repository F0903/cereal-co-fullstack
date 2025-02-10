<script>
    import { faSearch } from "@fortawesome/free-solid-svg-icons";
    import Fa from "svelte-fa";

    let { search = $bindable() } = $props();
</script>

<div class="input-container">
    <div class="icon-container">
        <Fa icon={faSearch} />
    </div>
    <input type="text" placeholder="Search" bind:value={search} />
</div>

<style>
    input {
        border: none;
        outline: none;
        background-color: var(--primary-color);
        color: var(--tertiary-color);

        font-family: "Roboto Flex", sans-serif;
        font-size: 1em;
        width: fit-content;
    }

    .input-container {
        display: flex;
        flex-direction: row;
        gap: 10px;
        align-items: center;
        justify-content: center;
        background-color: var(--primary-color);

        width: fit-content;
        padding: 15px;

        border-radius: 10px;
    }
</style>
