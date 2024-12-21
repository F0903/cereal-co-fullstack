<script lang="ts">
    import { faMinus, faPlus } from "@fortawesome/free-solid-svg-icons";
    import Fa from "svelte-fa";

    let {
        value = $bindable(0),
        max_value = undefined,
        on_value_changed = undefined,
        on_negative_value_callback: on_negative_callback = undefined,
    }: {
        value?: number;
        max_value?: number;
        on_value_changed?: (newVal: number) => void;
        on_negative_value_callback?: () => void;
    } = $props();

    function onIncrementClick() {
        if (max_value && value >= max_value) {
            return;
        }
        value += 1;
        if (on_value_changed) on_value_changed(value);
    }

    function onDecrementClick() {
        if (on_negative_callback && value - 1 <= 0) {
            on_negative_callback();
            return;
        }

        value -= 1;
        if (on_value_changed) on_value_changed(value);
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
        user-select: none;
    }

    .minus-container {
        border-left: 2px solid var(--seperator-color, --default-seperator-color);
        padding-left: 5px;
    }

    .plus-container {
        border-right: 2px solid
            var(--seperator-color, --default-seperator-color);
        padding-right: 5px;
    }

    .counter-button:hover {
        cursor: pointer;
        filter: brightness(1.5);
    }

    .counter {
        --seperator-color: var(--secondary-color);
        background-color: var(--background-color, var(--primary-color));
        padding: 5px;

        display: flex;
        flex-direction: row;
        align-items: center;
        justify-content: center;
        gap: 5px;

        font-size: 1.2em;

        border-radius: 5px;
    }
</style>
