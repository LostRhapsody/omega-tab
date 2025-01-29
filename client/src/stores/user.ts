import type { Subscription } from "@/types/Subscription";
import { defineStore } from "pinia";
import type { Tables } from "../types/Database";

interface UserState {
	userId: string | null;
	firstName: string | null;
	lastName: string | null;
	email: string | null;
	userPlan: Subscription | null;
}

export const useUserStore = defineStore("user", {
	state: (): UserState => ({
		userId: null,
		firstName: null,
		lastName: null,
		email: null,
		userPlan: null,
	}),

	actions: {
		setUserId(userId: string) {
			this.userId = userId;
		},

		setFirstName(firstname: string) {
			this.firstName = firstname;
		},

		setLastName(lastName: string) {
			this.lastName = lastName;
		},

		setEmail(email: string) {
			this.email = email;
		},

		setPlan(plan: Subscription) {
			this.userPlan = plan;
		},
	},
});
