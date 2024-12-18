<script lang="ts">
    import { goto } from "$app/navigation";
    import { changePassword, ChangePasswordForm } from "$lib/api/auth";
    import { ApiError } from "$lib/api/errors";
    import Button from "$lib/generic/Button.svelte";
    import ErrorBox from "$lib/generic/ErrorBox.svelte";
    import InputField from "$lib/generic/InputField.svelte";
    import { AssertionError, assertNotNull } from "$lib/utils/typeUtils";
    import { faArrowRight } from "@fortawesome/free-solid-svg-icons";
    import type { PageData } from "./$types";

    let { data }: { data: PageData } = $props();

    let form: HTMLFormElement;

    let disableButtons = $state(false);

    let error: "Unauthorized" | "OldPasswordError" | "PasswordError" | boolean =
        $state(false);
    let errorMessage = $state("");

    async function onChangeClick() {
        error = false;

        const formData = new FormData(form);

        try {
            const loginForm: ChangePasswordForm = {
                old_password_plain: assertNotNull(
                    formData.get("Old Password") as string,
                    "Old password cannot be empty!",
                    "OldPasswordError",
                ),
                new_password_plain: assertNotNull(
                    formData.get("Password") as string,
                    "Password cannot be empty!",
                    "PasswordError",
                ),
            };

            disableButtons = true;
            await changePassword(loginForm);
        } catch (err) {
            disableButtons = false;

            if (err instanceof ApiError) {
                error = err.statusCode === 401 ? "Unauthorized" : true;
                errorMessage =
                    err.statusCode === 401
                        ? "Invalid credentials!"
                        : err.message;
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

        await goto(data.redirect ?? "/");
    }
</script>

<div class="form-container">
    <h1>Change Password</h1>
    {#if error}
        <ErrorBox message={errorMessage} --margin-bottom="25px" />
    {/if}
    <form class="passwords-container" bind:this={form}>
        <InputField
            name="Old Password"
            input_type="password"
            has_input_error={error === "OldPasswordError"}
        />
        <InputField
            name="Password"
            input_type="password"
            has_input_error={error === "PasswordError"}
        />
        <Button
            disabled={disableButtons}
            prefixIcon={faArrowRight}
            onclick={onChangeClick}
            --padding="15px"
            --width="fit-content"
            --margin="15px auto 0px auto"
            --font-size="1.2em">Change</Button
        >
    </form>
</div>

<style>
    form {
        display: flex;
        flex-direction: column;
        gap: 15px;
    }

    .form-container {
        background-color: var(--secondary-color);
        margin: auto;
        margin-top: 150px;
        max-width: 500px;
        padding: 50px;
        border-radius: 15px;
    }
</style>
