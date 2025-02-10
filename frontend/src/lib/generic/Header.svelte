<script lang="ts">
    let { left, center, right } = $props();
</script>

<div class="header-container">
    <header>
        <div class="header-column left">
            {@render left()}
        </div>
        <div class="header-column center">
            {@render center()}
        </div>
        <div class="header-column right">
            {@render right()}
        </div>
    </header>
</div>

<style>
    .left {
        justify-self: start;
    }

    .right {
        justify-self: end;
    }

    .header-column {
        height: 100%;

        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }

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

        display: grid;
        grid-template-columns: 1fr auto 1fr;
        grid-auto-rows: var(--header-height);
        align-items: center;
        align-content: center;
        justify-items: center;
        justify-content: center;
    }
</style>
