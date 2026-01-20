import { createRouter, createWebHistory } from "vue-router";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("../views/LandingPage.vue"),
    },
    {
      path: "/privacy-policy",
      name: "privacyPolicy",
      component: () => import("../views/PrivacyPolicy.vue"),
    },
    {
      path: "/terms-of-service",
      name: "termsOfService",
      component: () => import("../views/TermsOfService.vue"),
    },
  ],
});

export default router;
