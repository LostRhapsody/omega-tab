import { fileURLToPath } from "node:url";
import { mergeConfig, defineConfig, configDefaults } from "vitest/config";
import type { UserConfig } from 'vite';
import viteConfig from "./vite.config";

export default mergeConfig(
	viteConfig as UserConfig,
	defineConfig({
		test: {
			environment: "jsdom",
			exclude: [...configDefaults.exclude, "e2e/**"],
			root: fileURLToPath(new URL("./", import.meta.url)),
		},
		build: {
			target: 'esnext',
			rollupOptions: {
				output: {
					manualChunks: {
						'vendor-vue': ['vue', 'vue-router', 'pinia'],
						'vendor-ui': ['vuetify', '@mdi/font'],
					},
				},
			},
			chunkSizeWarningLimit: 600
		},
	}),
);
