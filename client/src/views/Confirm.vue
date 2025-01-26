<template>
  <div class="min-h-screen flex flex-col items-center justify-center p-4">
    <!-- Loading State -->
    <div v-if="isLoading" class="text-center">
      <v-progress-circular
        indeterminate
        size="64"
        class="mb-4"
      ></v-progress-circular>
      <p class="text-h6">Confirming your subscription...</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="text-center">
      <v-icon
        icon="mdi-alert-circle"
        color="error"
        size="64"
        class="mb-4"
      ></v-icon>
      <p class="text-h6 text-error mb-4">Something went wrong</p>
      <v-btn color="primary" @click="retryConfirmation">
        Retry
      </v-btn>
    </div>

    <!-- Success State -->
    <div v-else class="text-center">
      <v-icon
        icon="mdi-check-circle"
        color="success"
        size="64"
        class="mb-4"
      ></v-icon>
      <h1 class="text-h4 mb-4">Welcome to Better New Tab!</h1>
      <p class="text-h6 mb-8">Your subscription has been confirmed.</p>
      <v-btn
        color="primary"
        size="x-large"
        @click="goToHome"
      >
        Start using Better New Tab
      </v-btn>
    </div>

    <!-- Snackbar -->
    <v-snackbar
      v-model="showSnackbar"
      :timeout="5000"
    >
      User created and subscription successful, start taking full advantage of Better New Tab today!
      <template v-slot:actions>
        <v-btn
          color="white"
          variant="text"
          @click="showSnackbar = false"
        >
          Close
        </v-btn>
      </template>
    </v-snackbar>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { useApi } from '../composables/useApi';

const router = useRouter();
const { api } = useApi();

const isLoading = ref(true);
const error = ref(false);
const showSnackbar = ref(false);

const confirmSubscription = async () => {
  try {
    isLoading.value = true;
    error.value = false;

    await api('/confirm', {
      method: 'POST',
      body: JSON.stringify({ email: 'evan.robertson77@gmail.com' }) // Replace with actual email
    });

    isLoading.value = false;
    showSnackbar.value = true;
  } catch (err) {
    console.error('Error confirming subscription:', err);
    error.value = true;
    isLoading.value = false;
  }
};

const retryConfirmation = () => {
  confirmSubscription();
};

const goToHome = () => {
  router.push('/');
};

onMounted(() => {
  confirmSubscription();
});
</script>