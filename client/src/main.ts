import "./assets/css/tailwind.css";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";
import "@mdi/font/css/materialdesignicons.css";

import "vuetify/styles";
import * as Sentry from "@sentry/vue";
import { createPinia } from "pinia";
import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

// Lazy load Vuetify
const initVuetify = async () => {
  const { createVuetify } = await import("vuetify");
  return createVuetify({
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
    defaults: {
      global: {
        borderRadius: "12px", // Set your desired border radius here
      },
    },
  });
};

const bootstrap = async () => {
  const app = createApp(App);
  const vuetify = await initVuetify();

  Sentry.init({
    app,
    dsn: "https://80a9e8cf52ce2c1f0a3e055a18d825cb@o4508773394153472.ingest.us.sentry.io/4508774150111232",
    integrations: [
      Sentry.browserTracingIntegration({ router }),
      Sentry.replayIntegration(),
    ],
    // Tracing
    tracesSampleRate: 1.0, //  Capture 100% of the transactions
    // Set 'tracePropagationTargets' to control for which URLs distributed tracing should be enabled
    tracePropagationTargets: ["localhost", /^https:\/\/yourserver\.io\/api/],
    // Session Replay
    replaysSessionSampleRate: 0.1, // This sets the sample rate at 10%. You may want to change it to 100% while in development and then sample at a lower rate in production.
    replaysOnErrorSampleRate: 1.0, // If you're not already sampling the entire session, change the sample rate to 100% when sampling sessions where errors occur.
  });

  app.use(createPinia());
  app.use(router);
  app.use(vuetify);
  app.mount("#app");
};

bootstrap();
