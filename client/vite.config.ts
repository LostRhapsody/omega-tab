import { sentryVitePlugin } from "@sentry/vite-plugin";
import { fileURLToPath, URL } from 'node:url'
import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import vueJsx from '@vitejs/plugin-vue-jsx'
import vueDevTools from 'vite-plugin-vue-devtools'
import vuetify from 'vite-plugin-vuetify'
import compression from 'vite-plugin-compression'

// https://vite.dev/config/
export default defineConfig({
  plugins: [
    vue({
      template: {
        compilerOptions: {
          // Enable hoisting of static content for better performance
          hoistStatic: true,
          // Cache static trees for better SSR performance
          cacheHandlers: true,
          // Optimize SSR compilation
          ssrCssVars: 'none'
        }
      }
    }),
    vueJsx(),
    vueDevTools(),
    vuetify({
      autoImport: true
    }),
    compression({
      algorithm: 'gzip',
      ext: '.gz',
      // Compress files larger than 1kb
      threshold: 1024,
      // Only compress files that would benefit from it
      filter: (file) => /\.(js|mjs|json|css|html)$/i.test(file),
      // Disable compression during development
      disable: process.env.NODE_ENV === 'development'
    }),
    sentryVitePlugin({
      org: "better-new-tab",
      project: "betternewtab-vue"
    }),
  ],

  resolve: {
    alias: {
      '@': fileURLToPath(new URL('./src', import.meta.url))
    },
  },

  // Development specific settings
  server: {
    proxy: {
      "/docs": {
        target: "http://localhost:5174", // VitePress dev server
        rewrite: (path) => path, // Keep /docs in the path
      },
    },
    hmr: {
      overlay: true
    },
    // Optimize dev server performance
    watch: {
      usePolling: false,
      // Decrease CPU usage in dev mode
      interval: 1000,
    }
  },

  // CSS optimization settings
  css: {
    devSourcemap: true,
    preprocessorOptions: {
      scss: {
        additionalData: '@import "@/assets/css/variables.scss";'
      }
    }
  },

  build: {
    sourcemap: true,
    manifest: true,
    // Improve build performance
    target: 'esnext',
    // Reduce disk space usage
    chunkSizeWarningLimit: 1000,
    // Better caching with content hash
    cssCodeSplit: true,
    rollupOptions: {
      // external: ['vue', 'vue-router', 'pinia', 'vuetify'],
      output: {
        manualChunks: (id) => {
          // Vendor chunks
          if (id.includes('node_modules')) {
            if (id.includes('vuetify')) return 'vendor-vuetify'
            if (id.includes('lodash')) return 'vendor-lodash'
            if (id.includes('@clerk')) return 'vendor-clerk'
            if (id.includes('@sentry')) return 'vendor-sentry'
            if (id.includes('@stripe')) return 'vendor-stripe'
            if (id.includes('fuse.js')) return 'vendor-fuse'
            return 'vendor' // other vendor modules
          }
          // Feature based code splitting
          if (id.includes('/src/components/')) {
            // Split large components into separate chunks
            if (id.includes('SearchBar')) return 'component-search'
            if (id.includes('LinkColumns')) return 'component-links'
            return 'components'
          }
          if (id.includes('/src/views/')) return 'views'
          if (id.includes('/src/stores/')) return 'stores'
          if (id.includes('/src/utils/')) return 'utils'
        },
        // Use content hash for better caching
        chunkFileNames: 'assets/[name].[hash].js',
        entryFileNames: 'assets/[name].[hash].js',
        assetFileNames: 'assets/[name].[hash].[ext]'
      },
    },
    minify: false,
    terserOptions: {
      compress: {
        drop_console: process.env.NODE_ENV === 'production',
        drop_debugger: process.env.NODE_ENV === 'production',
        pure_funcs: process.env.NODE_ENV === 'production' ? ['console.log'] : []
      }
    },
    // Improve chunk loading
    dynamicImportVarsOptions: {
      warnOnError: true,
      exclude: [/\.(vue|md)$/]
    },
  },

  // Optimize dependency pre-bundling
  optimizeDeps: {
    include: [
      'vue',
      'vue-router',
      'pinia',
      'vuetify',
      'lodash',
      '@clerk/clerk-js',
      'fuse.js',
    ],
    exclude: ['@sentry/vue'], // Exclude Sentry as it's not needed in dev
    // Add runtime optimization
    esbuildOptions: {
      target: 'esnext',
      supported: {
        'top-level-await': true
      },
    },
  }
})