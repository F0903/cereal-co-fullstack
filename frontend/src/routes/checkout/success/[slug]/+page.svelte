<script lang="ts">
    import { goto } from "$app/navigation";
    import Button from "$lib/generic/Button.svelte";
    import type { PageData } from "./$types";

    let { data }: { data: PageData } = $props();
</script>

<div class="success-container">
    <h1>Success!</h1>
    <p>Thank you for your order.</p>
    <div class="return-button-container">
        <Button onclick={() => goto("/")}>Return</Button>
    </div>
</div>

<style>
    .return-button-container {
        margin-top: 15px;
    }

    .success-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        max-width: 500px;
        margin: auto;
        margin-top: 200px;
    }
</style>
