<script lang="ts">
    import { goto } from "$app/navigation";
    import { signup, UserSignupForm } from "$lib/api/auth";
    import Button from "$lib/generic/Button.svelte";
    import ErrorBox from "$lib/generic/ErrorBox.svelte";
    import InputField from "$lib/generic/InputField.svelte";
    import { AssertionError, assertNotNull } from "$lib/utils/typeUtils";
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import type { PageData } from "./$types";

    let form: HTMLFormElement;

    let { data }: { data: PageData } = $props();

    let disableButtons = $state(false);

    let error: "PasswordError" | "MailError" | "NameError" | boolean =
        $state(false);
    let errorMessage = $state("");

    async function onSignUpClick() {
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

<form class="singup-container" bind:this={form}>
    <h1>Sign Up</h1>
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
    <div class="buttons-container">
        <Button
            disabled={disableButtons}
            prefixIcon={faPlus}
            onclick={onSignUpClick}>Sign Up</Button
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

    .singup-container {
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
