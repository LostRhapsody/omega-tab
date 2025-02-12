// use axum::extract::Json;
use stripe::{Client, Customer, ListCustomers};
// use serde::{Deserialize, Serialize};
use anyhow::Result;

pub struct StripeClient {}

impl StripeClient {
    pub async fn get_customer(email: &str) -> Option<Customer> {
        let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");

        let client = Client::new(secret_key);

        let mut list_customers = ListCustomers::new();
        list_customers.email = Some(email);

        match Customer::list(&client, &list_customers).await {
            Ok(customers) => customers.data.into_iter().next(),
            Err(err) => {
                eprintln!("Error retrieving customer: {:?}", err);
                None
            }
        }
    }

    pub async fn get_subscription(customer: &Customer) -> Option<stripe::Subscription> {
        let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
        let client = Client::new(secret_key);

        let mut list_subscriptions = stripe::ListSubscriptions::new();
        list_subscriptions.customer = Some(customer.id.clone());

        match stripe::Subscription::list(&client, &list_subscriptions).await {
            Ok(subscriptions) => subscriptions.data.into_iter().next(),
            Err(err) => {
                eprintln!("Error retrieving subscription: {:?}", err);
                None
            }
        }
    }

    pub async fn cancel_subscription(
        email: String,
        reason: Option<String>,
        feedback: Option<stripe::CancellationDetailsFeedback>
    ) -> Option<stripe::Subscription> {
        let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
        let client = Client::new(secret_key);

        // First get the customer
        let customer = Self::get_customer(&email).await?;

        // Then get their subscription
        let subscription = Self::get_subscription(&customer).await?;

        let cancel_options = if reason.is_some() || feedback.is_some() {
            let cancellation_details = stripe::CancellationDetails {
                comment: reason,
                feedback,
                reason: None,
            };
            stripe::CancelSubscription {
                cancellation_details: Some(cancellation_details),
                invoice_now: None,
                prorate: None,
            }
        } else {
            stripe::CancelSubscription::new()
        };

        // Cancel the subscription
        match stripe::Subscription::cancel(
            &client,
            &subscription.id,
            cancel_options,
        )
        .await
        {
            Ok(sub) => {
                println!("Subscription canceled: {:?}", sub);
                // Fetch the updated subscription to get the current status
                // match stripe::Subscription::retrieve(&client, &subscription.id, &[]).await {
                //     Ok(updated_subscription) => Some(updated_subscription),
                //     Err(err) => {
                //         eprintln!("Error retrieving updated subscription: {:?}", err);
                //         None
                //     }
                // }
                return Some(sub);
            },
            Err(err) => {
                eprintln!("Error canceling subscription: {:?}", err);
                None
            }
        }
    }

    pub async fn get_customer_email(customer_id: &str) -> Result<Option<String>> {
        let secret_key = std::env::var("STRIPE_SECRET_KEY").expect("STRIPE_SECRET_KEY must be set");
        let client = Client::new(secret_key);
        let customer_id = match customer_id.parse::<stripe::CustomerId>() {
            Ok(id) => id,
            Err(err) => {
                println!("Error parsing customer ID: {:?}", err);
                return Err(err.into());
            }
        };

        match Customer::retrieve(&client, &customer_id, &[]).await {
            Ok(customer) => Ok(customer.email),
            Err(err) => {
                println!("Error retrieving customer email: {:?}", err);
                Err(err.into())
            }
        }
    }
}
