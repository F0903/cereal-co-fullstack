<script lang="ts">
    import { JSONEditor } from "svelte-jsoneditor";
    import Button from "./Button.svelte";
    import { faClose } from "@fortawesome/free-solid-svg-icons";

    let { jsonObject = $bindable() }: { jsonObject: object } = $props();

    let visible = $state(false);

    let content = $state({ json: jsonObject });
    $effect(() => {
        jsonObject = content.json;
    });
</script>

{#if !visible}
    <Button onclick={() => (visible = true)} --width="100%">Edit</Button>
{:else}
    <div class="editor-overlay">
        <div class="my-json-editor jse-theme-dark">
            <JSONEditor bind:content />
        </div>
        <Button onclick={() => (visible = false)} prefixIcon={faClose}
            >Close Editor</Button
        >
    </div>
{/if}

<style>
    @import "svelte-jsoneditor/themes/jse-theme-dark.css"; /* JSONEditor dark theme */

    .my-json-editor {
        flex-grow: 1;
        align-self: stretch;
    }

    .editor-overlay {
        position: fixed;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        backdrop-filter: blur(5px);
        background-color: rgba(0, 0, 0, 0.35);
        z-index: 1;
        padding: 100px;
        padding-top: 125px;
        box-sizing: border-box;

        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
        gap: 25px;
    }
</style>
