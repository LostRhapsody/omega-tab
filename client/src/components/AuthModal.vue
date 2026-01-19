<template>
  <div>
    <!-- Login modal -->
    <Login
      v-if="activeModal === 'login'"
      v-model:showModal="showModal"
      @switch-to-signup="switchToSignUp"
      ref="loginRef"
    />

    <!-- SignUp modal -->
    <SignUp
      v-if="activeModal === 'signup'"
      v-model:showModal="showModal"
      @switch-to-login="switchToLogin"
      ref="signupRef"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from "vue";
import { nextTick } from "vue";
import Login from "@/components/Login.vue";
import SignUp from "@/components/SignUp.vue";
import { cache } from "@/utils/cache";
import type { AuthModalMethods } from "@/types/AuthModalMethods";

const emit = defineEmits(["update:showModal", "update:activeModal"]);

// Local state that syncs with props
const activeModal = ref("login");
const showModal = ref(false);

// Exposed methods for parent components
const openLogin = () => {
  // Clear all cache before opening auth modal to ensure fresh state
  cache.clearAll();
  activeModal.value = "login";
  showModal.value = true;
};

const openSignUp = () => {
  // Clear all cache before opening auth modal to ensure fresh state
  cache.clearAll();
  activeModal.value = "signup";
  showModal.value = true;
};

const closeModal = () => {
  showModal.value = false;
};

const switchToSignUp = () => {
  activeModal.value = "signup";
};

const switchToLogin = () => {
  activeModal.value = "login";
};

// Expose methods for parent components to use
defineExpose<AuthModalMethods>({
  openLogin,
  openSignUp,
  closeModal,
});
</script>
