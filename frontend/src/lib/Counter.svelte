<script lang="ts">
  import { faMinus, faPlus } from "@fortawesome/free-solid-svg-icons";
  import Fa from "svelte-fa";

  let {
    value = $bindable(0),
    on_negative_callback = undefined,
  }: { value?: number; on_negative_callback?: () => void } = $props();

  function onIncrementClick() {
    value += 1;
  }

  function onDecrementClick() {
    if (on_negative_callback && value - 1 <= 0) {
      on_negative_callback();
      return;
    }

    value -= 1;
  }
</script>

<div class="counter">
  <div
    class="plus-container counter-button"
    onkeypress={onIncrementClick}
    onclick={onIncrementClick}
    role="button"
    tabindex="0"
  >
    <Fa icon={faPlus} />
  </div>
  <span class="value">{value}</span>
  <div
    class="minus-container counter-button"
    onkeypress={onDecrementClick}
    onclick={onDecrementClick}
    role="button"
    tabindex="0"
  >
    <Fa icon={faMinus} />
  </div>
</div>

<style>
  .value {
    padding-left: 5px;
    padding-right: 5px;
  }

  .minus-container {
    border-left: 2px solid var(--secondary-color);
    padding-left: 5px;
  }

  .plus-container {
    border-right: 2px solid var(--secondary-color);
    padding-right: 5px;
  }

  .counter-button:hover {
    cursor: pointer;
    filter: brightness(1.2);
  }

  .counter {
    background-color: var(--primary-color);
    padding: 5px;

    display: flex;
    flex-direction: row;
    align-items: center;
    justify-content: center;
    gap: 5px;

    font-size: 1.2em;
  }
</style>
