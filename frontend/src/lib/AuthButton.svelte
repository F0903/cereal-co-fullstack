<script lang="ts">
  import {
    faLock,
    faLockOpen,
    type IconDefinition,
  } from "@fortawesome/free-solid-svg-icons";
  import Button from "./Button.svelte";
  import { goto } from "$app/navigation";
  import { logout } from "./api/auth";

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
    {text}
    {prefixIcon}
    {onclick}
    --background-color="var(--primary-color)"
    --padding="25px"
    --border="2px solid var(--secondary-color)"
    --height="100%"
  />
{/snippet}

{#if loggedIn}
  {@render button("Log out", faLock, onLogOutClick)}
{:else}
  {@render button("Log in", faLockOpen, onLogInClick)}
{/if}

<style>
</style>
