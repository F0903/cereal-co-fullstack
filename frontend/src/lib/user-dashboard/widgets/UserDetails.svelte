<script lang="ts">
    import { goto } from "$app/navigation";
    import type { UserInfo } from "$lib/api/auth";
    import Button from "$lib/generic/Button.svelte";
    import Spacer from "$lib/generic/Spacer.svelte";
    import { faCog, faRefresh } from "@fortawesome/free-solid-svg-icons";

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
        <Spacer
            --margin-top="25px"
            --margin-bottom="5px"
            --color="var(--primary-color)"
        />
        <div class="buttons">
            <Button
                prefixIcon={faRefresh}
                onclick={() => goto("/user/change-password?redirect=/user")}
                --width="fit-content"
                --background-color="var(--primary-color)"
                >Change Password</Button
            >
            {#if user.is_admin}
                <Button
                    prefixIcon={faCog}
                    prefixIconColor="var(--admin-color)"
                    onclick={() => goto("/admin")}
                    --width="fit-content"
                    --background-color="var(--primary-color)"
                    >Admin Dashboard</Button
                >
            {/if}
        </div>
    </div>
</div>

<style>
    .buttons {
        padding: 0px 10px;
        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        gap: 10px;
        box-sizing: border-box;
    }

    .content-column {
        display: flex;
        flex-direction: column;
        gap: 5px;
        box-sizing: border-box;
        height: 100%;
    }

    .value-title {
        font-weight: 600;
        margin-right: 5px;
    }

    h2 {
        padding-top: 25px;
        margin-bottom: 25px;
        padding-bottom: 2px;
        font-size: 1.8em;
        width: fit-content;
    }

    .user-details {
        box-sizing: border-box;
        min-height: 95%;
    }
</style>
