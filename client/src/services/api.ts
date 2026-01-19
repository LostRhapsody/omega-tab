// src/services/api.ts
import { useUserStore } from "@/stores/user";
import axios, { type AxiosInstance } from "axios";

const apiDomain = import.meta.env.VITE_API_BASE_URL || "http://localhost:3000";
const api: AxiosInstance = axios.create({
  baseURL: `${apiDomain}/api`,
  timeout: 10000, // Optional
});

// Request interceptor to add auth header
api.interceptors.request.use(
  (config) => {
    config.headers.set("Authorization", `Bearer ${localStorage.getItem("token") || ""}`);
    config.headers.set("Content-Type", "application/json");
    return config;
  },
  (error) => {
    return Promise.reject(error);
  },
);

// Response interceptor for token refresh and error handling
api.interceptors.response.use(
  (response) => {
    // Check for new token in response headers (auto-refresh from backend)
    const newToken = response.headers["x-new-auth-token"];
    if (newToken) {
      localStorage.setItem("token", newToken);
    }
    return response;
  },
  (error) => {
    if (error.response?.status === 401) {
      // Token expired or invalid - clear and redirect
      localStorage.removeItem("token");
      const userStore = useUserStore();
      userStore.clearUser();
      window.location.href = "/";
    }
    return Promise.reject(error);
  },
);

export default api;
