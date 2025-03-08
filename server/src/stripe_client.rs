// use axum::extract::Json;
use stripe::{Client, Customer, ListCustomers};
// use serde::{Deserialize, Serialize};
use anyhow::Result;

pub struct StripeClient {}

impl StripeClient {
    pub async fn get_customer(email: &str) -> Option<Customer> {
        tracing::info!("Fetching Stripe customer for email: {}", email);
        println!("Fetching Stripe customer for email: {}", email);
        let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");

        let client = Client::new(secret_key);

        let mut list_customers = ListCustomers::new();
        list_customers.email = Some(email);

        match Customer::list(&client, &list_customers).await {
            Ok(customers) => {
                let customer = customers.data.into_iter().next();
                if let Some(ref _c) = customer {
                    tracing::info!("Found Stripe customer for email: {}", email);
                    println!("Found Stripe customer for email: {}", email);
                } else {
                    tracing::info!("No Stripe customer found for email: {}", email);
                    println!("No Stripe customer found for email: {}", email);
                }
                customer
            },
            Err(err) => {
                tracing::error!("Error retrieving Stripe customer: {:?}", err);
                println!("Error retrieving Stripe customer: {:?}", err);
                None
            }
        }
    }

    pub async fn get_subscription(customer: &Customer) -> Option<stripe::Subscription> {
        tracing::info!("Fetching Stripe subscription for customer: {}", customer.id);
        let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
        let client = Client::new(secret_key);

        let mut list_subscriptions = stripe::ListSubscriptions::new();
        list_subscriptions.customer = Some(customer.id.clone());

        match stripe::Subscription::list(&client, &list_subscriptions).await {
            Ok(subscriptions) => {
                let subscription = subscriptions.data.into_iter().next();
                if subscription.is_some() {
                    tracing::info!("Found active subscription for customer: {}", customer.id);
                } else {
                    tracing::info!("No active subscription found for customer: {}", customer.id);
                }
                subscription
            },
            Err(err) => {
                tracing::error!("Error retrieving subscription: {:?}", err);
                None
            }
        }
    }

    pub async fn cancel_subscription(
        email: String,
        reason: Option<String>,
        feedback: Option<stripe::UpdateSubscriptionCancellationDetailsFeedback>
    ) -> Option<stripe::Subscription> {
        tracing::info!("Cancelling subscription for email: {}", email);

        let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
        let client = Client::new(secret_key);

        // First get the customer
        let customer = Self::get_customer(&email).await?;

        // Then get their subscription
        let subscription = Self::get_subscription(&customer).await?;

        tracing::info!("Processing cancellation for subscription: {}", subscription.id);

        let cancellation_details = if reason.is_some() || feedback.is_some() {
            Some(stripe::UpdateSubscriptionCancellationDetails {
                comment: reason,
                feedback,
            })
        } else {
            None
        };

        // Instead of immediately cancelling, set the cancel_at_period_end flag to true
        let cancel_params = stripe::UpdateSubscription {
            cancel_at_period_end: Some(true),
            cancellation_details: cancellation_details,
            ..Default::default()
        };

        match stripe::Subscription::update(
            &client,
            &subscription.id,
            cancel_params,
        )
        .await
        {
            Ok(sub) => {
                tracing::info!("Successfully cancelled subscription at period end: {}", sub.id);
                Some(sub)
            },
            Err(err) => {
                tracing::error!("Error canceling subscription: {:?}", err);
                None
            }
        }
    }

    pub async fn get_customer_email(customer_id: &str) -> Result<Option<String>> {
        tracing::info!("Fetching email for customer: {}", customer_id);
        let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
        let client = Client::new(secret_key);
        let customer_id = match customer_id.parse::<stripe::CustomerId>() {
            Ok(id) => id,
            Err(err) => {
                tracing::error!("Error parsing customer ID: {:?}", err);
                return Err(err.into());
            }
        };

        match Customer::retrieve(&client, &customer_id, &[]).await {
            Ok(customer) => {
                if let Some(ref email) = customer.email {
                    tracing::info!("Found email for customer {}: {}", customer_id, email);
                } else {
                    tracing::info!("No email found for customer: {}", customer_id);
                }
                Ok(customer.email)
            },
            Err(err) => {
                tracing::error!("Error retrieving customer email: {:?}", err);
                Err(err.into())
            }
        }
    }
}
