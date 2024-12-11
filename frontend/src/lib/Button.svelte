<script lang="ts">
  let { text } = $props();
</script>

<button>{text}</button>

<style>
  button:hover {
    cursor: pointer;
    filter: brightness(1.5);
  }

  button {
    display: block;
    border: 0;
    background-color: var(--quaternary-color);
    color: hsl(0, 0%, 85%);
    font-family: "Roboto Flex Variable", sans-serif;
    font-size: 1.1em;
    font-weight: 600;
    border-radius: var(--border-radius, 15px);
    padding: var(--padding, 10px);
  }
</style>
