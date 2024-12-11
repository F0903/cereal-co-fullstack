<script lang="ts">
    import { goto } from "$app/navigation";
    import { login } from "$lib/api/auth_api";
    import ErrorBox from "$lib/ErrorBox.svelte";
    import IconButton from "$lib/IconButton.svelte";
    import InputField from "$lib/InputField.svelte";
    import type { User } from "$lib/models/User";
    import {
        findParentWithTag,
        getObjectFromFormData,
    } from "$lib/utils/dom_utils";
    import { faArrowRight } from "@fortawesome/free-solid-svg-icons";

    let error: Error | null = $state(null);

    async function submit(event: Event) {
        event.preventDefault();

        error = null;

        let form = findParentWithTag("form", event.target as HTMLElement);
        const user = getObjectFromFormData<User>(form as HTMLFormElement);

        try {
            await login(user);
            await goto("/", { invalidateAll: true });
        } catch (err) {
            error = err as Error;
        }
    }
</script>

<div class="login-container">
    <h2>Login</h2>
    <form class="creator-form" action="">
        <InputField name="Username" has_input_error={error != null} />
        <InputField
            name="Password"
            input_type="password"
            has_input_error={error != null}
        />
        <IconButton
            fa_icon={faArrowRight}
            onclick={submit}
            margin="5px"
            bg_color="var(--tertiary-color)"
        />
    </form>
    {#if error}
        <ErrorBox message={error.message} />
    {/if}
</div>

<style>
    .creator-form {
        display: flex;
        flex-direction: column;
        gap: 15px;
        justify-content: center;
        align-items: center;
    }

    .login-container {
        display: flex;
        flex-direction: column;
        align-items: center;

        width: fit-content;
        height: 100%;

        border-radius: 10px;
        margin-top: 20vh;
        padding: 15px;
        margin-left: auto;
        margin-right: auto;

        background-color: var(--secondary-color);
    }
</style>
