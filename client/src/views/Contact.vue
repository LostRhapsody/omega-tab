<!-- Contact.vue -->
<template>
  <div class="min-h-screen bg-[rgb(3,5,21)] relative">
    <!-- Gradient Overlay -->
    <div class="absolute top-0 right-0 w-full h-full animated-gradient"></div>
    
    <AppHeader />

    <div class="container mx-auto px-4 py-24">
      <div class="text-center max-w-4xl mx-auto mb-16">
        <h1 class="text-4xl md:text-6xl font-bold text-white mb-6 leading-tight">
          Get in <span class="text-blue-400">touch</span> with us
        </h1>
        <p class="text-xl text-gray-300">
          We'd love to hear from you! Whether you have a question about our product, pricing, or just want to say hello, we're here to help.
        </p>
      </div>

      <div class="grid grid-cols-1 md:grid-cols-2 gap-12 max-w-6xl mx-auto">
        <!-- Contact Info -->
        <div class="bg-white/5 backdrop-filter backdrop-blur-lg rounded-lg p-8 shadow-lg">
          <h2 class="text-2xl font-bold text-white mb-6">Contact Information</h2>
          
          <div class="mb-6">
            <h3 class="text-xl font-semibold text-blue-400 mb-2">Email</h3>
            <p class="text-gray-300 flex items-center">
              <v-icon icon="mdi-email" class="mr-3" />
              <a href="mailto:support@betternewtab.com" class="hover:text-white transition-colors">support@betternewtab.com</a>
            </p>
          </div>

          <div class="mb-6">
            <h3 class="text-xl font-semibold text-blue-400 mb-2">Phone</h3>
            <p class="text-gray-300 flex items-center">
              <v-icon icon="mdi-phone" class="mr-3" />
              <a href="tel:+1-555-123-4567" class="hover:text-white transition-colors">+1 (555) 123-4567</a>
            </p>
          </div>

          <div class="mb-6">
            <h3 class="text-xl font-semibold text-blue-400 mb-2">Office Hours</h3>
            <p class="text-gray-300 flex items-center">
              <v-icon icon="mdi-clock-outline" class="mr-3" />
              Monday - Friday: 9am - 5pm PST
            </p>
          </div>

          <div>
            <h3 class="text-xl font-semibold text-blue-400 mb-4">Connect With Us</h3>
            <div class="flex space-x-4">
              <a href="https://twitter.com/betternewtab" target="_blank" class="text-gray-300 hover:text-white transition-colors">
                <v-icon icon="mdi-twitter" size="large" />
              </a>
              <a href="https://github.com/betternewtab" target="_blank" class="text-gray-300 hover:text-white transition-colors">
                <v-icon icon="mdi-github" size="large" />
              </a>
              <a href="https://linkedin.com/company/betternewtab" target="_blank" class="text-gray-300 hover:text-white transition-colors">
                <v-icon icon="mdi-linkedin" size="large" />
              </a>
            </div>
          </div>
        </div>

        <!-- Contact Form -->
        <div class="bg-white/5 backdrop-filter backdrop-blur-lg rounded-lg p-8 shadow-lg">
          <h2 class="text-2xl font-bold text-white mb-6">Send Us a Message</h2>
          
          <form @submit.prevent="submitForm">
            <div class="mb-4">
              <label for="name" class="block text-gray-300 mb-2">Name</label>
              <input
                id="name"
                v-model="form.name"
                type="text"
                class="w-full bg-white/10 border border-gray-600 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-blue-400 transition-colors"
                placeholder="Your Name"
                required
              />
            </div>

            <div class="mb-4">
              <label for="email" class="block text-gray-300 mb-2">Email</label>
              <input
                id="email"
                v-model="form.email"
                type="email"
                class="w-full bg-white/10 border border-gray-600 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-blue-400 transition-colors"
                placeholder="your.email@example.com"
                required
              />
            </div>

            <div class="mb-4">
              <label for="subject" class="block text-gray-300 mb-2">Subject</label>
              <input
                id="subject"
                v-model="form.subject"
                type="text"
                class="w-full bg-white/10 border border-gray-600 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-blue-400 transition-colors"
                placeholder="What is this about?"
                required
              />
            </div>

            <div class="mb-6">
              <label for="message" class="block text-gray-300 mb-2">Message</label>
              <textarea
                id="message"
                v-model="form.message"
                rows="5"
                class="w-full bg-white/10 border border-gray-600 rounded-lg px-4 py-2 text-white focus:outline-none focus:border-blue-400 transition-colors"
                placeholder="Tell us how we can help you..."
                required
              ></textarea>
            </div>

            <button
              type="submit"
              :disabled="isSubmitting"
              class="px-6 py-3 rounded-full bg-blue-500/70 text-white hover:bg-blue-600 transition-colors w-full flex justify-center items-center"
            >
              <v-progress-circular v-if="isSubmitting" indeterminate color="white" size="20" width="2" class="mr-2"></v-progress-circular>
              {{ isSubmitting ? 'Sending...' : 'Send Message' }}
            </button>

            <div v-if="formSubmitted" class="mt-4 p-3 bg-green-500/20 border border-green-500 rounded-lg text-green-400 text-center">
              Thank you for your message! We'll get back to you soon.
            </div>
          </form>
        </div>
      </div>

      <!-- FAQ Section -->
      <div class="max-w-4xl mx-auto mt-24">
        <h2 class="text-3xl font-bold text-white mb-12 text-center">Frequently Asked Questions</h2>
        
        <v-expansion-panels variant="accordion" class="bg-transparent">
          <v-expansion-panel v-for="(faq, index) in faqs" :key="index" class="!bg-white/5 !text-white rounded-lg">
            <v-expansion-panel-title class="!text-white">
              {{ faq.question }}
            </v-expansion-panel-title>
            <v-expansion-panel-text class="!text-gray-300">
              {{ faq.answer }}
            </v-expansion-panel-text>
          </v-expansion-panel>
        </v-expansion-panels>
      </div>
    </div>

    <AppFooter />
  </div>
