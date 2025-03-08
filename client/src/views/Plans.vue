<template>
  <div class="plans-page min-h-screen py-12 px-4">
    <div class="container mx-auto">
      <div class="mb-8">
        <v-btn @click="handleBack" color="primary" variant="text" prepend-icon="mdi-arrow-left">
          Back
        </v-btn>
      </div>

      <h1 class="text-center text-4xl font-bold mb-12">Choose Your Plan</h1>

      <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-8">
        <!-- Free Plan -->
        <v-card class="plan-card">
          <v-card-item>
            <v-card-title class="text-h4 mb-2">Free</v-card-title>
            <div class="text-h3 mb-1">$0</div>
            <div class="text-caption mb-4">forever</div>
            <v-divider class="mb-4"></v-divider>
            <v-list>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Up to 6 links</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Multiple search engines to choose from</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Quick launch links via shortcuts or search bar</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-item>
          <v-card-actions class="pa-4">
            <v-btn block color="primary" variant="flat">Get Started</v-btn>
          </v-card-actions>
        </v-card>

        <!-- Plus Plan -->
        <v-card class="plan-card">
          <v-card-item>
            <v-card-title class="text-h4 mb-2">Plus</v-card-title>
            <div class="text-h3 mb-1">$5</div>
            <div class="text-caption mb-4">per month</div>
            <v-divider class="mb-4"></v-divider>
            <v-list>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Unlimited links</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Search history</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Search autocomplete</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Automatically fetch links' titles, descriptions, and icons when adding</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-item>
          <v-card-actions class="pa-4">
            <a class="w-full" color="primary" :href=PlusPlanUrl><v-btn block variant="flat" color="primary">Upgrade to
                Plus</v-btn></a>
          </v-card-actions>
        </v-card>

        <!-- Pro Plan -->
        <v-card class="plan-card">
          <v-card-item>
            <v-card-title class="text-h4 mb-2">Pro</v-card-title>
            <div class="text-h3 mb-1">$10</div>
            <div class="text-caption mb-4">per month</div>
            <v-divider class="mb-4"></v-divider>
            <v-list>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>All the benefits of Plus</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Integrations (Jira, Confluence, GitHub, Linear)*</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-item>
          <v-card-actions class="pa-4">
            <v-btn block variant="flat" color="primary">Coming Soon</v-btn>
          </v-card-actions>
        </v-card>


        <!-- Team Plan -->
        <!-- <v-card class="plan-card">
          <v-card-item>
            <v-card-title class="text-h4 mb-2">Team</v-card-title>
            <div class="text-h3 mb-1">$8</div>
            <div class="text-caption mb-4">per user/month</div>
            <v-divider class="mb-4"></v-divider>
            <v-list>
              <v-list-item prepend-icon="mdi-check" class="text-white">
                <v-list-item-title>Up to 50 pinned items per page</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check" class="text-white">
                <v-list-item-title>Team-wide search history</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check" class="text-white">
                <v-list-item-title>Shared launch pages</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check" class="text-white">
                <v-list-item-title>Team analytics</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check" class="text-white">
                <v-list-item-title>Admin controls</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-item>
          <v-card-actions class="pa-4">
            <a class="w-full" color="primary" :href=TeamPlanUrl><v-btn block variant="flat" color="primary">Upgrade to Teams</v-btn></a>
          </v-card-actions>
        </v-card> -->

        <!-- Enterprise Plan -->
        <!-- <v-card class="plan-card">
          <v-card-item>
            <v-card-title class="text-h4 mb-2">Enterprise</v-card-title>
            <div class="text-h3 mb-1">Custom</div>
            <div class="text-caption mb-4">contact sales</div>
            <v-divider class="mb-4"></v-divider>
            <v-list>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Unlimited pinned items</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Custom integrations</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Advanced security features</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Dedicated support</v-list-item-title>
              </v-list-item>
              <v-list-item prepend-icon="mdi-check">
                <v-list-item-title>Custom deployment options</v-list-item-title>
              </v-list-item>
            </v-list>
          </v-card-item>
          <v-card-actions class="pa-4">
            <v-btn block color="primary" variant="flat">Contact Sales</v-btn>
          </v-card-actions>
        </v-card> -->
      </div>
      <div class="text-center mt-8 mb-8">
        <a href="https://docs.betternewtab.com" target="_blank" class="text-blue-400 hover:text-blue-300">
          <v-icon icon="mdi-book-open-page-variant" class="mr-1"></v-icon>
          *Visit Docs for more info
        </a>
      </div>
      <div class="text-center mt-8 p-4 border rounded-lg">
        <p class="text-white">Thanks for considering upgrading. Paid plans help support infrastructure cost to run BNT.
        </p>
        <p class="text-white">Reach out to <a href="mailto:evan.robertson77@gmail.com" class="underline">Evan
            Robertson</a> if you have any questions.</p>
        <div class="flex justify-center gap-4 mt-4">
          <a href="https://www.buymeacoffee.com/evanrobertson" target="_blank" class="text-white hover:text-gray-300">
            <v-icon icon="mdi-coffee" class="mr-1"></v-icon>
            Buy me a coffee
          </a>
          <a href="https://x.com/evantypesthings" target="_blank" class="text-white hover:text-gray-300">
            Follow on X
          </a>
          <a href="https://github.com/lostrhapsody" target="_blank" class="text-white hover:text-gray-300">
            <v-icon icon="mdi-github" class="mr-1"></v-icon>
            GitHub
          </a>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
  import { ref } from "vue";
  import { useRouter } from "vue-router";

  const router = useRouter();
  const selectedPlan = ref({
    id: "",
    name: "",
    price: 0,
  });
  const PlusPlanUrl = import.meta.env.VITE_PLUS_PLAN_URL;
  const ProPlanUrl = import.meta.env.VITE_PRO_PLAN_URL;
  const TeamPlanUrl = import.meta.env.VITE_TEAM_PLAN_URL;

  const handleBack = () => {
    router.go(-1);
  };
</script>

<style scoped>
  .plan-card {
    height: 100%;
    display: flex;
    flex-direction: column;
    transition: transform 0.2s;
    justify-content: space-between;
  }

  .plan-card:hover {
    transform: translateY(-5px);
  }

  div {
    white-space: normal !important;
    overflow: visible !important;
    text-overflow: clip !important;
  }
</style>