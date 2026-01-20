import "./assets/css/main.css";

import { createApp } from "vue";
import App from "./App.vue";
import router from "./router";

// Initialize application
const app = createApp(App);

app.use(router);
app.mount("#app");
