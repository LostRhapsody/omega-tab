import "./assets/css/tailwind.css";
import * as components from "vuetify/components";
import * as directives from "vuetify/directives";
import "@mdi/font/css/materialdesignicons.css";

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

	app.use(createPinia());
	app.use(router);
	app.use(vuetify);
	app.mount("#app");
};

bootstrap();
