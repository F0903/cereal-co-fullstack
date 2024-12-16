<script lang="ts">
  import type { Snippet } from "svelte";

  let { children }: { children: Snippet } = $props();
</script>

<div class="grid">
  {@render children()}
</div>

<style>
  .grid {
    padding: 50px;
    box-sizing: border-box;
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(225px, 1fr));
    gap: 20px;
    padding: 20px;

    background-color: var(--background-color, var(--primary-color));
  }
</style>
