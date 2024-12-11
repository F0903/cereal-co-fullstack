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
        width: fit-content;
    }

    .error-box {
        background-color: var(--tertiary-color);
        border-radius: 7px;
        padding: 15px;
        margin-top: 10px;
    }
</style>
