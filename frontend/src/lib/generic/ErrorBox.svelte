<script lang="ts">
    let { message }: { message: string } = $props();
</script>

<div class="error-box">
    <h2>Error!</h2>
    <p>{message}</p>
</div>

<style>
    h2 {
        padding-bottom: 1px;
        border-bottom: 3px solid;
        margin-bottom: 10px;
        width: fit-content;
    }

    .error-box {
        background-color: var(--background-color, var(--primary-color));
        border-left: 3px solid var(--error-color, hsl(0, 45%, 45%));
        border-radius: 7px;
        padding: 15px;
        margin-top: var(--margin-top, 10px);
        margin-bottom: var(--margin-bottom, 0px);
    }
</style>
