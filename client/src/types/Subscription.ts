import type { Features } from "./Features";

export type Subscription = {
	id: string;
	name: string;
	max_pins: number;
	features: Features;
	created_at: string | null;
	stripe_id: string | null;
};

export type SubscriptionResponse = {
	plan_id: string;
	current_period_end: number;
};
