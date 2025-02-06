import { sentryVitePlugin } from "@sentry/vite-plugin";
import { fileURLToPath, URL } from 'node:url'

import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import vueDevTools from 'vite-plugin-vue-devtools'
import vuetify from 'vite-plugin-vuetify'
import compression from 'vite-plugin-compression';

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    vueJsx(),
    vueDevTools(),
    vuetify({ autoImport: true }),
    compression({
      algorithm: 'gzip',
      ext: '.gz',
    }),
    sentryVitePlugin({
      org: "better-new-tab",
      project: "betternewtab-vue"
    })
  ],

  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },

  build: {
    sourcemap: true
  }
})