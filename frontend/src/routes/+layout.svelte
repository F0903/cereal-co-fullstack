<script lang="ts">
    import "../fonts.css";

    let { children } = $props();
</script>

{@render children()}

<style>
    :root {
        --primary-color: hsl(0, 0%, 15%);
        --secondary-color: hsl(0, 0%, 20%);
        --tertiary-color: hsl(0, 0%, 55%);
        --quaternary-color: hsl(231, 48%, 36%);
        --error-color: hsl(0, 45%, 45%);

        background-color: var(--primary-color);
        color: var(--tertiary-color);

        font-family: "Roboto Flex Variable", sans-serif;
    }
</style>
