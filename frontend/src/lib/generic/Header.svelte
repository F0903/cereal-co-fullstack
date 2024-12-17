<script lang="ts">
    let { children } = $props();
</script>

<div class="header-container">
    <header>
        {@render children()}
    </header>
</div>

<style>
    .header-container {
        position: sticky;
        top: 0;
        z-index: 99;
    }

    header {
        --header-height: 75px;

        position: relative;

        width: 100vw;
        height: var(--header-height);
        padding: 0px 15px;
        box-sizing: border-box;

        background-color: var(--secondary-color);

        box-shadow: 0px 1px 25px 2px hsl(0, 0%, 10%);
    }
</style>
