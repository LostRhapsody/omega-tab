<template>
  <div class="login-screen">
    <!-- Gradient Overlay -->
    <div class="login-screen__gradient"></div>

    <div class="login-screen__container">
      <div class="login-screen__header">
        <h1 class="login-screen__logo">OmegaTab_</h1>
        <p class="login-screen__subtitle">Sign in to access your dashboard</p>
      </div>

      <div class="login-screen__card">
        <TpAlert v-if="errorMessage" variant="error" dismissible @dismiss="errorMessage = ''">
          {{ errorMessage }}
        </TpAlert>

        <form @submit.prevent="login" class="login-screen__form">
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
            autocomplete="current-password"
            required
            @blur="validatePassword"
          />

          <TpButton
            variant="primary"
            type="submit"
            :disabled="!isFormValid || isLoading"
            :loading="isLoading"
            class="login-screen__submit"
          >
            Sign In
          </TpButton>
        </form>

        <p class="login-screen__footer">
          Don't have an account?
          <router-link to="/signup" class="login-screen__link">Sign up</router-link>
        </p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from "vue";
import { useRouter } from "vue-router";
import { authService } from "@/services/auth";
import { useUserStore } from "@/stores/user";
import { TpAlert, TpInput, TpButton } from "@/components/ui";

const router = useRouter();
const userStore = useUserStore();

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
  passwordError.value = "";
  return true;
};

const isFormValid = computed(() => {
  return email.value && password.value && !emailError.value && !passwordError.value;
});

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

    router.push("/");
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
</script>

<style scoped>
.login-screen {
  min-height: 100vh;
  background: var(--tp-bg-primary);
  display: flex;
  align-items: center;
  justify-content: center;
  position: relative;
}

.login-screen__gradient {
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

.login-screen__container {
  max-width: 400px;
  width: 100%;
  margin: 0 var(--tp-space-4);
  position: relative;
  z-index: 10;
}

.login-screen__header {
  text-align: center;
  margin-bottom: var(--tp-space-8);
}

.login-screen__logo {
  font-size: var(--tp-text-3xl);
  font-weight: var(--tp-font-bold);
  font-family: var(--tp-font-mono);
  color: var(--tp-text-primary);
  margin-bottom: var(--tp-space-2);
}

.login-screen__logo::after {
  content: "";
  animation: tp-cursor-blink 1s step-end infinite;
  border-right: 2px solid var(--tp-accent);
  margin-left: 2px;
}

.login-screen__subtitle {
  color: var(--tp-text-muted);
}

.login-screen__card {
  background: var(--tp-bg-secondary);
  border: 1px solid var(--tp-border);
  border-radius: var(--tp-radius-lg);
  padding: var(--tp-space-6);
}

.login-screen__form {
  display: flex;
  flex-direction: column;
  gap: var(--tp-space-4);
}

.login-screen__submit {
  width: 100%;
  margin-top: var(--tp-space-2);
}

.login-screen__footer {
  text-align: center;
  margin-top: var(--tp-space-6);
  color: var(--tp-text-muted);
  font-size: var(--tp-text-sm);
}

.login-screen__link {
  color: var(--tp-accent);
  text-decoration: none;
  font-weight: var(--tp-font-medium);
}

.login-screen__link:hover {
  text-decoration: underline;
}

@media (prefers-reduced-motion: reduce) {
  .login-screen__gradient {
    animation: none;
  }
}
</style>
