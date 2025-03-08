<!-- PrivacyPolicy.vue -->
<template>
  <div class="min-h-screen bg-[rgb(3,5,21)] relative">
    <!-- Gradient Overlay -->
    <div class="absolute top-0 right-0 w-full h-full animated-gradient"></div>
    
    <AppHeader />

    <!-- Privacy Policy Section -->
    <div class="container mx-auto px-4 py-12">
      <div class="text-center max-w-4xl mx-auto mb-12">
        <h1 class="text-4xl md:text-6xl font-bold text-white mb-6 leading-tight">
          Privacy <span class="text-blue-400">Policy</span>
        </h1>
        <p class="text-xl text-gray-300">
          Last updated: {{ new Date().toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' }) }}
        </p>
      </div>

      <div class="max-w-4xl mx-auto bg-white/5 backdrop-filter backdrop-blur-lg rounded-lg p-8 shadow-lg">
        <div class="prose prose-invert max-w-none">
          <p class="text-gray-300 mb-6">
            This Privacy Policy describes how BetterNewTab ("we", "us" or "our") collects, uses, and shares your personal information when you use our website and browser extension (the "Service").
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Information We Collect</h2>
          
          <div class="text-gray-300">
            <p class="mb-4">We collect several types of information from and about users of our Service, including:</p>
            
            <ul class="list-disc pl-6 mb-6">
              <li class="mb-2"><strong>Personal Information:</strong> Email address and user name, collected through our authentication provider, Clerk.</li>
              <li class="mb-2"><strong>User Content:</strong> Links, URLs, and descriptions that you enter into the website.</li>
              <li class="mb-2"><strong>Search History:</strong> We store your last 500 search terms locally on your device. Search frequency is tracked and scored to provide better search suggestions. This data is stored locally on your device and is not transmitted to our servers.</li>
              <li class="mb-2"><strong>Keystrokes:</strong> We track keystrokes solely for the purpose of enabling keyboard shortcuts. These are not logged or stored.</li>
              <li class="mb-2"><strong>Search Terms:</strong> When you use our search suggestion feature, your search terms are sent to Brave to generate search suggestions using their API.</li>
              <li class="mb-2"><strong>Payment Information:</strong> When you subscribe to our Service, we collect billing information (such as your name, billing address, and payment method details) through our payment processor, Stripe. We do not store full payment card details on our servers; this data is securely handled by Stripe.</li>
              <li class="mb-2"><strong>Cookies and Similar Technologies:</strong> We may use cookies or similar technologies (e.g., via Clerk for authentication) to manage sessions and improve your experience. You can control cookies through your browser settings.</li>
            </ul>
          </div>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">How We Use Your Information</h2>
          
          <div class="text-gray-300">
            <p class="mb-4">We use the information we collect to:</p>
            
            <ul class="list-disc pl-6 mb-6">
              <li class="mb-2">Provide, maintain, and improve our Service</li>
              <li class="mb-2">Process and complete transactions, and send related information</li>
              <li class="mb-2">Send technical notices, updates, security alerts, and support messages</li>
              <li class="mb-2">Respond to your comments, questions, and requests</li>
              <li class="mb-2">Provide personalized content and search suggestions</li>
              <li class="mb-2">Process payments and manage subscriptions through Stripe</li>
            </ul>
          </div>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Data Storage and Transfer</h2>
          
          <p class="text-gray-300 mb-4">
            Your data is stored in Supabase, not on our servers. Data is transferred through our servers to Supabase. Our operations are based in Canada, but we use various services such as Clerk, Supabase, and Brave that operate globally, primarily in the United States.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Error Tracking and Monitoring</h2>
          
          <p class="text-gray-300 mb-4">
            We use Sentry for error tracking and performance monitoring. Sentry may associate your email address and ID with crash details and telemetry data. This information is used solely for improving our Service and troubleshooting issues.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Analytics and Telemetry</h2>
          
          <p class="text-gray-300 mb-4">
            We do not use any analytics or telemetry services that collect and store information about your use of the Service.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Third-Party Services</h2>
          
          <p class="text-gray-300 mb-4">
            Our Service uses third-party services that may collect information used to identify you:
          </p>
          <ul class="list-disc pl-6 mb-6 text-gray-300">
            <li class="mb-2"><strong>Clerk:</strong> For authentication services; <a href="https://clerk.com/legal/privacy" class="text-blue-400 hover:underline">Privacy Policy</a></li>
            <li class="mb-2"><strong>Supabase:</strong> For data storage; <a href="https://sentry.io/privacy/" class="text-blue-400 hover:underline">Privacy Policy</a></li>
            <li class="mb-2"><strong>Brave:</strong> For search suggestions; <a href="https://brave.com/privacy/browser/" class="text-blue-400 hover:underline">Privacy Policy</a></li>
            <li class="mb-2"><strong>Sentry:</strong> For error tracking; <a href="https://sentry.io/privacy/" class="text-blue-400 hover:underline">Privacy Policy</a></li>
            <li class="mb-2"><strong>Stripe:</strong> For processing payments and managing subscriptions. Stripe collects and processes your payment information in accordance with its <a href="https://stripe.com/privacy" class="text-blue-400 hover:underline">Privacy Policy</a>.</li>
          </ul>
          <p class="text-gray-300 mb-4">
            We encourage you to review the privacy policies of these third-party services.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Sharing Your Information</h2>

          <p class="text-gray-300 mb-4">
            We do not sell, trade, or rent your personal information to third parties. We may share your information with third-party service providers (such as Clerk, Supabase, Brave, Sentry, and Stripe) to perform functions on our behalf, as described in this Privacy Policy. We may also disclose your information if required by law, to protect our rights, or in connection with a business transfer (e.g., merger or acquisition).
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Data Retention</h2>
          
          <p class="text-gray-300 mb-4">
            We will retain your personal information only for as long as necessary to fulfill the purposes outlined in this Privacy Policy. When your data is no longer needed for the purposes for which it was collected, we will delete or anonymize it.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Security of Your Information</h2>

          <p class="text-gray-300 mb-4">
            We take reasonable measures to protect your personal information from unauthorized access, loss, misuse, or alteration. This includes using encryption for data in transit and relying on the security practices of our third-party providers (e.g., Supabase, Stripe). However, no method of transmission over the internet or electronic storage is 100% secure, and we cannot guarantee absolute security.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Your Rights</h2>
          
          <p class="text-gray-300 mb-4">
            Depending on your location, you may have certain rights regarding your personal information, such as:
          </p>
          <ul class="list-disc pl-6 mb-6 text-gray-300">
            <li class="mb-2">The right to access personal information we hold about you</li>
            <li class="mb-2">The right to request correction or deletion of your personal information</li>
            <li class="mb-2">The right to restrict or object to our processing of your personal information</li>
            <li class="mb-2">The right to data portability</li>
          </ul>
          <p class="text-gray-300 mb-4">
            To exercise these rights, please contact us using the contact information provided below.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Children's Privacy</h2>
          <p class="text-gray-300 mb-4">
            Our Service is not intended for use by individuals under the age of 13. We do not knowingly collect personal information from children under 13. If we become aware that a child under 13 has provided us with personal information, we will take steps to delete it.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Changes to This Privacy Policy</h2>
          
          <p class="text-gray-300 mb-4">
            We may update our Privacy Policy from time to time. We will notify you of any changes by posting the new Privacy Policy on this page and updating the "Last updated" date.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Contact Us</h2>
          
          <p class="text-gray-300 mb-4">
            If you have any questions about this Privacy Policy, please contact us:
          </p>
          <ul class="list-disc pl-6 mb-6 text-gray-300">
            <li class="mb-2">By email: <a href="mailto:support@betternewtab.com" class="text-blue-400 hover:underline">support@betternewtab.com</a></li>
            <li class="mb-2">By visiting our <a href="/contact" class="text-blue-400 hover:underline">contact page</a></li>
          </ul>
        </div>
      </div>
    </div>

    <AppFooter />
  </div>
</template>

<script setup lang="ts">
import AppHeader from '@/components/AppHeader.vue';
import AppFooter from '@/components/AppFooter.vue';

// The rest of the script setup code is no longer needed as it's moved to AppHeader
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

.prose {
  max-width: 100%;
}

.prose h2 {
  color: white;
  margin-top: 1.5rem;
  margin-bottom: 1rem;
}

.prose p, .prose ul, .prose li {
  color: rgb(209, 213, 219);
}

.prose a {
  color: rgb(96, 165, 250);
  text-decoration: none;
}

.prose a:hover {
  text-decoration: underline;
}

div {
  white-space: normal !important;
  overflow: visible !important;
  text-overflow: clip !important;
}
</style>