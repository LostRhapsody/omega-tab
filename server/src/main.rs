mod stripe_client;
mod supabase;

use axum::extract::Path;
use axum::{extract::Json, http::StatusCode};
use axum::{
    routing::{get, post},
    Router,
};
use chrono::Utc;
use dotenv::dotenv;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use stripe_client::StripeClient;
use supabase::Supabase;
use tower_http::cors::{Any, CorsLayer};

#[derive(Serialize)]
pub struct SubscriptionResponse {
    plan_id: String,
    current_period_end: i64,
}

#[derive(Deserialize)]
pub struct CustomerRequest {
    email: String,
}

#[derive(Serialize)]
pub struct CustomerResponse {
    session_id: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/hello", get(hello_handler))
        .route("/confirm", post(confirm_handler))
        .route(
            "/links/{user_id}", get({
                move |path| links_handler(path)
            }),
        )
        .route(
            "/plan/{plan_id}", get({
                move |path| plan_handler(path)
            }),
        )
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn hello_handler() -> &'static str {
    "hello!"
}

async fn confirm_handler(
    Json(payload): Json<CustomerRequest>,
) -> Result<Json<SubscriptionResponse>, StatusCode> {
    println!("Received request for email: {}", payload.email);
    // Initialize Supabase client
    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("Initialized Supabase client");
    println!("Getting customer from Stripe");
    // Get Stripe customer
    let customer = match StripeClient::get_customer(&payload.email).await {
        Some(customer) => customer,
        None => return Err(StatusCode::NOT_FOUND),
    };

    println!("Got customer from Stripe");
    // Get Stripe subscription
    let subscription = match StripeClient::get_subscription(&customer).await {
        Some(sub) => sub,
        None => return Err(StatusCode::NOT_FOUND),
    };

    println!("Got subscription from Stripe");
    // Check if subscription is active
    if !subscription.status.eq(&stripe::SubscriptionStatus::Active) {
        return Err(StatusCode::PAYMENT_REQUIRED);
    }

    println!("Subscription is active");
    // Get first subscription item
    let item = subscription
        .items
        .data
        .first()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let plan = item
        .plan
        .as_ref()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?;

    let product_id = plan
        .product
        .as_ref()
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)?
        .id();

    println!("Got product id: {}", product_id);
    // Step 1: Verify/create user record
    let user = match supabase.get_user_by_email(&payload.email).await {
        Ok(user) => user,
        Err(_) => {
            // Create new user if doesn't exist
            supabase
                .create_user(&payload.email)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        }
    };

    println!("Got user from Supabase");
    println!("User: {:?}", user);
    // Step 2: Get corresponding Supabase plan
    let supabase_plan = supabase
        .get_plan_by_stripe_id(&product_id.to_string())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("Got Supabase plan");
    println!("Supabase plan: {:?}", supabase_plan);
    // Step 3: Verify/create subscription record
    let sub_result = supabase.get_user_subscription(&user.id).await;
    println!("Got user subscription from Supabase");

    println!("sub_result: {:?}", sub_result);
    match sub_result {
        Ok(sub) => {
            println!("Got subscription from Supabase");
            // Update existing subscription if plan changed
            if sub.plan_id != supabase_plan.id {
                let mut updates = HashMap::new();
                updates.insert("plan_id".to_string(), json!(supabase_plan.id));
                updates.insert(
                    "current_period_end".to_string(),
                    json!(subscription.current_period_end),
                );
                updates.insert("stripe_subscription_id".to_string(), json!(subscription.id));

                println!("Updating subscription");
                println!("updates: {:?}", updates);
                supabase
                    .update_subscription(&sub.id, updates)
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
            }
        }
        Err(_) => {
            println!("Creating subscription");
            // Create new subscription
            let new_sub = supabase::Subscription {
                id: uuid::Uuid::new_v4().to_string(),
                entity_id: user.id.clone(),
                entity_type: "user".to_string(),
                plan_id: supabase_plan.clone().id,
                status: "active".to_string(),
                stripe_subscription_id: Some(subscription.id.to_string()),
                current_period_end: Some(subscription.current_period_end.to_string()),
                created_at: Some(Utc::now().to_rfc3339()),
            };

            println!("new_sub: {:?}", new_sub);
            if let Err(e) = supabase.create_subscription(new_sub).await {
                println!("Error creating subscription: {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    println!("Created subscription");
    // Step 4: Verify/create user membership
    let membership_result = supabase.get_user_memberships(&user.id).await;
    println!("Got user memberships from Supabase");
    println!("membership_result: {:?}", membership_result);
    match membership_result {
        Ok(memberships) => {
            println!("Got memberships from Supabase good");
            if memberships.is_empty() {
                println!("Creating membership");
                let membership = supabase::UserMembership {
                    user_id: user.id.clone(),
                    entity_id: user.id.clone(),
                    entity_type: "user".to_string(),
                    role: "owner".to_string(),
                    created_at: Utc::now().to_rfc3339(),
                };

                println!("new membership: {:?}", membership);
                if let Err(e) = supabase.add_member(membership).await {
                    println!("Error creating membership: {:?}", e);
                    return Err(StatusCode::INTERNAL_SERVER_ERROR);
                }
                println!("Membership created successfully");
            }
        }
        Err(_) => return Err(StatusCode::INTERNAL_SERVER_ERROR),
    }

    println!("Created membership");
    println!("Returning response");
    // Return successful response
    Ok(Json(SubscriptionResponse {
        plan_id: supabase_plan.id,
        current_period_end: subscription.current_period_end,
    }))
}

async fn links_handler(
    Path(user_id): Path<String>,
) -> Result<Json<Vec<supabase::Link>>, StatusCode> {

    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let links = supabase
        .get_links(&user_id, "user")
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(links))
}

async fn plan_handler(Path(plan_id): Path<String>) -> Result<Json<supabase::Plan>, StatusCode> {
    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let plan = supabase
        .get_plan(&plan_id)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(plan))
}