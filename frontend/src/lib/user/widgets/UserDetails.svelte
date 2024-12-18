<script lang="ts">
    import type { UserInfo } from "$lib/api/auth";

    let { user }: { user: UserInfo } = $props();
</script>

<div class="user-details">
    <h2>User Details</h2>
    <div class="content-column">
        <div class="mail">
            <span class="value-title">Mail:</span><span>{user.mail}</span>
        </div>
        <div class="name">
            <span class="value-title">Name:</span><span>{user.name}</span>
        </div>
        <div class="address">
            <span class="value-title">Address:</span><span>{user.address}</span>
        </div>
        <div class="phone">
            <span class="value-title">Phone:</span><span>{user.phone}</span>
        </div>
    </div>
</div>

<style>
    .content-column {
        display: flex;
        flex-direction: column;
        gap: 5px;
    }

    .value-title {
        font-weight: 600;
        margin-right: 5px;
    }

    h2 {
        margin-bottom: 20px;
        padding-bottom: 2px;
        border-bottom: 2px solid var(--tertiary-color);
        width: fit-content;
    }
</style>
