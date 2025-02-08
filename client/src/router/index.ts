import type { Subscription, SubscriptionResponse } from "@/types/Subscription";
import { Clerk } from "@clerk/clerk-js";
// src/router/index.ts
import { createRouter, createWebHistory } from "vue-router";
import { useUserStore } from "../stores/user";

const router = createRouter({
  history: createWebHistory(),
  routes: [
    {
      path: "/",
      name: "home",
      component: () => import("../views/Home.vue"),
    },
    {
      path: "/plans",
      name: "plans",
      component: () => import("../views/Plans.vue"),
    },
    {
      path: "/confirm",
      name: "confirm",
      component: () => import("../views/Confirm.vue"),
    },
    {
      path: "/settings",
      name: "settings",
      component: () => import("../views/Settings.vue"),
      beforeEnter: async (to, from, next) => {
        const userStore = useUserStore();

        // if not logged in already in userStore (user probably refreshed page)
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

            gotUser = await userStore.fetchUserData({
              id: clerk.user.id,
              firstName: clerk.user.firstName || "",
              lastName: clerk.user.lastName || "",
              email: clerk.user.emailAddresses[0].emailAddress,
            });

            if (!gotUser) {
              throw new Error("Failed to fetch user data");
            }

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
