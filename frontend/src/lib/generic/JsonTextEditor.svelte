<script lang="ts">
    import { JSONEditor } from "svelte-jsoneditor";
    import Button from "./Button.svelte";
    import { faClose } from "@fortawesome/free-solid-svg-icons";
    import BlurredOverlay from "./BlurredOverlay.svelte";

    let {
        jsonObject = $bindable(),
        buttonText = "Edit",
    }: { jsonObject: object; buttonText?: string } = $props();

    let visible = $state(false);

    let content = $state({ json: jsonObject });
    $effect(() => {
        jsonObject = content.json;
    });
</script>

{#if !visible}
    <Button onclick={() => (visible = true)} --width="100%">{buttonText}</Button
    >
{:else}
    <BlurredOverlay>
        <div class="my-json-editor jse-theme-dark">
            <JSONEditor bind:content />
        </div>
        <Button onclick={() => (visible = false)} prefixIcon={faClose}
            >Close Editor</Button
        >
    </BlurredOverlay>
{/if}

<style>
    @import "svelte-jsoneditor/themes/jse-theme-dark.css"; /* JSONEditor dark theme */

    .my-json-editor {
        flex-grow: 1;
        align-self: stretch;
    }
</style>
