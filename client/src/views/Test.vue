<template>
  <div class="p-4">
    <h1 class="text-2xl mb-4">Backend Test</h1>
    <v-btn @click="testBackend" :loading="loading">
      Test Backend
    </v-btn>
    <div v-if="response" class="mt-4">
      Response: {{ response }}
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref } from "vue";

const loading = ref(false);
const response = ref("");

async function testBackend() {
	loading.value = true;
	try {
		const res = await fetch("http://localhost:3000/hello");
		response.value = await res.text();
	} catch (err) {
		console.error("Error:", err);
		response.value = "Error connecting to backend";
	} finally {
		loading.value = false;
	}
}
</script>