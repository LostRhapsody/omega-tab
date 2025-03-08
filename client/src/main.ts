import "./assets/css/tailwind.css";
// Consolidate Vuetify imports to reduce bundle size
import { createVuetify } from "vuetify";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";
import "vuetify/styles";
import "@mdi/font/css/materialdesignicons.css";

import * as Sentry from "@sentry/vue";
import { createHead } from "@unhead/vue";
import { createPinia } from "pinia";
import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

// Create Unhead instance
const head = createHead();

// Create Vuetify instance directly instead of lazy loading
const vuetify = createVuetify({
  components,
  directives,
  theme: {
    defaultTheme: "dark",
    themes: {
      dark: {
        dark: true,
        colors: {
          primary: "#1867C0",
          secondary: "#5CBBF6",
        },
      },
    },
  },
});

// Initialize application
const app = createApp(App);

// Configure Sentry
Sentry.init({
  app,
  dsn: "https://80a9e8cf52ce2c1f0a3e055a18d825cb@o4508773394153472.ingest.us.sentry.io/4508774150111232",
  integrations: [
    Sentry.browserTracingIntegration({ router }),
    Sentry.replayIntegration(),
  ],
  // Reduce sample rates for production
  tracesSampleRate: process.env.NODE_ENV === "production" ? 0.2 : 1.0,
  tracePropagationTargets: ["localhost", /^https:\/\/yourserver\.io\/api/],
  replaysSessionSampleRate: process.env.NODE_ENV === "production" ? 0.1 : 0.5,
  replaysOnErrorSampleRate: 1.0,
});

// Use plugins and mount the app
app.use(head);
app.use(createPinia());
app.use(router);
app.use(vuetify);
app.mount("#app");
