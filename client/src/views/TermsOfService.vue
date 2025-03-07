<!-- TermsOfService.vue -->
<template>
  <div class="min-h-screen bg-[rgb(3,5,21)] relative">
    <!-- Gradient Overlay -->
    <div class="absolute top-0 right-0 w-full h-full animated-gradient"></div>
    
    <AppHeader />

    <!-- Terms of Service Section -->
    <div class="container mx-auto px-4 py-12">
      <div class="text-center max-w-4xl mx-auto mb-12">
        <h1 class="text-4xl md:text-6xl font-bold text-white mb-6 leading-tight">
          Terms of <span class="text-blue-400">Service</span>
        </h1>
        <p class="text-xl text-gray-300">
          Last updated: {{ new Date().toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' }) }}
        </p>
      </div>

      <div class="max-w-4xl mx-auto bg-white/5 backdrop-filter backdrop-blur-lg rounded-lg p-8 shadow-lg">
        <div class="prose prose-invert max-w-none">
          <p class="text-gray-300 mb-6">
            Please read these Terms of Service ("Terms", "Terms of Service") carefully before using the BetterNewTab website and browser extension (the "Service") operated by BetterNewTab ("us", "we", or "our").
          </p>
          
          <p class="text-gray-300 mb-6">
            Your access to and use of the Service is conditioned on your acceptance of and compliance with these Terms. These Terms apply to all visitors, users, and others who access or use the Service.
          </p>
          
          <p class="text-gray-300 mb-6">
            By accessing or using the Service you agree to be bound by these Terms. If you disagree with any part of the terms, then you may not access the Service.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">1. Accounts</h2>
          
          <p class="text-gray-300 mb-4">
            When you create an account with us, you must provide information that is accurate, complete, and current at all times. Failure to do so constitutes a breach of the Terms, which may result in immediate termination of your account on our Service.
          </p>
          
          <p class="text-gray-300 mb-4">
            You are responsible for safeguarding the password that you use to access the Service and for any activities or actions under your password, whether your password is with our Service or a third-party service.
          </p>
          
          <p class="text-gray-300 mb-4">
            You agree not to disclose your password to any third party. You must notify us immediately upon becoming aware of any breach of security or unauthorized use of your account.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">2. Content</h2>
          
          <p class="text-gray-300 mb-4">
            Our Service allows you to post, link, store, share and otherwise make available certain information, text, graphics, links, or other material ("Content"). You are responsible for the Content that you post on or through the Service, including its legality, reliability, and appropriateness.
          </p>
          
          <p class="text-gray-300 mb-4">
            By posting Content on or through the Service, you represent and warrant that: (i) the Content is yours (you own it) or you have the right to use it and grant us the rights and license as provided in these Terms, and (ii) that the posting of your Content on or through the Service does not violate the privacy rights, publicity rights, copyrights, contract rights or any other rights of any person.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">3. Accounts and Subscription</h2>
          
          <p class="text-gray-300 mb-4">
            Some features of the Service are available only with a paid subscription. By selecting a subscription, you agree to pay the subscription fees indicated for that service. Subscription fees are not refundable except as expressly stated in these Terms.
          </p>
          
          <p class="text-gray-300 mb-4">
            We may change the fees and charges in effect or add new fees and charges from time to time, but we will give you advance notice of these changes by email.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">4. Intellectual Property</h2>
          
          <p class="text-gray-300 mb-4">
            The Service and its original content (excluding Content provided by users), features, and functionality are and will remain the exclusive property of BetterNewTab and its licensors. The Service is protected by copyright, trademark, and other laws of both the United States and Canada. Our trademarks and trade dress may not be used in connection with any product or service without the prior written consent of BetterNewTab.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">5. Links to Other Websites</h2>
          
          <p class="text-gray-300 mb-4">
            Our Service may contain links to third-party websites or services that are not owned or controlled by BetterNewTab.
          </p>
          
          <p class="text-gray-300 mb-4">
            BetterNewTab has no control over, and assumes no responsibility for, the content, privacy policies, or practices of any third-party websites or services. You further acknowledge and agree that BetterNewTab shall not be responsible or liable, directly or indirectly, for any damage or loss caused or alleged to be caused by or in connection with the use of or reliance on any such content, goods, or services available on or through any such websites or services.
          </p>
          
          <p class="text-gray-300 mb-4">
            We strongly advise you to read the terms and conditions and privacy policies of any third-party websites or services that you visit.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">6. Termination</h2>
          
          <p class="text-gray-300 mb-4">
            We may terminate or suspend your account immediately, without prior notice or liability, for any reason whatsoever, including without limitation if you breach the Terms.
          </p>
          
          <p class="text-gray-300 mb-4">
            Upon termination, your right to use the Service will immediately cease. If you wish to terminate your account, you may simply discontinue using the Service or contact us to delete your account.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">7. Limitation of Liability</h2>
          
          <p class="text-gray-300 mb-4">
            In no event shall BetterNewTab, nor its directors, employees, partners, agents, suppliers, or affiliates, be liable for any indirect, incidental, special, consequential or punitive damages, including without limitation, loss of profits, data, use, goodwill, or other intangible losses, resulting from (i) your access to or use of or inability to access or use the Service; (ii) any conduct or content of any third party on the Service; (iii) any content obtained from the Service; and (iv) unauthorized access, use or alteration of your transmissions or content, whether based on warranty, contract, tort (including negligence) or any other legal theory, whether or not we have been informed of the possibility of such damage.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">8. Disclaimer</h2>
          
          <p class="text-gray-300 mb-4">
            Your use of the Service is at your sole risk. The Service is provided on an "AS IS" and "AS AVAILABLE" basis. The Service is provided without warranties of any kind, whether express or implied, including, but not limited to, implied warranties of merchantability, fitness for a particular purpose, non-infringement or course of performance.
          </p>
          
          <p class="text-gray-300 mb-4">
            BetterNewTab, its subsidiaries, affiliates, and its licensors do not warrant that a) the Service will function uninterrupted, secure or available at any particular time or location; b) any errors or defects will be corrected; c) the Service is free of viruses or other harmful components; or d) the results of using the Service will meet your requirements.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">9. Governing Law</h2>
          
          <p class="text-gray-300 mb-4">
            These Terms shall be governed and construed in accordance with the laws of Canada, without regard to its conflict of law provisions.
          </p>
          
          <p class="text-gray-300 mb-4">
            Our failure to enforce any right or provision of these Terms will not be considered a waiver of those rights. If any provision of these Terms is held to be invalid or unenforceable by a court, the remaining provisions of these Terms will remain in effect. These Terms constitute the entire agreement between us regarding our Service, and supersede and replace any prior agreements we might have between us regarding the Service.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">10. Changes</h2>
          
          <p class="text-gray-300 mb-4">
            We reserve the right, at our sole discretion, to modify or replace these Terms at any time. If a revision is material, we will try to provide at least 30 days' notice prior to any new terms taking effect. What constitutes a material change will be determined at our sole discretion.
          </p>
          
          <p class="text-gray-300 mb-4">
            By continuing to access or use our Service after those revisions become effective, you agree to be bound by the revised terms. If you do not agree to the new terms, please stop using the Service.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">11. Contact Us</h2>
          
          <p class="text-gray-300 mb-4">
            If you have any questions about these Terms, please contact us:
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