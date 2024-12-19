<script lang="ts">
    import ProductEditor from "$lib/admin-dashboard/widgets/ProductEditor.svelte";
    import ListSelection from "$lib/generic/ListSelection.svelte";
    import type { PageData } from "./$types";

    let { data }: { data: PageData } = $props();

    let viewSelection = $state("Manage Products");
</script>

<div class="admin-view-container">
    <div class="admin-view-select-container">
        <ListSelection
            title="options"
            options={["Manage Products"]}
            bind:selection={viewSelection}
        />
    </div>
    {#if data.loggedIn}
        <div class="admin-view">
            {#if viewSelection === "Manage Products"}
                <ProductEditor />
            {/if}
        </div>
    {/if}
</div>

<style>
    .admin-view-select-container {
        width: 33%;
        max-width: 250px;
        float: left;
        box-sizing: border-box;
    }

    .admin-view {
        padding: 50px;
        padding-top: 15px;
        overflow-y: scroll;
        width: 100%;

        box-sizing: border-box;
    }

    .admin-view-container {
        background-color: var(--primary-color);

        min-height: 500px;
        height: calc(
            100% - 75px
        ); /* Minux header height (variable cannot be reached here) */
        width: 100%;

        box-sizing: border-box;

        display: flex;
        flex-direction: row;

        overflow: hidden;
    }
</style>
