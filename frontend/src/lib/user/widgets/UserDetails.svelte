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
        <div class="username">
            <span class="value-title">Username:</span><span
                >{user.decorative_username}</span
            >
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
