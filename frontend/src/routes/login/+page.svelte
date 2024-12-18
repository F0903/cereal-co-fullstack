<script lang="ts">
    import { goto } from "$app/navigation";
    import { login, UserLoginForm } from "$lib/api/auth";
    import { ApiError } from "$lib/api/errors";
    import Button from "$lib/generic/Button.svelte";
    import ErrorBox from "$lib/generic/ErrorBox.svelte";
    import InputField from "$lib/generic/InputField.svelte";
    import { AssertionError, assertNotNull } from "$lib/utils/typeUtils";
    import { faLockOpen, faPlus } from "@fortawesome/free-solid-svg-icons";

    let form: HTMLFormElement;

    let disableButtons = $state(false);

    let error: "Unauthorized" | "MailError" | "PasswordError" | boolean =
        $state(false);
    let errorMessage = $state("");

    async function onLoginClick() {
        error = false;

        const formData = new FormData(form);

        try {
            const loginForm: UserLoginForm = {
                mail: assertNotNull(
                    formData.get("Mail") as string,
                    "Mail cannot be empty!",
                    "MailError",
                ),
                password_plain: assertNotNull(
                    formData.get("Password") as string,
                    "Password cannot be empty!",
                    "PasswordError",
                ),
            };

            disableButtons = true;
            await login(loginForm);
        } catch (err) {
            disableButtons = false;

            if (err instanceof ApiError) {
                error = "Unauthorized";
                errorMessage = "Invalid credentials!";
                return;
            }
            if (err instanceof AssertionError) {
                error = (err.failureName as any) ?? true;
                errorMessage = err.message;
                return;
            }

            error = true;
            errorMessage = err as string;
            return;
        }

        await goto("/", { invalidateAll: true });
    }
</script>

<form class="login-container" bind:this={form}>
    <h1>Log In</h1>
    <InputField
        name="Mail"
        input_type="email"
        has_input_error={error === "MailError"}
    />
    <InputField
        name="Password"
        input_type="password"
        has_input_error={error === "PasswordError"}
    />
    <div class="buttons-container">
        <Button
            disabled={disableButtons}
            prefixIcon={faPlus}
            onclick={() => goto("/signup?redirect=/login")}
            --background-color="var(--primary-color)">Sign Up</Button
        >
        <Button
            disabled={disableButtons}
            prefixIcon={faLockOpen}
            onclick={onLoginClick}>Login</Button
        >
    </div>
    {#if error}
        <ErrorBox message={errorMessage} />
    {/if}
</form>

<style>
    .buttons-container {
        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: normal;
        gap: 10px;
    }

    .login-container {
        background-color: var(--secondary-color);
        margin: auto;
        margin-top: 150px;
        max-width: 500px;
        padding: 50px;
        border-radius: 15px;

        display: flex;
        flex-direction: column;
        gap: 25px;
    }
</style>
