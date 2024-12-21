<script lang="ts">
    let { center = true } = $props();
</script>

<div class="spacer" class:center></div>

<style>
    .spacer.center {
        margin-top: auto;
        margin-bottom: auto;
    }

    .spacer {
        display: inline;
        box-sizing: content-box;

        height: var(--height, 25%);
        width: var(--thickness, 2px);

        background-color: var(--color, var(--tertiary-color));
        background-clip: content-box;

        margin-top: var(--margin-top, 0px);
        margin-bottom: var(--margin-bottom, 0px);
        margin-left: var(--margin-left, 0px);
        margin-right: var(--margin-right, 0px);
        padding: 0px var(--spacing, 5px);
    }
</style>
