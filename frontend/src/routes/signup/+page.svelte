<script lang="ts">
    import { goto } from "$app/navigation";
    import { signup, UserSignupForm } from "$lib/api/auth";
    import Button from "$lib/generic/Button.svelte";
    import ErrorBox from "$lib/generic/ErrorBox.svelte";
    import InputField from "$lib/generic/InputField.svelte";
    import { AssertionError, assertNotNull } from "$lib/utils/typeUtils";
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import type { PageData } from "./$types";
    import { ApiError } from "$lib/api/errors";

    let form: HTMLFormElement;

    let { data }: { data: PageData } = $props();

    let disableButtons = $state(false);

    let error: "PasswordError" | "MailError" | "NameError" | boolean =
        $state(false);
    let errorMessage = $state("");

    async function onSignUpClick(event: Event) {
        event.preventDefault();

        error = false;

        const formData = new FormData(form);

        try {
            const password = assertNotNull(
                formData.get("Password") as string,
                "Password cannot be empty!",
                "PasswordError",
            );
            const confirmPassword = formData.get("Confirm Password") as string;

            if (password !== confirmPassword) {
                error = "PasswordError";
                errorMessage = "Passwords do not match!";
                return;
            }

            const signupForm: UserSignupForm = {
                mail: assertNotNull(
                    formData.get("Mail") as string,
                    "Mail cannot be empty!",
                    "MailError",
                ),
                password_plain: password,
                name: assertNotNull(
                    formData.get("Name") as string,
                    "Name cannot be empty!",
                    "NameError",
                ),
                address: formData.get("Address") as string,
                phone: formData.get("Phone") as string,
            };

            disableButtons = true;
            await signup(signupForm);
        } catch (err) {
            disableButtons = false;

            if (err instanceof ApiError) {
                if (err.statusCode == 409) {
                    error = "MailError";
                    errorMessage = "A user already exists with this mail.";
                    return;
                }

                error = true;
                errorMessage = err.message;
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

        await goto(data.redirect ?? "/");
    }
</script>

<div class="form-wrapper">
    <div class="form-container">
        <h1>Sign Up</h1>
        {#if error}
            <ErrorBox message={errorMessage} --margin-bottom="25px" />
        {/if}
        <form class="singup-container" bind:this={form}>
            <InputField
                name="Mail"
                input_type="email"
                has_input_error={error === "MailError"}
            />
            <InputField name="Name" has_input_error={error === "NameError"} />
            <InputField name="Address" optional />
            <InputField name="Phone" optional pattern="\+?(\d*)" />
            <InputField
                name="Password"
                input_type="password"
                has_input_error={error === "PasswordError"}
            />
            <InputField
                name="Confirm Password"
                input_type="password"
                has_input_error={error === "PasswordError"}
            />
            <Button
                disabled={disableButtons}
                prefixIcon={faPlus}
                onclick={onSignUpClick}
                --width="fit-content"
                --font-size="1.2em"
                --padding="15px"
                --margin="15px auto">Sign Up</Button
            >
        </form>
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

        min-width: 300px;
        width: 500px;
        max-width: 1000px;
        padding: 50px;
        border-radius: 15px;
    }

    .form-wrapper {
        display: flex;
        flex-direction: column;

        align-items: center;
        justify-content: center;

        height: 100%;
    }
</style>
