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
    width: 100%;
    height: 250px;
    border-radius: 15px;
    overflow: hidden;
    box-sizing: border-box;
  }
</style>
