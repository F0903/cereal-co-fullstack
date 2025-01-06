<script lang="ts">
    let { children } = $props();
</script>

<div class="blurred-overlay">
    {@render children()}
</div>

<style>
    .blurred-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        backdrop-filter: blur(5px);
        background-color: rgba(0, 0, 0, 0.35);
        z-index: 1;
        padding: 100px;
        padding-top: 125px;
        box-sizing: border-box;

        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 25px;
    }
</style>
