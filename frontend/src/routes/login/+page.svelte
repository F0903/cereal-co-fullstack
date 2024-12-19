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

    async function onLoginClick(event: Event) {
        event.preventDefault();

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

<div class="form-wrapper">
    <div class="form-container">
        <h1>Log In</h1>
        {#if error}
            <ErrorBox message={errorMessage} --margin-bottom="25px" />
        {/if}
        <form class="login-container" bind:this={form} onsubmit={onLoginClick}>
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
            <Button
                disabled={disableButtons}
                prefixIcon={faLockOpen}
                onclick={onLoginClick}
                --width="fit-content"
                --padding="15px"
                --font-size="1.2em"
                --margin="15px auto 0px auto">Login</Button
            >
        </form>
        <Button
            disabled={disableButtons}
            prefixIcon={faPlus}
            onclick={() => goto("/signup?redirect=/login")}
            --background-color="var(--primary-color)"
            --width="fit-content"
            --margin="15px auto 0px auto"
            --font-size="1em">Sign Up</Button
        >
    </div>
</div>

<style>
    h1 {
        margin-bottom: 20px;
    }

    form {
        display: flex;
        flex-direction: column;
        gap: 15px;
    }

    .form-container {
        background-color: var(--secondary-color);
        margin: auto;
        max-width: 500px;
        padding: 50px;

        border-radius: 15px;
    }

    .form-wrapper {
        padding-top: 150px;
    }
</style>
