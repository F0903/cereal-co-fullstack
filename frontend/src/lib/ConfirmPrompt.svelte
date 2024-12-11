<script lang="ts">
    import IconButton from "./IconButton.svelte";
    import { faCheck, faCross } from "@fortawesome/free-solid-svg-icons";

    let {
        action_message,
        show_prompt = $bindable(false),
        on_confirm,
        on_decline,
    }: {
        action_message: string;
        show_prompt: boolean;
        on_confirm: () => void;
        on_decline: () => void;
    } = $props();

    let dialog: HTMLDialogElement;

    $effect(() => {
        if (show_prompt) {
            dialog.showModal();
        }
    });

    function onDialogClose() {
        show_prompt = false;
    }

    function onConfirm() {
        on_confirm();
        dialog.close();
    }

    function onDecline() {
        on_decline();
        dialog.close();
    }
</script>

<div class="blur-background"></div>
<div class="confirm-prompt-container">
    <dialog class="confirm-prompt" bind:this={dialog} onclose={onDialogClose}>
        <h2>Are you sure?</h2>
        <p>Do you wish to {action_message}</p>
        <div class="buttons">
            <IconButton fa_icon={faCheck} onclick={} />
            <IconButton fa_icon={faCross} onclick={on_decline} />
        </div>
    </dialog>
</div>

<style>
    .buttons {
        display: flex;
        flex-direction: row;
        justify-content: center;
        align-items: center;
    }

    .confirm-prompt {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;

        border-radius: 5px;
    }

    .confirm-prompt-container {
        position: fixed;
        width: 100%;
        height: 100%;
    }

    .blur-background {
        position: fixed;
        width: 100%;
        height: 100%;

        filter: blur(2);
    }
</style>
