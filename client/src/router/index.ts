import type { Subscription, SubscriptionResponse } from "@/types/Subscription";
import { CacheKeys, cache } from "@/utils/cache";
import { Clerk } from "@clerk/clerk-js";
// src/router/index.ts
import { createRouter, createWebHistory } from "vue-router";
import { useUserSettingsStore } from "../stores/settings";
import { useUserStore } from "../stores/user";

const isStaging = import.meta.env.VITE_STAGING === "true";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("../views/Home.vue"),
      beforeEnter: (to, from, next) => {
        // Check if we're in staging mode and not logged in
        if (isStaging && !cache.get(CacheKeys.STAGING_LOGGED_IN)) {
          next("/staging-login");
        } else {
          next();
        }
      },
    },
    {
      path: "/staging-login",
      name: "stagingLogin",
      component: () => import("../views/staging_login.vue"),
    },
    {
      path: "/plans",
      name: "plans",
      component: () => import("../views/Plans.vue"),
      beforeEnter: (to, from, next) => {
        // Check if we're in staging mode and not logged in
        if (isStaging && !cache.get(CacheKeys.STAGING_LOGGED_IN)) {
          next("/staging-login");
        } else {
          next();
        }
      },
    },
    {
      path: "/confirm",
      name: "confirm",
      component: () => import("../views/Confirm.vue"),
      beforeEnter: (to, from, next) => {
        // Check if we're in staging mode and not logged in
        if (isStaging && !cache.get(CacheKeys.STAGING_LOGGED_IN)) {
          next("/staging-login");
        } else {
          next();
        }
      },
    },
    {
      path: "/contact",
      name: "contact",
      component: () => import("../views/Contact.vue"),
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
    {
      path: "/settings",
      name: "settings",
      component: () => import("../views/Settings.vue"),
      beforeEnter: async (to, from, next) => {
        // Check if we're in staging mode and not logged in
        if (isStaging && !cache.get(CacheKeys.STAGING_LOGGED_IN)) {
          next("/staging-login");
          return;
        }

        const userStore = useUserStore();
        const userSettingsStore = useUserSettingsStore();

        // if not logged in already in userStore (user probably refreshed page)
        // note: don't refreh from cache here, speed is not important, accuracy is.
        if (!userStore.userId) {
          try {
            const clerk = new Clerk(import.meta.env.VITE_CLERK_PUBLISHABLE_KEY);
            await clerk.load();

            if (!clerk.user) {
              throw new Error("User not logged in");
            }

            if (!clerk.user.emailAddresses[0]) {
              throw new Error("No user email found");
            }

            let gotUser = false;

            // Fetch user data asynchronously without blocking the UI
            userStore
              .fetchUserData({
                id: clerk.user.id,
                firstName: clerk.user.firstName || "",
                lastName: clerk.user.lastName || "",
                email: clerk.user.emailAddresses[0].emailAddress,
              })
              .then((success) => {
                if (!success) {
                  console.error("Failed to fetch user data");
                }
              })
              .catch((err) => {
                console.error("Error fetching user data:", err);
              });

            // Proceed without waiting for fetchUserData to complete
            gotUser = !!userStore.userId;

            if (!gotUser) {
              throw new Error("Failed to fetch user data");
            }

            // always fetch settings with User
            await userSettingsStore.fetchSettings();

            next();
          } catch (err) {
            console.error(err);
            next("/");
          }
        } else {
          next();
        }
      },
    },
  ],
});

export default router;
