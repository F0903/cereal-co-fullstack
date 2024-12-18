<script lang="ts">
    import type { UserInfo } from "$lib/api/auth";
    import HorizontalSpacer from "$lib/generic/HorizontalSpacer.svelte";
    import AuthButton from "./AuthButton.svelte";
    import UserButton from "./UserButton.svelte";

    let { loggedIn, user }: { loggedIn: boolean; user: UserInfo } = $props();
</script>

<div class="auth-container">
    {#if loggedIn}
        <UserButton {user} />
        <HorizontalSpacer --height="60%" --color="var(--secondary-color)" />
    {/if}
    <AuthButton {loggedIn} />
</div>

<style>
    .auth-container {
        width: 100%;
        display: flex;
        padding: 5px;
        border-radius: 15px;

        background-color: var(--primary-color);

        flex-direction: row;
        justify-content: center;
        align-items: center;
    }
</style>
