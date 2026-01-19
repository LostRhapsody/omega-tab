import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import type { PluginOption } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import compression from 'vite-plugin-compression'
import tailwindcss from '@tailwindcss/vite'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue(),
    tailwindcss(),
    compression({
      algorithm: 'gzip',
      ext: '.gz',
      threshold: 1024,
      disable: process.env.NODE_ENV === 'development'
    }) as PluginOption,
  ],

  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },

  // Development specific settings
  server: {
    hmr: {
      overlay: true
    },
  },

  // CSS settings
  css: {
    devSourcemap: true,
  },

  build: {
    sourcemap: true,
    rollupOptions: {
      output: {
        manualChunks: {
          vendor: [
            'vue',
            'vue-router',
            'pinia',
            'lodash'
          ],
          search: ['fuse.js']
        }
      }
    },
    terserOptions: {
      compress: {
        drop_console: process.env.NODE_ENV === 'production',
        drop_debugger: process.env.NODE_ENV === 'production'
      }
    }
  },

  optimizeDeps: {
    include: [
      'vue',
      'vue-router',
      'pinia',
      'lodash',
      'fuse.js',
    ]
  }
})
