<template>
  <div class="min-h-screen">
    <AppHeader />

    <section class="tp-section">
      <div class="tp-container">
        <div class="text-center max-w-3xl mx-auto mb-12">
          <h1 class="text-3xl md:text-4xl font-bold mb-4">
            Terms of <span class="tp-accent">Service</span>
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
            Please read these Terms of Service ("Terms", "Terms of Service") carefully before using
            the OmegaTab website and browser extension (the "Service") operated by OmegaTab ("us",
            "we", or "our").
          </p>

          <p class="tp-secondary">
            Your access to and use of the Service is conditioned on your acceptance of and
            compliance with these Terms. These Terms apply to all visitors, users, and others who
            access or use the Service.
          </p>

          <p class="tp-secondary">
            By accessing or using the Service you agree to be bound by these Terms. If you disagree
            with any part of the terms, then you may not access the Service.
          </p>

          <h2>1. Accounts</h2>

          <p class="tp-secondary">
            When you create an account with us, you must provide information that is accurate,
            complete, and current at all times. Failure to do so constitutes a breach of the Terms,
            which may result in immediate termination of your account on our Service.
          </p>

          <p class="tp-secondary">
            You are responsible for safeguarding the password that you use to access the Service and
            for any activities or actions under your password.
          </p>

          <p class="tp-secondary">
            You agree not to disclose your password to any third party. You must notify us
            immediately upon becoming aware of any breach of security or unauthorized use of your
            account.
          </p>

          <h2>2. Content</h2>

          <p class="tp-secondary">
            Our Service allows you to post, link, store, share and otherwise make available certain
            information, text, graphics, links, or other material ("Content"). You are responsible
            for the Content that you post on or through the Service, including its legality,
            reliability, and appropriateness.
          </p>

          <p class="tp-secondary">
            By posting Content on or through the Service, you represent and warrant that: (i) the
            Content is yours or you have the right to use it and grant us the rights and license as
            provided in these Terms, and (ii) the posting of your Content does not violate the
            privacy rights, publicity rights, copyrights, contract rights or any other rights of any
            person.
          </p>

          <h2>3. Accounts and Subscription</h2>

          <p class="tp-secondary">
            Some features of the Service are available only with a paid subscription. By selecting a
            subscription, you agree to pay the subscription fees indicated for that service.
            Subscription fees are not refundable except as expressly stated in these Terms.
          </p>

          <p class="tp-secondary">
            We may change the fees and charges in effect or add new fees and charges from time to
            time, but we will give you advance notice of these changes by email.
          </p>

          <h2>4. Intellectual Property</h2>

          <p class="tp-secondary">
            The Service and its original content (excluding Content provided by users), features,
            and functionality are and will remain the exclusive property of OmegaTab and its
            licensors. The Service is protected by copyright, trademark, and other laws of both the
            United States and Canada.
          </p>

          <h2>5. Links to Other Websites</h2>

          <p class="tp-secondary">
            Our Service may contain links to third-party websites or services that are not owned or
            controlled by OmegaTab.
          </p>

          <p class="tp-secondary">
            OmegaTab has no control over, and assumes no responsibility for, the content, privacy
            policies, or practices of any third-party websites or services. You further acknowledge
            and agree that OmegaTab shall not be responsible or liable for any damage or loss caused
            by or in connection with the use of any such content, goods, or services.
          </p>

          <h2>6. Termination</h2>

          <p class="tp-secondary">
            We may terminate or suspend your account immediately, without prior notice or liability,
            for any reason whatsoever, including without limitation if you breach the Terms.
          </p>

          <p class="tp-secondary">
            Upon termination, your right to use the Service will immediately cease. If you wish to
            terminate your account, you may simply discontinue using the Service or contact us to
            delete your account.
          </p>

          <h2>7. Limitation of Liability</h2>

          <p class="tp-secondary">
            In no event shall OmegaTab, nor its directors, employees, partners, agents, suppliers,
            or affiliates, be liable for any indirect, incidental, special, consequential or
            punitive damages, including without limitation, loss of profits, data, use, goodwill, or
            other intangible losses, resulting from your access to or use of or inability to access
            or use the Service.
          </p>

          <h2>8. Disclaimer</h2>

          <p class="tp-secondary">
            Your use of the Service is at your sole risk. The Service is provided on an "AS IS" and
            "AS AVAILABLE" basis. The Service is provided without warranties of any kind, whether
            express or implied.
          </p>

          <h2>9. Governing Law</h2>

          <p class="tp-secondary">
            These Terms shall be governed and construed in accordance with the laws of Canada,
            without regard to its conflict of law provisions.
          </p>

          <h2>10. Changes</h2>

          <p class="tp-secondary">
            We reserve the right, at our sole discretion, to modify or replace these Terms at any
            time. If a revision is material, we will try to provide at least 30 days' notice prior
            to any new terms taking effect.
          </p>

          <p class="tp-secondary">
            By continuing to access or use our Service after those revisions become effective, you
            agree to be bound by the revised terms. If you do not agree to the new terms, please
            stop using the Service.
          </p>

          <h2>11. Contact Us</h2>

          <p class="tp-secondary mb-4">
            If you have any questions about these Terms, please contact us:
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
