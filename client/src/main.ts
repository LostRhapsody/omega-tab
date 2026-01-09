import "./assets/css/tailwind.css";

import { createHead } from "@unhead/vue";
import { createPinia } from "pinia";
import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

// Create Unhead instance
const head = createHead();

// Initialize application
const app = createApp(App);

// Use plugins and mount the app
app.use(head);
app.use(createPinia());
app.use(router);
app.mount("#app");
