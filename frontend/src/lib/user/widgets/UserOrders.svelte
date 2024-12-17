<script lang="ts">
    import type { UserInfo } from "$lib/api/auth";
    import { getOrdersByFilter, OrderWithTimestamps } from "$lib/api/orders";
    import OrderView from "$lib/orders/OrderView.svelte";
    import { onMount } from "svelte";

    let { user }: { user: UserInfo } = $props();

    let orders: OrderWithTimestamps[] = $state([]);

    onMount(async () => {
        orders = await getOrdersByFilter(user.mail);
    });
</script>

<div class="user-details">
    <h2>User Orders</h2>
    <div class="content-column">
        {#each orders as order}
            <OrderView {order} />
        {/each}
    </div>
</div>

<style>
    .content-column {
        display: flex;
        flex-direction: column;
        gap: 5px;
    }

    h2 {
        margin-bottom: 20px;
        padding-bottom: 2px;
        border-bottom: 2px solid var(--tertiary-color);
        width: fit-content;
    }
</style>
