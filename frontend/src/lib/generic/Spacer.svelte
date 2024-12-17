<script lang="ts">
    let { center = true } = $props();
</script>

<div class="spacer" class:center></div>

<style>
    .spacer.center {
        margin-left: auto;
        margin-right: auto;
    }

    .spacer {
        display: block;
        width: var(--width, 25%);
        height: var(--thickness, 2px);
        background-color: var(--color, var(--tertiary-color));
        background-clip: content-box;
        margin-top: var(--margin-top, 0px);
        margin-bottom: var(--margin-bottom, 0px);
        padding: var(--spacing, 5px) 0px;
    }
</style>
