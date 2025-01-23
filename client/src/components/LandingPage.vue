<template>
  <v-app>
    <!-- Hero Section -->
    <v-container fluid class="hero-section bg-black pa-12">
      <v-row align="center" justify="center" class="text-center">
        <v-col cols="12" md="8">
          <h1 class="text-h2 font-weight-bold mb-4 text-white">
            Your Team's Universal Starting Point
          </h1>
          <p class="text-h5 mb-8 text-white">
            Transform every new tab into your team's command center. Launch tools, find documents, and stay updated - all in one place.
          </p>
          <v-btn
            size="x-large"
            color="secondary"
            class="me-4"
            variant="flat"
          >
            Start Free Trial
          </v-btn>
          <v-btn
            size="x-large"
            color="white"
            variant="outlined"
          >
            Watch Demo
          </v-btn>
        </v-col>
      </v-row>
    </v-container>

    <!-- Features Grid -->
    <v-container class="py-12">
      <v-row>
        <v-col cols="12" class="text-center mb-8">
          <h2 class="text-h3 font-weight-bold">Everything at Your Fingertips</h2>
          <p class="text-h6 mt-4">No more hunting through bookmarks or asking "Where was that document again?"</p>
        </v-col>
      </v-row>

      <v-row>
        <v-col cols="12" md="4">
          <v-card class="h-100">
            <v-card-item>
              <template v-slot:prepend>
                <v-icon
                  color="primary"
                  size="x-large"
                  icon="mdi-rocket-launch"
                ></v-icon>
              </template>
              <v-card-title class="text-h5">Quick Launch</v-card-title>
              <v-card-text>
                Access your tools and documents instantly with keyboard shortcuts or quick search. Your most-used resources, just a keystroke away.
              </v-card-text>
            </v-card-item>
          </v-card>
        </v-col>

        <v-col cols="12" md="4">
          <v-card class="h-100">
            <v-card-item>
              <template v-slot:prepend>
                <v-icon
                  color="primary"
                  size="x-large"
                  icon="mdi-account-group"
                ></v-icon>
              </template>
              <v-card-title class="text-h5">Team Sync</v-card-title>
              <v-card-text>
                Share important links and documents with your team. Keep everyone on the same page with shared launch pages and team-wide search history.
              </v-card-text>
            </v-card-item>
          </v-card>
        </v-col>

        <v-col cols="12" md="4">
          <v-card class="h-100">
            <v-card-item>
              <template v-slot:prepend>
                <v-icon
                  color="primary"
                  size="x-large"
                  icon="mdi-cog"
                ></v-icon>
              </template>
              <v-card-title class="text-h5">Custom Workflow</v-card-title>
              <v-card-text>
                Choose your search engine, customize your layout, and organize your tools exactly how you want them. Your workflow, your way.
              </v-card-text>
            </v-card-item>
          </v-card>
        </v-col>
      </v-row>
    </v-container>

    <!-- Integration Section -->
    <v-container fluid class="bg-grey-lighten-4 py-12">
      <v-row align="center" justify="center">
        <v-col cols="12" md="6" class="text-center">
          <h2 class="text-h3 font-weight-bold mb-6">Powerful Integrations</h2>
          <p class="text-h6 mb-8">
            Connect with your favorite tools like Jira, Google Workspace, and more. Get instant access to tasks, documents, and updates.
          </p>
          <v-row justify="center" class="mt-4">
            <v-col cols="auto">
              <v-img
                src="/images/jira-logo.png"
                alt="Jira"
                width="100"
                class="mx-4"
              ></v-img>
            </v-col>
            <!-- Add more integration logos here -->
          </v-row>
        </v-col>
      </v-row>
    </v-container>

    <!-- Pricing Section -->
    <v-container class="py-12">
      <v-row>
        <v-col cols="12" class="text-center mb-8">
          <h2 class="text-h3 font-weight-bold">Simple, Transparent Pricing</h2>
          <p class="text-h6 mt-4">Choose the plan that works best for your team</p>
        </v-col>
      </v-row>

      <v-row justify="center">
        <v-col cols="12" md="4">
          <v-card class="h-100" variant="outlined">
            <v-card-item>
              <v-card-title class="text-h4">Free</v-card-title>
              <p class="text-h3 mt-4">$0</p>
              <p class="text-caption">forever</p>
            </v-card-item>
            <v-card-text>
              <v-list>
                <v-list-item prepend-icon="mdi-check">
                  <v-list-item-title>Up to 6 pinned items</v-list-item-title>
                </v-list-item>
                <v-list-item prepend-icon="mdi-check">
                  <v-list-item-title>Basic search functionality</v-list-item-title>
                </v-list-item>
                <v-list-item prepend-icon="mdi-check">
                  <v-list-item-title>Single personal launch page</v-list-item-title>
                </v-list-item>
              </v-list>
            </v-card-text>
            <v-card-actions class="justify-center pa-4">
              <v-btn
                color="primary"
                variant="flat"
                block
              >
                Get Started
              </v-btn>
            </v-card-actions>
          </v-card>
        </v-col>

        <v-col cols="12" md="4">
          <v-card class="h-100" color="primary">
            <v-card-item>
              <v-card-title class="text-h4 text-white">Pro</v-card-title>
              <p class="text-h3 mt-4 text-white">$8</p>
              <p class="text-caption text-white">per month</p>
            </v-card-item>
            <v-card-text>
              <v-list bg-color="primary">
                <v-list-item prepend-icon="mdi-check" class="text-white">
                  <v-list-item-title>Up to 20 pinned items</v-list-item-title>
                </v-list-item>
                <v-list-item prepend-icon="mdi-check" class="text-white">
                  <v-list-item-title>Advanced search with history</v-list-item-title>
                </v-list-item>
                <v-list-item prepend-icon="mdi-check" class="text-white">
                  <v-list-item-title>Multiple launch pages</v-list-item-title>
                </v-list-item>
              </v-list>
            </v-card-text>
            <v-card-actions class="justify-center pa-4">
              <v-btn
                color="white"
                variant="flat"
                block
              >
                Start Free Trial
              </v-btn>
            </v-card-actions>
          </v-card>
        </v-col>
      </v-row>
    </v-container>

    <!-- CTA Section -->
    <v-container fluid class="bg-primary py-12">
      <v-row align="center" justify="center" class="text-center">
        <v-col cols="12" md="8">
          <h2 class="text-h3 font-weight-bold mb-4 text-white">
            Ready to Transform Your Workflow?
          </h2>
          <p class="text-h6 mb-8 text-white">
            Join thousands of teams who've made BNT their starting point for productivity.
          </p>
          <v-btn
            size="x-large"
            color="secondary"
            variant="flat"
          >
            Start Free Trial
          </v-btn>
        </v-col>
      </v-row>
    </v-container>
  </v-app>
</template>

<style scoped>
.hero-section {
  min-height: 80vh;
  display: flex;
  align-items: center;
}

.v-card {
  transition: transform 0.2s;
}

.v-card:hover {
  transform: translateY(-5px);
}
</style>