<script lang="ts">
  let { src, alt } = $props();
</script>

<div class="image-container">
  <img {src} {alt} />
</div>

<style>
  img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    object-position: center;
  }

  .image-container {
    width: var(--width, 100%);
    height: var(--height, 250px);
    border-radius: var(--border-radius, 15px);
    overflow: hidden;
    box-sizing: border-box;
  }
</style>
