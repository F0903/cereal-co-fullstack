<script lang="ts">
    import type { UserInfo } from "../api/auth";
    import { faUserCircle } from "@fortawesome/free-solid-svg-icons";
    import Button from "../generic/Button.svelte";
    import { goto } from "$app/navigation";

    let { user }: { user: UserInfo } = $props();

    async function onUserClick() {
        await goto("/user");
    }
</script>

<Button
    prefixIcon={faUserCircle}
    onclick={onUserClick}
    --background-color="var(--primary-color)"
    --padding="15px"
>
    <span class="username" class:admin={user.is_admin}
        >{user.decorative_username}</span
    >
</Button>

<style>
    .username.admin {
        color: rgb(192, 48, 48);
    }

    .username {
        font-weight: 700;
        color: white;
    }
</style>
