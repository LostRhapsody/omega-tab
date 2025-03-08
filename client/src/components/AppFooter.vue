<template>
  <div class="pt-24 text-center bottom-0 w-full">
    <footer
      class="px-4 py-4 min-h-24 flex align-center border-t border-gray-700 bg-white/5 backdrop-filter backdrop-blur-3xl relative z-10">
      <div class="container mx-auto">
        <!-- Desktop View -->
        <div class="hidden md:flex justify-between items-center">
          <!-- Company Name -->
          <div class="flex items-center space-x-2">
            <span class="text-white text-xl font-semibold">BetterNewTab_</span>
          </div>

          <!-- Footer Links -->
          <div class="flex space-x-8 border border-gray-300 px-6 py-4 rounded-full">
            <a href="/docs/getting-started" class="text-gray-300 hover:text-white transition-colors">Getting Started</a>
            <span class="text-gray-300 cursor-default">•</span>
            <a href="/privacy-policy" class="text-gray-300 hover:text-white transition-colors">Privacy Policy</a>
            <span class="text-gray-300 cursor-default">•</span>
            <a href="/terms-of-service" class="text-gray-300 hover:text-white transition-colors">Terms of Service</a>
            <span class="text-gray-300 cursor-default">•</span>
            <a href="/contact" class="text-gray-300 hover:text-white transition-colors">Contact</a>
          </div>

          <!-- Social Media Links -->
          <div class="flex items-center space-x-4">
            <a href="https://twitter.com/betternewtab" target="_blank" class="text-gray-300 hover:text-white transition-colors">
              <v-icon icon="mdi-twitter" />
            </a>
            <a href="https://github.com/betternewtab" target="_blank" class="text-gray-300 hover:text-white transition-colors">
              <v-icon icon="mdi-github" />
            </a>
            <a href="https://linkedin.com/company/betternewtab" target="_blank" class="text-gray-300 hover:text-white transition-colors">
              <v-icon icon="mdi-linkedin" />
            </a>
          </div>
        </div>

        <!-- Mobile View -->
        <div class="flex md:hidden flex-col items-center space-y-4">
          <!-- Company Name -->
          <div class="flex items-center space-x-2">
            <span class="text-white text-xl font-semibold">BetterNewTab_</span>
          </div>

          <!-- Footer Links -->
          <div class="flex flex-wrap justify-center gap-3 text-sm">
            <a href="/docs/getting-started" class="text-gray-300 hover:text-white transition-colors">Getting Started</a>
            <a href="/privacy-policy" class="text-gray-300 hover:text-white transition-colors">Privacy Policy</a>
            <a href="/terms-of-service" class="text-gray-300 hover:text-white transition-colors">Terms of Service</a>
            <a href="/contact" class="text-gray-300 hover:text-white transition-colors">Contact</a>
          </div>

          <!-- Social Media Links -->
          <div class="flex items-center space-x-4">
            <a href="https://twitter.com/betternewtab" target="_blank" class="text-gray-300 hover:text-white transition-colors">
              <v-icon icon="mdi-twitter" />
            </a>
            <a href="https://github.com/betternewtab" target="_blank" class="text-gray-300 hover:text-white transition-colors">
              <v-icon icon="mdi-github" />
            </a>
            <a href="https://linkedin.com/company/betternewtab" target="_blank" class="text-gray-300 hover:text-white transition-colors">
              <v-icon icon="mdi-linkedin" />
            </a>
          </div>
        </div>
      </div>
    </footer>
  </div>
</template>

<script setup>
// No additional logic needed for the footer
</script>

<style scoped>
/* Footer specific styles can be added here if needed */
</style>