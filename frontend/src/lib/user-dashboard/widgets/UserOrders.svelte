<script lang="ts">
    import type { UserInfo } from "$lib/api/auth";
    import { getOrdersByMail, OrderWithTimestamps } from "$lib/api/orders";
    import OrderView from "$lib/orders/OrderView.svelte";
    import { onMount } from "svelte";

    let { user }: { user: UserInfo } = $props();

    let orders: OrderWithTimestamps[] = $state([]);

    onMount(async () => {
        orders = await getOrdersByMail(user.mail);
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
        gap: 10px;
    }

    h2 {
        padding-top: 25px;
        margin-bottom: 25px;
        padding-bottom: 2px;
        font-size: 1.8em;
        width: fit-content;
    }
</style>
