import "./assets/css/main.css";

import { createHead } from "@unhead/vue";
import { createPinia } from "pinia";
import { createApp } from "vue";
import type { Plugin } from "vue";
import App from "./App.vue";
import router from "./router";

// Create Unhead instance
const head = createHead();

// Initialize application
const app = createApp(App);

// Use plugins and mount the app
app.use(head as unknown as Plugin);
app.use(createPinia() as unknown as Plugin);
app.use(router);
app.mount("#app");
