<script lang="ts">
  import type { FullAutoFill } from "svelte/elements";

  let {
    name,
    input_type = "text",
    autocomplete = "off",
    has_input_error = false,
  }: {
    name: string;
    input_type?: string;
    autocomplete?: FullAutoFill;
    has_input_error?: boolean;
  } = $props();
</script>

<div class="input-field" class:error={has_input_error}>
  <label for={name}>{name}</label>
  <input type={input_type} {name} {autocomplete} />
</div>

<style>
  input {
    border: 0;
    background-color: inherit;
    display: block;
    width: 100%;
    box-sizing: border-box;

    padding-top: 7px;

    font: inherit;
    color: inherit;
  }

  label {
    display: block;
    width: 100%;
    border-bottom: 2px solid
      var(--label-underline-color, var(--secondary-color));
    box-sizing: border-box;
    font-weight: var(--label-font-weight, 600);
    font-size: 1.15em;
    padding-bottom: 7px;
  }

  .input-field.error {
    border-color: var(--error-color);
  }

  .input-field {
    color: var(--text-color, var(--tertiary-color));
    background-color: var(--bg-color, var(--primary-color));
    box-sizing: border-box;
    padding: 15px;
    border-radius: 5px;
    border: 2px dashed transparent;
  }
</style>
