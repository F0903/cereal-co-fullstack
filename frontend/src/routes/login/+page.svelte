<script lang="ts">
  import { goto } from "$app/navigation";
  import { login, UserForm } from "$lib/api/auth";
  import { ApiError } from "$lib/api/errors";
  import Button from "$lib/Button.svelte";
  import ErrorBox from "$lib/ErrorBox.svelte";
  import InputField from "$lib/InputField.svelte";
  import { assertNotNull } from "$lib/utils/typeUtils";
  import { faLockOpen, faPlus } from "@fortawesome/free-solid-svg-icons";

  let form: HTMLFormElement;

  let error: "Unauthorized" | boolean = $state(false);
  let errorMessage = $state("");

  async function onLoginClick() {
    error = false;

    const formData = new FormData(form);

    const userForm: UserForm = {
      mail: assertNotNull(formData.get("Mail") as string),
      password_plain: assertNotNull(formData.get("Password") as string),
    };

    try {
      await login(userForm);
    } catch (err) {
      if (err instanceof ApiError) {
        error = "Unauthorized";
        errorMessage = "Invalid credentials!";
        return;
      }
      if (err instanceof Error) {
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
  <InputField name="Mail" has_input_error={error === "Unauthorized"} />
  <InputField
    name="Password"
    input_type="password"
    has_input_error={error === "Unauthorized"}
  />
  <div class="buttons-container">
    <Button
      text="Sign Up"
      prefixIcon={faPlus}
      onclick={() => goto("/signup?redirect=/login")}
      --background-color="var(--primary-color)"
    />
    <Button text="Login" prefixIcon={faLockOpen} onclick={onLoginClick} />
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
