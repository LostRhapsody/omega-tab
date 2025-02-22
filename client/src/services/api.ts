import { useUserStore } from "@/stores/user";
// src/services/api.ts
import axios, { type AxiosInstance } from "axios";

const api: AxiosInstance = axios.create({
  baseURL: import.meta.env.VITE_API_BASE_URL || "http://localhost:3000",
  timeout: 10000, // Optional
});

// Request interceptor to add custom headers
api.interceptors.request.use(
  (config) => {
    const userStore = useUserStore();
    // Add your custom headers here
    config.headers.set("X-User-Id", userStore.userId);
    config.headers.set("X-User-Email", userStore.email);
    config.headers.set('Authorization', `Bearer ${localStorage.getItem('token') || ''}`);
    config.headers.set("Content-Type", "application/json");
    return config;
  },
  (error) => {
    return Promise.reject(error);
  },
);

// Optional: Response interceptor for global error handling
// api.interceptors.response.use(
//   (response) => response,
//   (error) => {
//     // Example: Handle 401 globally
//     if (error.response?.status === 401) {
//       console.error('Unauthorized! Redirecting to login...');
//       // Add logic to redirect or handle errors
//     }
//     return Promise.reject(error);
//   }
// );

export default api;
