<template>
  <div class="staging-login-container">
    <div class="staging-login-card">
      <h1>Staging Environment</h1>
      <p>This is a test environment. Please enter the password to continue.</p>
      
      <form @submit.prevent="handleLogin" class="staging-login-form">
        <div class="input-group">
          <label for="password">Password</label>
          <input 
            type="password" 
            id="password" 
            v-model="password" 
            placeholder="Enter staging password"
            required
          />
        </div>
        
        <div v-if="error" class="error-message">
          {{ error }}
        </div>
        
        <button 
          type="submit" 
          class="login-button" 
          :disabled="isLoading"
        >
          {{ isLoading ? 'Logging in...' : 'Access Staging' }}
        </button>
      </form>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { useRouter } from 'vue-router';
import api from '@/services/api';
import { API } from '@/constants/api';
import { CacheKeys, cache } from '@/utils/cache';

const password = ref('');
const error = ref('');
const isLoading = ref(false);
const router = useRouter();

const handleLogin = async () => {
  error.value = '';
  isLoading.value = true;
  
  try {
    const response = await api.post(API.STAGING_LOGIN, { password: password.value });
    
    if (response.status === 200) {
      // Store in cache that we're logged in
      cache.set(CacheKeys.STAGING_LOGGED_IN, true);
      // Navigate to home page
      router.push('/');
    }
  } catch (err: any) {
    error.value = err.response?.status === 403 
      ? 'Invalid password. Please try again.' 
      : 'An error occurred. Please try again.';
  } finally {
    isLoading.value = false;
  }
};
</script>

<style scoped>
.staging-login-container {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
  background-color: #f5f5f5;
}

.staging-login-card {
  background-color: white;
  border-radius: 8px;
  box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
  padding: 2rem;
  width: 100%;
  max-width: 400px;
  text-align: center;
}

h1 {
  margin-bottom: 1rem;
  color: #333;
}

p {
  margin-bottom: 2rem;
  color: #666;
}

.staging-login-form {
  display: flex;
  flex-direction: column;
}

.input-group {
  margin-bottom: 1.5rem;
  text-align: left;
}

label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: #333;
}

input {
  width: 100%;
  padding: 0.75rem;
  border: 1px solid #ddd;
  border-radius: 4px;
  font-size: 1rem;
}

.error-message {
  color: #e53935;
  margin-bottom: 1rem;
  font-size: 0.9rem;
}

.login-button {
  background-color: #4f46e5;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 0.75rem 1rem;
  font-size: 1rem;
  cursor: pointer;
  transition: background-color 0.2s;
}

.login-button:hover {
  background-color: #4338ca;
}

.login-button:disabled {
  background-color: #a5b4fc;
  cursor: not-allowed;
}
</style>