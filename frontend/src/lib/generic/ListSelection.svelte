<script lang="ts">
    import Button from "$lib/generic/Button.svelte";
    import Spacer from "./Spacer.svelte";

    let {
        title,
        options,
        selection = $bindable(),
    }: {
        title: string;
        options: string[];
        selection: string;
    } = $props();

    function onSelect(name: string) {
        if (name === selection) return;
        selection = name;
    }
</script>

<div class="list-container">
    <div class="list-title-container">
        <span>{title}</span>
    </div>
    <Spacer
        --thickness="1px"
        --width="85%"
        --spacing="0px"
        --margin-top="10px"
        --color="var(--secondary-color)"
    />
    <div class="list">
        {#each options as option}
            <div class="list-item" class:selected={selection === option}>
                <Button
                    --background-color="var(--list-color)"
                    --margin="auto"
                    --padding="10px 20px"
                    onclick={() => onSelect(option)}>{option}</Button
                >
            </div>
            <Spacer
                --thickness="1px"
                --spacing="2px"
                --width="15%"
                --color="var(--secondary-color)"
            />
        {/each}
    </div>
</div>

<style>
    .list-item.selected {
        filter: brightness(1.5);
    }

    .list-item {
        padding: 5px;
    }

    .list {
        height: 100%;
        padding: 10px;

        box-sizing: border-box;
        overflow-y: scroll;

        display: flex;
        flex-direction: column;
    }

    .list-title-container {
        padding-left: var(--title-left-padding, 15px);
        padding-top: var(--title-top-padding, 15px);

        font-size: var(--title-size, 1em);
        text-transform: uppercase;
        font-weight: 700;
    }

    .list-container {
        --list-color: hsl(0, 0%, 10%);

        background-color: var(--list-color);

        height: 100%;
        box-sizing: border-box;
    }
</style>
