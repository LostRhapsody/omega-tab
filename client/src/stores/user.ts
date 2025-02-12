import { API } from "@/constants/api";
import type { Subscription, SubscriptionResponse } from "@/types/Subscription";
import type { ClerkUser, User, UserState } from "@/types/User";
import { CacheKeys, cache } from "@/utils/cache";
import { defineStore } from "pinia";

export const useUserStore = defineStore("user", {
  state: (): UserState => ({
    userId: null,
    firstName: null,
    lastName: null,
    email: null,
    userPlan: null,
    isLoading: false,
    error: null,
  }),

  actions: {
    /**
     * Fetches user data from the API using the Clerk user information and updates the store state.
     * This includes the user record, subscription record, and plan record.
     * @param clerk_user - The Clerk user object containing authentication details.
     * @returns true if the user data was successfully fetched, false otherwise.
     * @throws Error if the user data could not be fetched.
     */
    async fetchUserData(clerk_user: ClerkUser) {
      // Try to load from cache first
      const cachedData = cache.get<UserState>(CacheKeys.USER);
      if (cachedData) {
        Object.assign(this.$state, cachedData);
      }

      let user: User;
      let subscription: SubscriptionResponse;
      this.isLoading = true;
      try {
        // first fetch user from DB / confirm user exists
        const response1 = await fetch(API.GET_USER(clerk_user.id));
        /*
          if 200, set user data
          if 404, create user
          else, throw error
        */
        switch (response1.status) {
          case 200: {
            user = await response1.json();
            break;
          }
          case 404: {
            /*
              if 404 create user
              if can't create user, throw error
            */
            try {
              const response2 = await fetch(API.CREATE_USER, {
                method: "POST",
                body: JSON.stringify({
                  user_id: clerk_user.id,
                  email: clerk_user.email,
                }),
              });
              if (!response2.ok) {
                // and throwing an error
                throw new Error(
                  `Failed to create user, status: ${response2.status}`,
                );
              }
              user = await response2.json();
              break;
            } catch (error) {
              this.error = error as string;
              this.isLoading = false;
              return false;
            }
          } // this is part of the switch block just an fyi, if triggered will go to catch below
          default:
            throw new Error(
              `Failed to fetch user data, unexpected status: ${response1.status}`,
            );
        }
      } catch (error) {
        this.error = error as string;
        this.isLoading = false;
        return false;
      }

      this.setEmail(user.email);
      this.setUserId(user.id);
      this.setFirstName(clerk_user.firstName);
      this.setLastName(clerk_user.lastName);

      try {
        // second fetch subscription information
        const response3 = await fetch(
          API.CONFIRM_SUBSCRIPTION(user.email, user.id),
        );
        /*
          if 200, set subscription data
          if 404, throw user not found error (what)
          else, throw error
        */
        switch (response3.status) {
          case 200: {
            subscription = await response3.json();
            break;
          }
          case 404: {
            throw new Error("User not found!");
          }
          default: {
            throw new Error(
              `Failed to fetch subscription data, unexpected status: ${response3.status}`,
            );
          }
        }
      } catch (error) {
        this.error = error as string;
        this.isLoading = false;
        return false;
      }

      try {
        // third, load the user's plan data
        const response4 = await fetch(API.GET_USER_PLAN(subscription.plan_id));
        /*
          if 200, set user plan data
          if 404, throw plan not found error
          else, throw error
        */
        switch (response4.status) {
          case 200: {
            this.setPlan(await response4.json());
            break;
          }
          case 404: {
            throw new Error("Plan not found! Contact support.");
          }
          default: {
            throw new Error("Failed to fetch subscription data");
          }
        }
      } catch (error) {
        this.error = error as string;
        this.isLoading = false;
        return false;
      }

      // Update cache after successful fetch
      cache.set(CacheKeys.USER, this.$state);
      this.isLoading = false;
      return true;
    },

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
