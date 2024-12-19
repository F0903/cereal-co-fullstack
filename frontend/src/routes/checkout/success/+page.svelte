<script lang="ts">
    import { goto } from "$app/navigation";
    import Button from "$lib/generic/Button.svelte";
</script>

<div class="success-wrapper">
    <div class="success-container">
        <h1>Success!</h1>
        <p>Thank you for your order.</p>
        <div class="return-button-container">
            <Button onclick={() => goto("/")}>Return</Button>
        </div>
    </div>
</div>

<style>
    h1 {
        width: fit-content;
        border-bottom: 3px solid var(--tertiary-color);
    }

    .return-button-container {
        margin-top: 15px;
    }

    .success-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 25px;

        max-width: 500px;
        margin: auto;
    }

    .success-wrapper {
        padding-top: 150px;
    }
</style>
