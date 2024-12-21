<script lang="ts">
    import { page } from "$app/state";
    import HorizontalSpacer from "$lib/generic/HorizontalSpacer.svelte";
</script>

<div class="error">
    <h1 class="code">
        {page.status}
    </h1>
    <HorizontalSpacer
        --spacing="10px"
        --thickness="2px"
        --height="3em"
        --color="var(--quaternary-color)"
    />
    <span class="message">{page.error?.message}</span>
</div>

<style>
    .code {
        font-size: 4em;
        font-weight: 350;
        font-family: "Roboto Mono Variable", monospace;
        padding-bottom: 5px;
    }

    .message {
        font-size: 1.8em;
        font-weight: 200;
    }

    .error {
        position: absolute;
        top: 40%;
        left: 50%;
        transform: translate(-50%, -50%);

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        gap: 5px;
    }
</style>
