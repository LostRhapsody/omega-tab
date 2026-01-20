<template>
  <div class="min-h-screen">
    <AppHeader />

    <section class="tp-section">
      <div class="tp-container">
        <div class="text-center max-w-3xl mx-auto mb-12">
          <h1 class="text-3xl md:text-4xl font-bold mb-4">
            Privacy <span class="tp-accent">Policy</span>
          </h1>
          <p class="tp-secondary">
            Last updated:
            {{
              new Date().toLocaleDateString("en-US", {
                month: "long",
                day: "numeric",
                year: "numeric",
              })
            }}
          </p>
        </div>

        <div class="prose-container tp-card max-w-3xl mx-auto">
          <p class="tp-secondary mb-6">
            This Privacy Policy describes how OmegaTab ("we", "us" or "our") collects, uses, and
            shares your personal information when you use our website and browser extension (the
            "Service").
          </p>

          <h2>Information We Collect</h2>

          <p class="tp-secondary mb-4">
            We collect several types of information from and about users of our Service, including:
          </p>

          <ul class="tp-secondary">
            <li>
              <strong>Personal Information:</strong> Email address, collected when you create an
              account.
            </li>
            <li>
              <strong>User Content:</strong> Links, URLs, and descriptions that you enter into the
              website.
            </li>
            <li>
              <strong>Search History:</strong> We store your last 500 search terms locally on your
              device. Search frequency is tracked and scored to provide better search suggestions.
              This data is stored locally on your device and is not transmitted to our servers.
            </li>
            <li>
              <strong>Keystrokes:</strong> We track keystrokes solely for the purpose of enabling
              keyboard shortcuts. These are not logged or stored.
            </li>
            <li>
              <strong>Search Terms:</strong> When you use our search suggestion feature, your search
              terms are sent to Brave to generate search suggestions using their API.
            </li>
            <li>
              <strong>Payment Information:</strong> When you subscribe to our Service, we collect
              billing information through Stripe. We do not store full payment card details on our
              servers.
            </li>
            <li>
              <strong>Cookies and Similar Technologies:</strong> We may use cookies to manage
              sessions and improve your experience.
            </li>
          </ul>

          <h2>How We Use Your Information</h2>

          <p class="tp-secondary mb-4">We use the information we collect to:</p>

          <ul class="tp-secondary">
            <li>Provide, maintain, and improve our Service</li>
            <li>Process and complete transactions</li>
            <li>Send technical notices, updates, and support messages</li>
            <li>Respond to your comments, questions, and requests</li>
            <li>Provide personalized content and search suggestions</li>
            <li>Process payments and manage subscriptions through Stripe</li>
          </ul>

          <h2>Data Storage and Transfer</h2>

          <p class="tp-secondary">
            Your data is stored in Supabase, not on our servers. Data is transferred through our
            servers to Supabase. Our operations are based in Canada, but we use various services
            such as Supabase and Brave that operate globally, primarily in the United States.
          </p>

          <h2>Error Tracking and Monitoring</h2>

          <p class="tp-secondary">
            We use Sentry for error tracking and performance monitoring. Sentry may associate your
            email address and ID with crash details and telemetry data. This information is used
            solely for improving our Service.
          </p>

          <h2>Analytics and Telemetry</h2>

          <p class="tp-secondary">
            We do not use any analytics or telemetry services that collect and store information
            about your use of the Service.
          </p>

          <h2>Third-Party Services</h2>

          <p class="tp-secondary mb-4">
            Our Service uses third-party services that may collect information used to identify you:
          </p>

          <ul class="tp-secondary">
            <li>
              <strong>Supabase:</strong> For data storage;
              <a href="https://supabase.com/privacy">Privacy Policy</a>
            </li>
            <li>
              <strong>Brave:</strong> For search suggestions;
              <a href="https://brave.com/privacy/browser/">Privacy Policy</a>
            </li>
            <li>
              <strong>Sentry:</strong> For error tracking;
              <a href="https://sentry.io/privacy/">Privacy Policy</a>
            </li>
            <li>
              <strong>Stripe:</strong> For processing payments;
              <a href="https://stripe.com/privacy">Privacy Policy</a>
            </li>
          </ul>

          <h2>Sharing Your Information</h2>

          <p class="tp-secondary">
            We do not sell, trade, or rent your personal information to third parties. We may share
            your information with third-party service providers to perform functions on our behalf,
            as described in this Privacy Policy.
          </p>

          <h2>Data Retention</h2>

          <p class="tp-secondary">
            We will retain your personal information only for as long as necessary to fulfill the
            purposes outlined in this Privacy Policy.
          </p>

          <h2>Security of Your Information</h2>

          <p class="tp-secondary">
            We take reasonable measures to protect your personal information from unauthorized
            access, loss, misuse, or alteration. This includes using encryption for data in transit
            and relying on the security practices of our third-party providers.
          </p>

          <h2>Your Rights</h2>

          <p class="tp-secondary mb-4">
            Depending on your location, you may have certain rights regarding your personal
            information:
          </p>

          <ul class="tp-secondary">
            <li>The right to access personal information we hold about you</li>
            <li>The right to request correction or deletion of your personal information</li>
            <li>The right to restrict or object to our processing of your personal information</li>
            <li>The right to data portability</li>
          </ul>

          <h2>Children's Privacy</h2>

          <p class="tp-secondary">
            Our Service is not intended for use by individuals under the age of 13. We do not
            knowingly collect personal information from children under 13.
          </p>

          <h2>Changes to This Privacy Policy</h2>

          <p class="tp-secondary">
            We may update our Privacy Policy from time to time. We will notify you of any changes by
            posting the new Privacy Policy on this page.
          </p>

          <h2>Contact Us</h2>

          <p class="tp-secondary mb-4">
            If you have any questions about this Privacy Policy, please contact us:
          </p>

          <ul class="tp-secondary">
            <li>
              By email:
              <a href="mailto:support@omega-tab.evanrobertson.dev"
                >support@omega-tab.evanrobertson.dev</a
              >
            </li>
            <li>By visiting our <a href="/contact">contact page</a></li>
          </ul>
        </div>
      </div>
    </section>

    <AppFooter />
  </div>
</template>

<script setup lang="ts">
import AppFooter from "@/components/AppFooter.vue";
import AppHeader from "@/components/AppHeader.vue";
</script>

<style scoped>
.prose-container h2 {
  font-size: var(--tp-text-xl);
  font-weight: var(--tp-font-bold);
  margin-top: var(--tp-space-8);
  margin-bottom: var(--tp-space-4);
}

.prose-container ul {
  list-style: disc;
  padding-left: var(--tp-space-6);
  margin-bottom: var(--tp-space-6);
}

.prose-container li {
  margin-bottom: var(--tp-space-2);
}

.prose-container a {
  color: var(--tp-accent);
  text-decoration: underline;
  text-underline-offset: 2px;
}

.prose-container a:hover {
  color: var(--tp-text-primary);
}

.prose-container p {
  margin-bottom: var(--tp-space-4);
}
</style>
