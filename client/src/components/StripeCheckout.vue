<template>
  <v-dialog v-model="showDialog" max-width="500">
    <v-card>
      <v-card-title>Confirm Subscription</v-card-title>
      <v-card-text>
        <p>You are about to subscribe to the {{ planName }} plan.</p>
        <p v-if="price" class="text-h6">{{ formatPrice(price) }}<span v-if="planName === 'Team'"> per user</span> a month</p>
        <div v-if="loading" class="d-flex justify-center">
          <v-progress-circular indeterminate></v-progress-circular>
        </div>
      </v-card-text>
      <v-card-actions>
        <v-spacer></v-spacer>
        <v-btn color="grey" variant="text" @click="closeDialog">Cancel</v-btn>
        <v-btn
          color="primary"
          :loading="loading"
          @click="handleCheckout"
        >
          Proceed to Checkout
        </v-btn>
      </v-card-actions>
    </v-card>
  </v-dialog>
</template>

<script setup lang="ts">
import { loadStripe } from "@stripe/stripe-js";
import { computed, defineEmits, defineProps, ref } from "vue";

const props = defineProps<{
	modelValue: boolean;
	planId: string;
	planName: string;
	price: number;
}>();

const emit = defineEmits(["update:modelValue"]);

const stripePromise = loadStripe(import.meta.env.VITE_STRIPE_PUBLISHABLE_KEY);
const loading = ref(false);
const showDialog = computed({
	get: () => props.modelValue,
	set: (value) => emit("update:modelValue", value),
});

const closeDialog = () => {
	showDialog.value = false;
	emit("update:modelValue", false);
};

const formatPrice = (price: number) => {
	return new Intl.NumberFormat("en-US", {
		style: "currency",
		currency: "USD",
	}).format(price);
};

const handleCheckout = async () => {
	loading.value = true;

	try {
		const stripe = await stripePromise;
		if (!stripe) throw new Error("Stripe failed to load");

		// Create checkout session on your backend
		const response = await fetch(
			"http://localhost:3000/api/create-checkout-session",
			{
				method: "POST",
				headers: {
					"Content-Type": "application/json",
				},
				body: JSON.stringify({
					plan_id: props.planId,
				}),
			},
		);

		const session = await response.json();

		// Redirect to Stripe Checkout
		const { error } = await stripe.redirectToCheckout({
			sessionId: session.id,
		});

		if (error) {
			console.error("Error:", error);
			// Handle the error and show user feedback
		}
	} catch (e) {
		console.error("Checkout error:", e);
		// Handle error and show user feedback
	} finally {
		loading.value = false;
	}
};
</script>