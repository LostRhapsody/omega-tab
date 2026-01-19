<template>
  <TpModal v-model="dialog" title="" size="sm">
    <div class="login-form">
      <div class="login-form__header">
        <h2 class="login-form__title">Sign in to New Tab</h2>
        <p class="login-form__subtitle">Welcome back! Please sign in.</p>
      </div>

      <TpAlert v-if="errorMessage" variant="error" dismissible @dismiss="errorMessage = ''">
        {{ errorMessage }}
      </TpAlert>

      <form @submit.prevent="login" class="login-form__fields">
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
          placeholder="Enter your password"
          :error="passwordError"
          :disabled="isLoading"
          autocomplete="password"
          required
          @blur="validatePassword"
        />

        <TpButton
          variant="primary"
          type="submit"
          :disabled="!isFormValid || isLoading"
          :loading="isLoading"
          class="login-form__submit"
        >
          Login
        </TpButton>
      </form>

      <p class="login-form__switch">
        Don't have an account?
        <button type="button" class="login-form__link" @click="switchToSignUp">
          Sign up here.
        </button>
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
const isLoading = ref(false);
const errorMessage = ref("");

const emailError = ref("");
const passwordError = ref("");

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

const isFormValid = computed(() => {
  return email.value && password.value && !emailError.value && !passwordError.value;
});

const open = () => {
  dialog.value = true;
  errorMessage.value = "";
};

const close = () => {
  dialog.value = false;
  errorMessage.value = "";
};

const login = async () => {
  const emailValid = validateEmail();
  const passwordValid = validatePassword();

  if (!emailValid || !passwordValid) return;

  isLoading.value = true;
  errorMessage.value = "";

  try {
    const response = await authService.login(email.value, password.value);
    authService.setToken(response.token);

    await userStore.fetchUserData({
      id: response.user.id,
      email: response.user.email,
    });

    emit("login-success");
    close();
    window.location.reload();
  } catch (error: unknown) {
    const err = error as { response?: { status: number } };
    if (err.response?.status === 401) {
      errorMessage.value = "Invalid email or password";
    } else {
      errorMessage.value = "Login failed. Please try again.";
    }
  } finally {
    isLoading.value = false;
  }
};

const switchToSignUp = () => {
  emit("switch-to-signup");
};

const emit = defineEmits<{
  "switch-to-signup": [];
  "login-success": [];
}>();

defineExpose({ open, close });
</script>

<style scoped>
.login-form {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-6);
}

.login-form__header {
  text-align: center;
  margin-bottom: var(--tp-space-2);
}

.login-form__title {
  font-size: var(--tp-text-xl);
  font-weight: var(--tp-font-bold);
  color: var(--tp-text-primary);
  margin-bottom: var(--tp-space-2);
}

.login-form__subtitle {
  font-size: var(--tp-text-sm);
  color: var(--tp-text-muted);
}

.login-form__fields {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-4);
}

.login-form__submit {
  width: 100%;
  margin-top: var(--tp-space-2);
}

.login-form__switch {
  text-align: center;
  font-size: var(--tp-text-sm);
  color: var(--tp-text-secondary);
}

.login-form__link {
  color: var(--tp-accent);
  font-weight: var(--tp-font-semibold);
  text-decoration: none;
  cursor: pointer;
  transition: color var(--tp-transition-fast);
}

.login-form__link:hover {
  color: var(--tp-accent-hover);
  text-decoration: underline;
}

.login-form__link:focus-visible {
  outline: var(--tp-focus-ring);
  outline-offset: var(--tp-focus-offset);
}
</style>
