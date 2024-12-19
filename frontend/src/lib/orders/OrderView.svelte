<script lang="ts">
    import Spacer from "$lib/generic/Spacer.svelte";
    import type { OrderWithTimestamps } from "../api/orders";
    import OrderTable from "./OrderTable.svelte";

    let { order }: { order: OrderWithTimestamps } = $props();
</script>

<div class="order-view">
    <h3 class="order-title">Order {order.order_id}</h3>
    <div class="value-column">
        <div class="order-value">
            <span class="value-title">Shipping Name</span>
            <span>{order.order_form.shipping_name}</span>
        </div>
        <div class="order-value">
            <span class="value-title">Shipping Mail</span>
            <span>{order.order_form.shipping_mail}</span>
        </div>
        <div class="order-value">
            <span class="value-title">Shipping Phone</span>
            <span>{order.order_form.shipping_phone}</span>
        </div>
        <div class="order-value">
            <span class="value-title">Shipping Address</span>
            <span>{order.order_form.shipping_address}</span>
        </div>
        <div class="order-value">
            <span class="value-title">Order Date</span>
            <span>{new Date(order.created_at).toUTCString()}</span>
        </div>
    </div>

    <Spacer --spacing="20px" --width="85%" --thickness="1px" />

    <div class="order-table">
        <h4 class="order-table-title">Order Items</h4>
        <OrderTable {order} />
    </div>

    <div class="total-container">
        <span class="total-title">Total: </span>
        <span class="total-text">${order.order_form.total}</span>
    </div>
</div>

<style>
    h4 {
        margin-bottom: 10px;
    }

    h3 {
        margin-bottom: 15px;
    }

    .total-text {
        font-family: "Roboto Slab Variable", serif;
        font-weight: 600;
    }

    .total-container {
        font-size: 1.4em;
        border-bottom: 2px solid var(--tertiary-color);
        width: fit-content;
        margin: auto;
    }

    .order-table-title {
        font-size: 1.2em;
    }

    .value-column {
        display: flex;
        flex-direction: column;
        gap: 5px;

        margin-bottom: 10px;
    }

    .value-title {
        font-weight: 600;
        margin-right: 5px;
    }

    .order-title {
        font-size: 1.5em;
        margin-top: 10px;
    }

    .order-view {
        background-color: var(--primary-color);
        padding: 25px;
        border-radius: 10px;
    }
</style>
