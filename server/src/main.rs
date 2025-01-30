mod stripe_client;
mod supabase;

use axum::{
    extract::{Json, Path},
    http::StatusCode,
    routing::{delete, get, post},
    Router,
};
use chrono::{TimeZone, Utc};
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

#[derive(Serialize)]
pub struct CustomerResponse {
    session_id: String,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    user_id: String,
    email: String,
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    message: String,
}

#[derive(Deserialize)]
pub struct CreateLinkRequest {
    url: String,
    description: Option<String>,
    title: String,
    next_order_index: i32,
    owner_type: String,
    owner_id: String,
    column_type: String,
}

#[derive(Serialize)]
pub struct CreateLinkResponse {
    link: supabase::Link,
    message: String,
}

#[derive(Deserialize)]
pub struct UpdateLinkRequest {
    id: String,
    url: Option<String>,
    description: Option<String>,
    title: Option<String>,
    icon: Option<String>,
    column_type: Option<String>,
}

#[derive(Serialize)]
pub struct UpdateLinkResponse {
    message: String,
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // confirm subscription
        .route("/confirm/{user_email}/{user_id}", get( move | path | confirm_handler(path)))
        // create and update links
        .route("/link", post(create_link).put(update_link))
        // read links
        .route(
            "/user/{user_id}/links",
            get(move |path| links_handler(path)),
        )
        // delete link
        .route("/link/{link_id}", delete(move |path| delete_link(path)))
        // get plan
        .route("/plan/{plan_id}", get(move |path| plan_handler(path)))
        // create user
        .route("/create_user", post(create_user_handler))
        // get user
        .route("/user/{user_id}", get(move |path| get_user_handler(path)))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn create_user_handler(
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<supabase::User>, StatusCode> {
    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = supabase::User {
        id: payload.user_id,
        email: payload.email,
        created_at: Utc::now().to_rfc3339(),
    };

    match supabase.create_user(user.clone()).await {
        Ok(_) => Ok(Json(user)),
        Err(e) => match e.to_string().as_str() {
            "409" => Err(StatusCode::BAD_REQUEST),
            _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
        },
    }
}

/// Handles the confirmation of a user's subscription status.
///
/// This function performs the following steps:
/// 1. Retrieves the Stripe customer associated with the provided email.
/// 2. Retrieves the Stripe subscription associated with the customer.
/// 3. Checks if the subscription is active.
/// 4. Retrieves the corresponding Supabase plan based on the Stripe product ID.
/// 5. Verifies and updates the user's subscription record in Supabase.
/// 6. Verifies and creates the user's membership record in Supabase if it doesn't exist.
///
/// # Arguments
///
/// * `Path((user_email, user_id))` - A tuple containing the user's email and user ID.
/// * sent to the server as  /confirm/{user_email}/{user_id}
///
/// # Returns
///
/// * `Result<Json<SubscriptionResponse>, StatusCode>` - A JSON response containing the subscription details or an error status code.
/// * If no subscription is found, the function returns a free plan ID and a current period end of 0.
///
/// # Errors
///
/// This function returns an appropriate `StatusCode` in case of errors:
/// * `StatusCode::INTERNAL_SERVER_ERROR` - If there is an internal server error.
/// * `StatusCode::NOT_FOUND` - If the user is not found in Supabase.
/// * `StatusCode::BAD_REQUEST` - If there is a conflict with the existing subscription.
///
/// # Example
///
/// ```rust
/// let response = confirm_handler(Path(("user@example.com".to_string(), "user_id".to_string()))).await;
/// match response {
///     Ok(json) => println!("Subscription confirmed: {:?}", json),
///     Err(status) => println!("Error confirming subscription: {:?}", status),
/// }
/// ```
async fn confirm_handler(
    Path((user_email, user_id)): Path<(String, String)>,
) -> Result<Json<SubscriptionResponse>, StatusCode> {

    println!("Confirming email: {}", user_email);

    let free_plan_id = std::env::var("FREE_PLAN_ID").expect("FREE_PLAN_ID must be set");

    // Initialize Supabase client
    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("Initialized Supabase client");
    println!("Getting customer from Stripe");
    // Get Stripe customer - if one has not been created, they have not subscribed
    let customer = match StripeClient::get_customer(&user_email).await {
        Some(customer) => customer,
        None => {
            return Ok(Json(SubscriptionResponse {
                plan_id: free_plan_id,
                current_period_end: 0,
            }))
        }
    };

    println!("Got customer from Stripe, getting subscription");
    // Get Stripe subscription - if one is not returned, they have not subscribed (or subscription is inactive)
    let subscription = match StripeClient::get_subscription(&customer).await {
        Some(sub) => sub,
        None => {
            return Ok(Json(SubscriptionResponse {
                plan_id: free_plan_id,
                current_period_end: 0,
            }))
        }
    };

    println!("Got subscription from Stripe");
    // Double check if subscription is active
    if !subscription.status.eq(&stripe::SubscriptionStatus::Active) {
        return Ok(Json(SubscriptionResponse {
            plan_id: free_plan_id,
            current_period_end: 0,
        }));
    }

    println!("Subscription is active");
    // Get the subscription- we need the product id for later
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

    let user = supabase
        .get_user(&user_id)
        .await
        .map_err(|e| match e.to_string().as_str() {
            "404" => {
                println!(
                    "User not found, can't create new user for: {} because signup was supposed to be done through Clerk",
                    user_email
                );
                StatusCode::NOT_FOUND
            }
            _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    // Get corresponding Supabase plan
    let supabase_plan = supabase
        .get_plan_by_stripe_id(&product_id.to_string())
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    println!("Supabase plan: {:?}", supabase_plan);

    // Verify subscription record
    let sub_result = supabase.get_user_subscription(&user.id).await;

    // verify we got the subcription
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
            println!("No subscription found, creating subscription");
            // Create new subscription
            let new_sub = supabase::Subscription {
                id: uuid::Uuid::new_v4().to_string(),
                entity_id: user.id.clone(),
                entity_type: "user".to_string(),
                plan_id: supabase_plan.clone().id,
                status: "active".to_string(),
                stripe_subscription_id: subscription.id.to_string(),
                current_period_end: Utc
                    .timestamp_opt(subscription.current_period_end, 0)
                    .single()
                    .expect("Invalid timestamp")
                    .to_rfc3339(),
                created_at: Utc::now().to_rfc3339(),
            };

            println!("new_sub: {:?}", new_sub);
            if let Err(e) = supabase
                .create_subscription(
                    &new_sub.entity_id,
                    &new_sub.entity_type,
                    &new_sub.plan_id,
                    &new_sub.status,
                    &new_sub.stripe_subscription_id,
                    &new_sub.current_period_end,
                )
                .await
            {
                println!("Error creating subscription: {:?}", e);
                return Err(StatusCode::INTERNAL_SERVER_ERROR);
            }
        }
    }

    // Verify/create user membership
    let membership_result = supabase.get_user_memberships(&user.id).await;

    match membership_result {
        Ok(memberships) => {
            println!("Got membership vector from Supabase good");
            if memberships.is_empty() {
                println!("Vector is empty, creating membership record");
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
        Err(_) => {
            println!("Error getting membership vector from Supabase");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    println!("Returning response");
    // Return the subscription
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

async fn create_link(
    Json(payload): Json<CreateLinkRequest>,
) -> Result<(StatusCode, Json<supabase::Link>), StatusCode> {
    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let link = supabase::Link {
        id: uuid::Uuid::new_v4().to_string(),
        url: payload.url,
        description: payload.description,
        created_at: Utc::now().to_rfc3339(),
        title: payload.title,
        icon: None,
        order_index: payload.next_order_index,
        owner_type: payload.owner_type,
        owner_id: payload.owner_id,
        column_type: payload.column_type,
    };

    if let Err(e) = supabase.create_link(link.clone()).await {
        println!("Error creating link: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok((StatusCode::CREATED, Json(link)))
}

async fn update_link(
    Json(payload): Json<UpdateLinkRequest>,
) -> Result<Json<UpdateLinkResponse>, StatusCode> {
    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut updates = HashMap::new();
    if let Some(url) = payload.url {
        updates.insert("url".to_string(), json!(url));
    }
    if let Some(description) = payload.description {
        updates.insert("description".to_string(), json!(description));
    }
    if let Some(title) = payload.title {
        updates.insert("title".to_string(), json!(title));
    }
    if let Some(icon) = payload.icon {
        updates.insert("icon".to_string(), json!(icon));
    }
    if let Some(icon) = payload.column_type {
        updates.insert("column_type".to_string(), json!(icon));
    }

    if let Err(e) = supabase.update_link(&payload.id, updates).await {
        println!("Error updating link: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(Json(UpdateLinkResponse {
        message: "Link updated successfully".to_string(),
    }))
}

async fn delete_link(Path(link_id): Path<String>) -> Result<StatusCode, StatusCode> {
    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if let Err(e) = supabase.delete_link(&link_id).await {
        println!("Error deleting link: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::NO_CONTENT)
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
        .map_err(|e| match e.to_string().as_str() {
            "404" => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(plan))
}

async fn get_user_handler(Path(user_id): Path<String>) -> Result<Json<supabase::User>, StatusCode> {
    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = supabase
        .get_user(&user_id)
        .await
        .map_err(|e| match e.to_string().as_str() {
            "404" => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(user))
}
