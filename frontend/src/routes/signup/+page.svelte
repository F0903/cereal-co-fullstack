<script lang="ts">
    import { goto } from "$app/navigation";
    import { signup, UserForm } from "$lib/api/auth";
    import Button from "$lib/generic/Button.svelte";
    import ErrorBox from "$lib/generic/ErrorBox.svelte";
    import InputField from "$lib/generic/InputField.svelte";
    import { assertNotNull } from "$lib/utils/typeUtils";
    import { faPlus } from "@fortawesome/free-solid-svg-icons";
    import type { PageData } from "./$types";

    let form: HTMLFormElement;

    let { data }: { data: PageData } = $props();

    let error: "PasswordNotMatching" | boolean = $state(false);
    let errorMessage = $state("");

    async function onSignUpClick() {
        error = false;

        const formData = new FormData(form);

        const password = assertNotNull(formData.get("Password") as string);
        const confirmPassword = assertNotNull(
            formData.get("Confirm Password") as string,
        );

        if (password !== confirmPassword) {
            error = "PasswordNotMatching";
            errorMessage = "Passwords do not match!";
            return;
        }

        const userForm: UserForm = {
            mail: assertNotNull(formData.get("Mail") as string),
            password_plain: password,
        };

        try {
            await signup(userForm);
        } catch (err) {
            if (err instanceof Error) {
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
    <InputField name="Mail" input_type="email" />
    <InputField
        name="Password"
        input_type="password"
        has_input_error={error === "PasswordNotMatching"}
    />
    <InputField
        name="Confirm Password"
        input_type="password"
        has_input_error={error === "PasswordNotMatching"}
    />
    <div class="buttons-container">
        <Button prefixIcon={faPlus} onclick={onSignUpClick}>Sign Up</Button>
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
