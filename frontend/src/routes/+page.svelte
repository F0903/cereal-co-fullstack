<script lang="ts">
    import ProductsWidget from "$lib/ProductsWidget.svelte";
</script>

<div class="widgets">
    <ProductsWidget />
</div>

<style>
    .widgets {
        display: flex;
        flex-direction: row;
        flex-wrap: wrap;

        justify-content: center;
    }
</style>
