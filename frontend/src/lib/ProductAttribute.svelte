<script lang="ts">
    let {
        attribute = $bindable(),
        editable,
    }: { attribute: Attribute; editable: boolean } = $props();

    function onValueBlur(event: FocusEvent) {
        const target = event.target as HTMLElement;
        const value = target.textContent;

        if (value === null) {
            console.error("value was null");
            return;
        }

        attribute.Value = value;
    }
</script>

<div class="attr-container" class:editable>
    <h3 class="attr-name">
        {attribute.Name}:
    </h3>
    <span class="attr-value" contenteditable={editable} onblur={onValueBlur}>
        {attribute.Value}
    </span>
</div>

<style>
    .attr-name {
        padding-bottom: 5px;
        border-bottom: 1px solid var(--secondary-color);
        margin-top: 0px;
        margin-bottom: 10px;
    }

    .attr-value {
        min-height: 20px;
        width: 100%;
    }

    .attr-container.editable {
        border-color: hsl(16, 65%, 40%);
    }

    .attr-container {
        background-color: var(--tertiary-color);
        margin: 5px 0px;
        padding: 10px;

        border-width: 2px;
        border-style: dashed;
        border-color: transparent;
        border-radius: 5px;
    }

    span {
        font-size: 1.1em;
        display: block;
    }
</style>
