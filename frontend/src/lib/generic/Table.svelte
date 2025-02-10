<script lang="ts">
    let { self = $bindable(), children } = $props();
</script>

<div class="table-container" bind:this={self}>
    <table>
        {@render children()}
    </table>
</div>

<style>
    .table-container {
        width: 100%;
        height: fit-content;
        max-height: var(--max-height);
        overflow: scroll;

        box-sizing: border-box;
        border-radius: 15px;
        border: 2px solid var(--primary-color);

        margin-top: var(--margin-top, 0px);
        margin-bottom: var(--margin-bottom, 20px);

        box-shadow: 0px 2px 13px 7px rgba(0, 0, 0, 0.1);
    }

    table {
        width: 100%;
        box-sizing: border-box;

        border-collapse: collapse; /* Collapses borders into a single line */
    }

    /* Table headers */
    table :global(thead) {
        position: sticky;
        top: 0px;

        background-color: var(--head-background-color, var(--primary-color));
        color: var(--tertiary-color);
        text-align: left;
    }

    table :global(th) {
        padding: 12px 15px;
        font-size: 1em;
        text-transform: uppercase;
        border-bottom: 2px solid hsl(0, 0%, 30%);
    }

    /* Table body */
    table :global(tbody) {
        background-color: var(--body-background-color, var(--primary-color));
    }

    table :global(tr + tr td) {
        border-top: 1px solid hsl(0, 0%, 30%);
    }

    table :global(td) {
        padding: 12px 15px; /* Adds padding inside body cells */
        font-size: 14px; /* Sets font size */
    }

    /* Alternating row colors */
    table :global(tbody tr:nth-child(odd)) {
        background-color: var(--odd-row-color, var(--primary-color));
    }

    table :global(tbody tr:nth-child(even)) {
        background-color: var(--even-row-color, var(--secondary-color));
    }
</style>
