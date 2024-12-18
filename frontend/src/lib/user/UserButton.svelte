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
    hoverAnimation={false}
    --background-color="var(--primary-color)"
    --padding="15px"
>
    <span class="name" class:admin={user.is_admin}>{user.name}</span>
</Button>

<style>
    .name.admin {
        color: rgb(192, 48, 48);
    }

    .name {
        font-weight: 700;
        color: white;
    }
</style>
