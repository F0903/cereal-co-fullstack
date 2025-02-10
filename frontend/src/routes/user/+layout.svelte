<script lang="ts">
    import { goto } from "$app/navigation";
    import AuthButton from "$lib/auth/AuthButton.svelte";
    import Button from "$lib/generic/Button.svelte";
    import { faHome } from "@fortawesome/free-solid-svg-icons";
    import type { Snippet } from "svelte";
    import type { LayoutData } from "./$types";
    import Header from "$lib/generic/Header.svelte";

    let { children, data }: { children: Snippet; data: LayoutData } = $props();
</script>

<Header>
    {#snippet left()}{/snippet}
    {#snippet center()}
        <h1>User Dashboard</h1>
    {/snippet}
    {#snippet right()}
        <div class="buttons-container">
            <Button
                prefixIcon={faHome}
                onclick={() => goto("/")}
                --background-color="var(--primary-color)">Home</Button
            >
            <AuthButton loggedIn={data.loggedIn} />
        </div>
    {/snippet}
</Header>
{@render children()}

<style>
    .buttons-container {
        display: flex;
        flex-direction: row;
        height: 100%;
        gap: 25px;
        float: right;
        box-sizing: border-box;
        padding: 10px;
    }

    h1 {
        margin: 0px;
        position: absolute;
        top: 50%;
        left: 50%;
        transform: translate(-50%, -50%);

        font-weight: 400;
    }
</style>
