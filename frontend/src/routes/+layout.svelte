<script lang="ts">
    import "../fonts.css";

    let { children } = $props();
</script>

{@render children()}

<style>
    :global(body) {
        width: 100vw;
        height: 100vh;
        box-sizing: border-box;
        margin: 0;
    }

    :global(*) {
        margin: 0;
        padding: 0;
        box-sizing: border-box;
    }

    :root {
        --primary-color: hsl(0, 0%, 15%);
        --secondary-color: hsl(0, 0%, 20%);
        --tertiary-color: hsl(0, 0%, 55%);
        --quaternary-color: hsl(231, 48%, 36%);

        --error-color: hsl(0, 45%, 45%);

        --admin-color: hsl(0, 60%, 47%);

        background-color: var(--primary-color);
        color: var(--tertiary-color);

        font-family: "Roboto Flex Variable", sans-serif;
    }
</style>
