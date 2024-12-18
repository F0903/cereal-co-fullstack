<script lang="ts">
    import {
        faLock,
        faLockOpen,
        type IconDefinition,
    } from "@fortawesome/free-solid-svg-icons";
    import Button from "../generic/Button.svelte";
    import { goto } from "$app/navigation";
    import { logout } from "../api/auth";

    let { loggedIn } = $props();

    async function onLogInClick() {
        await goto("/login");
    }

    async function onLogOutClick() {
        await logout();
        await goto("/", { invalidateAll: true });
    }
</script>

{#snippet button(text: string, prefixIcon: IconDefinition, onclick: () => void)}
    <Button
        {prefixIcon}
        {onclick}
        hoverAnimation={false}
        --background-color="var(--primary-color)"
        --padding="25px"
        --height="100%">{text}</Button
    >
{/snippet}

{#if loggedIn}
    {@render button("Log out", faLock, onLogOutClick)}
{:else}
    {@render button("Log in", faLockOpen, onLogInClick)}
{/if}

<style>
</style>
