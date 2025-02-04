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
use reqwest::header::{HeaderMap, HeaderValue, USER_AGENT};

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
    title: Option<String>,
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

#[derive(Deserialize, Clone, Debug)]
pub struct Metadata {
    title: Option<String>,
    description: Option<String>,
    favicon: Option<String>,
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
        // cancel subscription
        .route("/cancel/{user_email}/{user_id}", get( move | path | cancel_handler(path)))
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
    // TODO - cleanup task, if the subscription EXISTS in Supbase but NOT stripe, then the customer probably
    // unsubscribed somehow. We should cancel the subscription in Supabase as well.
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

    let url = if !payload.url.starts_with("https://") {
        format!("https://{}", payload.url)
    } else {
        payload.url
    };

    let metadata = get_metadata(&url).await.map_err(|e| {
        println!("Error getting metadata: {:?}", e);
    }).unwrap_or_else(|_| Metadata {
        title: Some(url.clone()),
        description: None,
        favicon: None,
    });

    let title = if payload.title.as_deref() == Some("") {
        metadata.title.unwrap_or_else(|| "".to_string())
    } else {
        payload.title.unwrap_or_else(|| metadata.title.unwrap().clone())
    };

    let favicon = metadata.favicon.unwrap_or_else(|| "".to_string());

    let description = if payload.description.as_deref() == Some("") {
        metadata.description.unwrap_or_else(|| "".to_string())
    } else {
        payload.description.unwrap_or_else(|| metadata.description.unwrap().clone())
    };

    let link = supabase::Link {
        id: uuid::Uuid::new_v4().to_string(),
        url: url,
        description: Some(description),
        created_at: Utc::now().to_rfc3339(),
        title: title,
        icon: Some(favicon),
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
) -> Result<StatusCode, StatusCode> {
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

    Ok(StatusCode::OK)
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

/// Handles the cancellation of a user's subscription.
///
/// This function performs the following steps:
/// 1. Validates the user email and user ID inputs
/// 2. Verifies the user exists in Supabase
/// 3. Confirms the user has an active Stripe subscription
/// 4. Cancels the subscription in Stripe
///
/// # Arguments
///
/// * `Path((user_email, user_id))` - A tuple containing the user's email and user ID
/// * sent to the server as `/cancel/{user_email}/{user_id}`
///
/// # Returns
///
/// * `Result<StatusCode, StatusCode>` - Returns OK (200) if cancellation is successful
///
/// # Errors
///
/// This function returns an appropriate `StatusCode` in case of errors:
/// * `StatusCode::BAD_REQUEST` (400) - If email or user ID is empty
/// * `StatusCode::NOT_FOUND` (404) - If user doesn't exist in Supabase
/// * `StatusCode::UNAUTHORIZED` (401) - If user has no active subscription
/// * `StatusCode::INTERNAL_SERVER_ERROR` (500) - For any other errors
///
/// # Example
///
/// ```rust
/// let response = cancel_handler(Path(("user@example.com".to_string(), "user_id".to_string()))).await;
/// match response {
///     Ok(_) => println!("Subscription cancelled successfully"),
///     Err(status) => println!("Error cancelling subscription: {:?}", status),
/// }
/// ```
async fn cancel_handler(
    Path((user_id, user_email)): Path<(String, String)>,
) -> Result<StatusCode, StatusCode> {

    println!("Cancelling email: {}", user_email);
    println!("Cancelling user id: {}", user_id);

    // confirm the user email and user id are present and formatted well else throw a 400
    if user_email.is_empty() || user_id.is_empty() {
        println!("User email or user id is empty");
        return Err(StatusCode::BAD_REQUEST)
    }

    // first confirm if user exists, if not throw a 404
    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    ).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _user = supabase.get_user(&user_id).await.map_err(|e| match e.to_string().as_str() {
        "404" => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    println!("Got user: {:?}", _user);

    // then get the supabase subscription
    let supa_sub = supabase.get_user_subscription(&user_id).await.map_err(|e| match e.to_string().as_str() {
        "404" => StatusCode::NOT_FOUND,
        _ => StatusCode::INTERNAL_SERVER_ERROR,
    })?;

    println!("Got supabase subscription: {:?}", supa_sub);

    // then confirm the user's subscription is active, if not throw a 401
    let customer = match StripeClient::get_customer(&user_email).await {
        Some(customer) => customer,
        None => {
            return Err(StatusCode::UNAUTHORIZED)
        }
    };

    println!("Got customer: {:?}", customer);

    let subscription = match StripeClient::get_subscription(&customer).await {
        Some(sub) => sub,
        None => {
            return Err(StatusCode::UNAUTHORIZED)
        }
    };

    println!("Got subscription: {:?}", subscription);

    if !subscription.status.eq(&stripe::SubscriptionStatus::Active) {
        return Err(StatusCode::UNAUTHORIZED)
    }

    // let's try and cancel the subscription with Stripe
    let sub = match StripeClient::cancel_subscription(&user_email).await {
        Some(sub) => sub,
        None => {
            println!("No subscription found");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    println!("Cancelled subscription: {:?}", sub);

    // If that worked, update the subscription records in supabase
    let mut updates = HashMap::new();
    updates.insert("status".to_string(), json!("cancelled"));
    updates.insert("stripe_subscription_id".to_string(), json!(""));
    updates.insert("current_period_end".to_string(), json!(Utc::now().to_rfc3339()));

    println!("updates: {:?}", updates);

    if let Err(e) = supabase.update_subscription(&supa_sub.id, updates).await {
        println!("Error occurred updating the sub in supabase: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    return Ok(StatusCode::OK)

}

async fn get_metadata(url: &str) -> Result<Metadata, StatusCode> {
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_static("Mozilla/5.0 (compatible; BetterNewTab_Bot/1.0; +http://betternewtab.com/bot)"));

    let client = reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let response = client.get(url).send().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    if !response.status().is_success() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let document = response.text().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let document = scraper::Html::parse_document(&document);

    let title_selector = scraper::Selector::parse("title").unwrap();
    let title = document.select(&title_selector).next().map(|t| t.inner_html());

    let description_selector = scraper::Selector::parse("meta[name='description']").unwrap();
    let description = document.select(&description_selector).next().and_then(|d| d.value().attr("content")).map(|d| d.to_string());

    let favicon = Some(format!("{}/favicon.ico", url.trim_end_matches('/')));

    Ok(Metadata {
        title,
        description,
        favicon,
    })
}