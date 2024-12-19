<script lang="ts">
    import ListSelection from "$lib/generic/ListSelection.svelte";
    import UserDetails from "$lib/user-dashboard/widgets/UserDetails.svelte";
    import UserOrders from "$lib/user-dashboard/widgets/UserOrders.svelte";
    import type { PageData } from "./$types";

    let { data }: { data: PageData } = $props();

    let viewSelection = $state("Details");
</script>

<div class="user-view-container">
    <div class="user-view-select-container">
        <ListSelection
            title="options"
            options={["Details", "Orders"]}
            bind:selection={viewSelection}
        />
    </div>
    {#if data.loggedIn}
        <div class="user-view">
            {#if viewSelection === "Details"}
                <UserDetails user={data.currentUser!} />
            {:else if viewSelection === "Orders"}
                <UserOrders user={data.currentUser!} />
            {/if}
        </div>
    {/if}
</div>

<style>
    .user-view-select-container {
        width: 33%;
        float: left;

        box-sizing: border-box;
    }

    .user-view {
        padding: 50px;
        padding-top: 15px;
        overflow-y: scroll;
        width: 100%;

        box-sizing: border-box;
    }

    .user-view-container {
        background-color: var(--secondary-color);

        margin-left: auto;
        margin-right: auto;
        margin-top: 50px;

        min-height: 500px;
        max-height: 85vh;
        max-width: 1000px;

        border-radius: 15px;

        box-sizing: border-box;

        display: flex;
        flex-direction: row;
        align-items: stretch;

        overflow: hidden;
    }
</style>