</template>

<script setup lang="ts">
import AppHeader from '@/components/AppHeader.vue';
import AppFooter from '@/components/AppFooter.vue';
import { ref } from 'vue';

const form = ref({
  name: '',
  email: '',
  subject: '',
  message: ''
});

const isSubmitting = ref(false);
const formSubmitted = ref(false);
const showSnackbar = ref(false);
const snackbarText = ref('');
const snackbarColor = ref('');

const faqs = ref([
  {
    question: 'What is BetterNewTab?',
    answer: 'BetterNewTab is a browser extension that enhances your new tab page with customizable links, powerful search features, and productivity tools.'
  },
  {
    question: 'How do I install BetterNewTab?',
    answer: 'You can install BetterNewTab from our website or directly from your browser\'s extension store.'
  },
  {
    question: 'Is BetterNewTab free to use?',
    answer: 'Yes, BetterNewTab offers a free tier with basic features. Premium plans are available for additional functionality.'
  }
]);

const submitForm = async () => {
  isSubmitting.value = true;
  
  try {
    // Here you would typically make an API call to submit the form
    // For demo purposes, we'll just simulate a delay
    await new Promise(resolve => setTimeout(resolve, 1500));
    
    // Reset form and show success message
    form.value = { name: '', email: '', subject: '', message: '' };
    formSubmitted.value = true;
    
    // After 5 seconds, hide the success message
    setTimeout(() => {
      formSubmitted.value = false;
    }, 5000);
    
    snackbarText.value = 'Message sent successfully!';
    snackbarColor.value = 'success';
    showSnackbar.value = true;
  } catch (error) {
    snackbarText.value = 'Failed to send message. Please try again.';
    snackbarColor.value = 'error';
    showSnackbar.value = true;
  } finally {
    isSubmitting.value = false;
  }
};
</script>

<style scoped>
.v-text-field :deep(.v-field__input) {
  color: white !important;
}

.v-text-field :deep(.v-field__outline) {
  --v-field-border-opacity: 0.2;
}

.animated-gradient {
  animation: animateBg 12s ease infinite;
  background-image: linear-gradient(45deg, rgba(255, 255, 255, 0.6), rgb(141, 145, 255));
  background-size: 400% 400%;
  opacity: 0.5;
}

@keyframes animateBg {
  0% {
    background-position: 0% 50%;
  }

  50% {
    background-position: 100% 50%;
  }

  100% {
    background-position: 0% 50%;
  }
}

/* Style the expansion panels */
.v-expansion-panels :deep(.v-expansion-panel-title) {
  background-color: rgba(255, 255, 255, 0.05) !important;
  color: white !important;
}

.v-expansion-panels :deep(.v-expansion-panel-text) {
  background-color: rgba(255, 255, 255, 0.03) !important;
  color: rgb(209, 213, 219) !important;
}

/* Form input focus effects */
input:focus, textarea:focus {
  box-shadow: 0 0 0 2px rgba(96, 165, 250, 0.3);
}

div {
  white-space: normal !important;
  text-overflow: clip !important;
}
</style>