<template>
  <TpModal v-model="dialog" title="" size="sm">
    <div class="signup-form">
      <div class="signup-form__header">
        <h2 class="signup-form__title">Sign up for New Tab</h2>
        <p class="signup-form__subtitle">Create your account to get started.</p>
      </div>

      <TpAlert v-if="errorMessage" variant="error" dismissible @dismiss="errorMessage = ''">
        {{ errorMessage }}
      </TpAlert>

      <form @submit.prevent="signUp" class="signup-form__fields">
        <TpInput
          v-model="email"
          label="Email"
          type="email"
          placeholder="you@example.com"
          :error="emailError"
          :disabled="isLoading"
          required
          @blur="validateEmail"
        />

        <TpInput
          v-model="password"
          label="Password"
          type="password"
          placeholder="Create a password"
          :error="passwordError"
          :disabled="isLoading"
          autocomplete="new-password"
          required
          @blur="validatePassword"
        />

        <TpInput
          v-model="confirmPassword"
          label="Confirm Password"
          type="password"
          placeholder="Confirm your password"
          :error="confirmPasswordError"
          :disabled="isLoading"
          autocomplete="new-password"
          required
          @blur="validateConfirmPassword"
        />

        <TpButton
          variant="primary"
          type="submit"
          :disabled="!isFormValid || isLoading"
          :loading="isLoading"
          class="signup-form__submit"
        >
          Sign Up
        </TpButton>
      </form>

      <p class="signup-form__switch">
        Already have an account?
        <button type="button" class="signup-form__link" @click="switchToLogin">Login here.</button>
      </p>
    </div>
  </TpModal>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { authService } from "@/services/auth";
import { useUserStore } from "@/stores/user";
import { TpModal, TpInput, TpButton, TpAlert } from "@/components/ui";

const userStore = useUserStore();

const dialog = ref(true);
const email = ref("");
const password = ref("");
const confirmPassword = ref("");
const isLoading = ref(false);
const errorMessage = ref("");

const emailError = ref("");
const passwordError = ref("");
const confirmPasswordError = ref("");

const validateEmail = () => {
  if (!email.value) {
    emailError.value = "Email is required";
    return false;
  }
  if (!/.+@.+\..+/.test(email.value)) {
    emailError.value = "Email must be valid";
    return false;
  }
  emailError.value = "";
  return true;
};

const validatePassword = () => {
  if (!password.value) {
    passwordError.value = "Password is required";
    return false;
  }
  if (password.value.length < 6) {
    passwordError.value = "Password must be at least 6 characters";
    return false;
  }
  passwordError.value = "";
  return true;
};

const validateConfirmPassword = () => {
  if (!confirmPassword.value) {
    confirmPasswordError.value = "Please confirm your password";
    return false;
  }
  if (confirmPassword.value !== password.value) {
    confirmPasswordError.value = "Passwords do not match";
    return false;
  }
  confirmPasswordError.value = "";
  return true;
};

const isFormValid = computed(() => {
  return (
    email.value &&
    password.value &&
    confirmPassword.value &&
    !emailError.value &&
    !passwordError.value &&
    !confirmPasswordError.value
  );
});

const open = () => {
  dialog.value = true;
  errorMessage.value = "";
};

const close = () => {
  dialog.value = false;
  errorMessage.value = "";
};

const switchToLogin = () => {
  emit("switch-to-login");
};

const emit = defineEmits<{
  "switch-to-login": [];
  "signup-success": [];
}>();

const signUp = async () => {
  const emailValid = validateEmail();
  const passwordValid = validatePassword();
  const confirmValid = validateConfirmPassword();

  if (!emailValid || !passwordValid || !confirmValid) return;

  isLoading.value = true;
  errorMessage.value = "";

  try {
    const response = await authService.register(email.value, password.value);
    authService.setToken(response.token);

    await userStore.fetchUserData({
      id: response.user.id,
      email: response.user.email,
    });

    emit("signup-success");
    close();
    window.location.reload();
  } catch (error: unknown) {
    const err = error as { response?: { status: number } };
    if (err.response?.status === 409) {
      errorMessage.value = "Email already registered";
    } else {
      errorMessage.value = "Registration failed. Please try again.";
    }
  } finally {
    isLoading.value = false;
  }
};

defineExpose({ open, close });
</script>

<style scoped>
.signup-form {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-6);
}

.signup-form__header {
  text-align: center;
  margin-bottom: var(--tp-space-2);
}

.signup-form__title {
  font-size: var(--tp-text-xl);
  font-weight: var(--tp-font-bold);
  color: var(--tp-text-primary);
  margin-bottom: var(--tp-space-2);
}

.signup-form__subtitle {
  font-size: var(--tp-text-sm);
  color: var(--tp-text-muted);
}

.signup-form__fields {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-4);
}

.signup-form__submit {
  width: 100%;
  margin-top: var(--tp-space-2);
}

.signup-form__switch {
  text-align: center;
  font-size: var(--tp-text-sm);
  color: var(--tp-text-secondary);
}

.signup-form__link {
  color: var(--tp-accent);
  font-weight: var(--tp-font-semibold);
  text-decoration: none;
  cursor: pointer;
  transition: color var(--tp-transition-fast);
}

.signup-form__link:hover {
  color: var(--tp-accent-hover);
  text-decoration: underline;
}

.signup-form__link:focus-visible {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}
</style>
