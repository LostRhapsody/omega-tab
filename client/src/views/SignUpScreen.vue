<template>
  <div class="signup-screen">
    <!-- Gradient Overlay -->
    <div class="signup-screen__gradient"></div>

    <div class="signup-screen__container">
      <div class="signup-screen__header">
        <h1 class="signup-screen__logo">OmegaTab_</h1>
        <p class="signup-screen__subtitle">Create your account to get started</p>
      </div>

      <div class="signup-screen__card">
        <TpAlert v-if="errorMessage" variant="error" dismissible @dismiss="errorMessage = ''">
          {{ errorMessage }}
        </TpAlert>

        <form @submit.prevent="signUp" class="signup-screen__form">
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
            class="signup-screen__submit"
          >
            Sign Up
          </TpButton>
        </form>

        <p class="signup-screen__footer">
          Already have an account?
          <router-link to="/login" class="signup-screen__link"> Login here </router-link>
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { authService } from "@/services/auth";
import { useUserStore } from "@/stores/user";
import { TpAlert, TpInput, TpButton } from "@/components/ui";

const userStore = useUserStore();

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
  errorMessage.value = "";
};

const close = () => {
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
.signup-screen {
  min-height: 100vh;
  background: var(--tp-bg-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
}

.signup-screen__gradient {
  position: absolute;
  top: 0;
  right: 0;
  width: 100%;
  height: 100%;
  animation: animateBg 12s ease infinite;
  background-image: linear-gradient(45deg, var(--tp-accent-glow), var(--tp-accent));
  background-size: 400% 400%;
  opacity: 0.15;
  pointer-events: none;
}

@keyframes animateBg {
  0% {
    background-position: 0% 50%;
  }
  50% {
    background-position: 100% 50%;
  }
  100% {
    background-position: 0% 50%;
  }
}

.signup-screen__container {
  max-width: 400px;
  width: 100%;
  margin: 0 var(--tp-space-4);
  position: relative;
  z-index: 10;
}

.signup-screen__header {
  text-align: center;
  margin-bottom: var(--tp-space-8);
}

.signup-screen__logo {
  font-size: var(--tp-text-3xl);
  font-weight: var(--tp-font-bold);
  font-family: var(--tp-font-mono);
  color: var(--tp-text-primary);
  margin-bottom: var(--tp-space-2);
}

.signup-screen__logo::after {
  content: "";
  animation: tp-cursor-blink 1s step-end infinite;
  border-right: 2px solid var(--tp-accent);
  margin-left: 2px;
}

.signup-screen__subtitle {
  color: var(--tp-text-muted);
}

.signup-screen__card {
  background: var(--tp-bg-secondary);
  border: 1px solid var(--tp-border);
  border-radius: var(--tp-radius-lg);
  padding: var(--tp-space-6);
}

.signup-screen__form {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-4);
}

.signup-screen__submit {
  width: 100%;
  margin-top: var(--tp-space-2);
}

.signup-screen__footer {
  text-align: center;
  margin-top: var(--tp-space-6);
  color: var(--tp-text-muted);
  font-size: var(--tp-text-sm);
}

.signup-screen__link {
  color: var(--tp-accent);
  text-decoration: none;
  font-weight: var(--tp-font-medium);
  cursor: pointer;
  background: none;
  border: none;
  font-size: inherit;
}

.signup-screen__link:hover {
  text-decoration: underline;
}

@media (prefers-reduced-motion: reduce) {
  .signup-screen__gradient {
    animation: none;
  }
}
</style>
