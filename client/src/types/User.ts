import type { Link } from "./Link";
import type { Subscription, SubscriptionResponse } from "./Subscription";
import type { UserSettings } from "./UserSettings";

// this is the "Supabase" User type, i.e. exactly what is in the supabase table
export type User = {
  id: string;
  email: string;
  createdAt: string;
};

// this is the "Pinia" User type, i.e. what we want to store in the store
export type UserState = {
  userId: string | null;
  firstName: string | null;
  lastName: string | null;
  email: string | null;
  userPlan: Subscription | null;
  isLoading: boolean;
  error: string | null;
};

// this is the "Clerk" User type, i.e. the data we get from Clerk
export type ClerkUser = {
  id: string;
  email: string;
  firstName: string;
  lastName: string;
};

export type UserDataResponse = {
  user: User;
  subscription: SubscriptionResponse | null;
  plan: Subscription | null;
  settings: settings_blob;
  links: Link[];
};

export type settings_blob = {
  settings_blob: object;
};
