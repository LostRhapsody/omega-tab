import type { Subscription, SubscriptionResponse } from "@/types/Subscription";
import { Clerk } from "@clerk/clerk-js";
// src/router/index.ts
import { createRouter, createWebHistory } from "vue-router";
import { useApi } from "../composables/useApi";
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
			path: "/test",
			name: "test",
			component: () => import("../views/Test.vue"),
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
				const { api } = useApi();
				if (!userStore.userId) {
					const clerk = new Clerk(import.meta.env.VITE_CLERK_PUBLISHABLE_KEY);
					await clerk.load();

					if (clerk.user) {
						if (!clerk.user?.emailAddresses[0]) {
							throw new Error("No user email found");
						}

						const email = clerk.user.emailAddresses[0].emailAddress;

						// Set user in store
						userStore.setUserId(clerk.user.id);
						userStore.setFirstName(clerk.user.firstName || "");
						userStore.setLastName(clerk.user.lastName || "");
						userStore.setEmail(email);
						const subscriptionData = (await api("/confirm", {
							method: "POST",
							body: JSON.stringify({
								email: email,
							}),
						})) as SubscriptionResponse;
						const userPlan: Subscription = await api(
							`/plan/${subscriptionData.plan_id}`,
						);
						userStore.setPlan(userPlan);
						next();
					} else {
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
