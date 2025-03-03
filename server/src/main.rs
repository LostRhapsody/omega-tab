// test user: evanr@fdm4.com
//TestPasswordForClerk
mod brave;
mod middleware;
mod resend;
mod stripe_client;
mod supabase;
mod user_jwt;

use axum::{
    extract::{Extension, Json, Path, State},
    http::{HeaderMap, StatusCode},
    routing::{delete, get, post},
    Router,
};
use base64::prelude::*;
use brave::Brave;
use chrono::{TimeZone, Utc};
use dotenv::dotenv;
use middleware::{authenticate_user, extract_user, UserContext};
use resend::ResendClient;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, env};
use stripe::{Event, EventType, Subscription};
use stripe_client::StripeClient;
use supabase::Supabase;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::prelude::*;
use url::Url;

#[derive(Serialize, Clone)]
pub struct SubscriptionResponse {
    plan_id: String,
    current_period_end: i64,
}

#[derive(Deserialize)]
pub struct CreateUserRequest {
    user_id: String,
    email: String,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CreateLinkRequest {
    url: String,
    description: Option<String>,
    title: Option<String>,
    next_order_index: i32,
    owner_type: String,
    owner_id: String,
    column_type: String,
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Metadata {
    title: Option<String>,
    description: Option<String>,
    favicon: Option<String>,
    mime_type: Option<String>,
}

#[derive(Serialize)]
pub struct SuggestionResponse {
    suggestions: Vec<brave::Suggestion>,
}

#[derive(Deserialize, Debug)]
pub struct FeedbackRequest {
    reasons: Option<stripe::CancellationDetailsFeedback>,
    feedback_comment: Option<String>,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserSettingsRequest {
    search_history: bool,
    autosuggest: bool,
    jira_api: bool,
    confluence_api: bool,
    linear_api: bool,
    new_tabs: bool,
}

#[derive(Serialize)]
pub struct UserDataResponse {
    user: supabase::User,
    subscription: Option<SubscriptionResponse>,
    plan: Option<supabase::Plan>,
    settings: Option<supabase::UserSettings>,
    links: Vec<supabase::Link>,
}

fn main() {
    let _guard = sentry::init(("https://dacfc75c4bbf7f8a70134067d078c21a@o4508773394153472.ingest.us.sentry.io/4508773395857408", sentry::ClientOptions {
        release: sentry::release_name!(),

        // 1.0 is send 100% of traces to Sentry, 0.2 is 20%, etc.
        traces_sample_rate: 0.2,

        ..sentry::ClientOptions::default()
    }));

    tracing_subscriber::Registry::default()
        .with(sentry::integrations::tracing::layer())
        .init();

    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            runtime().await;
        });

    println!("Ending request");
    tracing::info!("Ending request");
}

async fn runtime() {
    tracing::info!("Starting request");

    dotenv().ok();

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let client = reqwest::Client::new();

    // Reminder! Anything you return must be serializable
    let app = Router::new()
        // confirm subscription
        .route("/confirm", get(confirm_handler))
        // cancel subscription
        .route("/cancel", post(cancel_handler))
        // create and update links
        .route("/link", post(create_link).put(update_link))
        // read links
        .route("/user/links", get(links_handler))
        // delete link
        .route(
            "/link/{link_id}",
            delete(move |path, user_context| delete_link(path, user_context)),
        )
        // get plan
        .route(
            "/plan/{plan_id}",
            get(move |path, user_context| plan_handler(path, user_context)),
        )
        // create user
        .route("/create_user", post(create_user_handler))
        // get user
        .route("/user", get(get_user_handler))
        // get suggestion
        .route(
            "/suggest/{query}",
            get(move |path, user_context, headers| suggest_handler(path, user_context, headers)),
        )
        .route("/feedback", post(feedback_handler))
        .route(
            "/settings",
            post(create_settings).put(update_settings).get(get_settings),
        )
        // cancel subscription event listener for Stripe
        .route("/stripe_cancel_hook", post(cancel_subscription_hook))
        .route("/user_data", get(get_user_data_handler))
        .with_state(client)
        .layer(axum::middleware::from_fn(authenticate_user))
        .layer(axum::middleware::from_fn(extract_user))
        .layer(cors);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://0.0.0.0:3000");

    axum::serve(listener, app).await.unwrap();
}

async fn create_user_handler(
    Extension(user_context): Extension<UserContext>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<supabase::User>, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    println!("Creating new user: {}", payload.email);

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "POST");
    });

    tracing::info!("Creating new user: {}", payload.email);

    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|e| {
        tracing::error!("Error initializing Supabase client: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user = supabase::User {
        id: payload.user_id,
        email: payload.email,
        created_at: Utc::now().to_rfc3339(),
        auth_token: None,
    };

    match supabase.create_user(user.clone()).await {
        Ok(_) => {
            tracing::info!("Successfully created user: {}", user.email);
            let create_settings_result = create_user_default_settings(&user).await;
            if let Err(e) = create_settings_result {
                if e == StatusCode::INTERNAL_SERVER_ERROR {
                    tracing::error!("Failed to create user settings for user: {}", user.email);
                }
            }
            Ok(Json(user))
        }
        Err(e) => {
            tracing::error!("Failed to create user: {}", e);
            match e.to_string().as_str() {
                "409" => Err(StatusCode::BAD_REQUEST),
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
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
#[tracing::instrument]
async fn confirm_handler(
    Extension(user_context): Extension<UserContext>,
) -> Result<Json<SubscriptionResponse>, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    println!("Confirming email: {}", user_email);

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "GET");
    });

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
            println!("No customer found, returning free plan");
            return Ok(Json(SubscriptionResponse {
                plan_id: free_plan_id,
                current_period_end: 0,
            }));
        }
    };

    println!("Got customer from Stripe, getting subscription");
    // Get Stripe subscription - if one is not returned, they have not subscribed (or subscription is inactive)
    // TODO - cleanup task, if the subscription EXISTS in Supbase but NOT stripe, then the customer probably
    // unsubscribed somehow. We should cancel the subscription in Supabase as well.
    let subscription = match StripeClient::get_subscription(&customer).await {
        Some(sub) => sub,
        None => {
            println!("No subscription found, returning free plan");
            return Ok(Json(SubscriptionResponse {
                plan_id: free_plan_id,
                current_period_end: 0,
            }));
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

    /*
    Note: The subscription has been verified as active in stripe above
    so everything below is just to verify and update the subscription record in Supabase
    */

    // Verify subscription record
    let sub_result = supabase.get_user_subscription(&user.id).await;

    // verify we got the subcription
    match sub_result {
        Ok(sub) => {
            println!("Got subscription from Supabase");
            // Update existing subscription if plan changed
            // if plan id has changed (user upgraded/downgraded); or,
            // the plan exists but has been cancelled and is now being re-activated
            if (sub.plan_id != supabase_plan.id) || sub.status.eq("cancelled") {
                let mut updates = HashMap::new();
                updates.insert("plan_id".to_string(), json!(supabase_plan.id));
                updates.insert(
                    "current_period_end".to_string(),
                    json!(Utc
                        .timestamp(subscription.current_period_end, 0)
                        .to_rfc3339()),
                );
                updates.insert("stripe_subscription_id".to_string(), json!(subscription.id));
                updates.insert("status".to_string(), json!("active"));

                println!("Updating subscription");
                println!("updates: {:?}", updates);
                supabase
                    .update_subscription(&sub.id, updates)
                    .await
                    .map_err(|e| {
                        println!("Error updating subscription: {:?}", e);
                        tracing::error!("Error updating subscription: {:?}", e);
                        StatusCode::INTERNAL_SERVER_ERROR
                    })?;
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
            println!("Got membership vector from Supabase");
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
    Extension(user_context): Extension<UserContext>,
) -> Result<Json<Vec<supabase::Link>>, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    println!("Fetching links for user: {}", user_id);

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "GET");
    });

    tracing::info!("Fetching links for user: {}", user_id);

    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|e| {
        tracing::error!("Error initializing Supabase client: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let links = supabase.get_links(&user_id, "user").await.map_err(|e| {
        tracing::error!("Failed to fetch links for user {}: {:?}", user_id, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!(
        "Successfully fetched {} links for user {}",
        links.len(),
        user_id
    );
    Ok(Json(links))
}

async fn create_link(
    State(client): State<reqwest::Client>,
    Extension(user_context): Extension<UserContext>,
    headers: HeaderMap,
    Json(payload): Json<CreateLinkRequest>,
) -> Result<(StatusCode, Json<supabase::Link>), StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    println!(
        "Creating new link for owner {}: {}",
        payload.owner_id, payload.url
    );

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "POST");
    });

    tracing::info!(
        "Creating new link for owner {}: {}",
        payload.owner_id,
        payload.url
    );

    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|e| {
        tracing::error!("Error initializing Supabase client: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let url = if !payload.url.starts_with("https://") {
        format!("https://{}", payload.url)
    } else {
        payload.url
    };

    // Check for the custom authorization header
    let auth_token = headers
        .get("X-User-Authorization")
        .ok_or_else(|| {
            println!("Missing X-User-Authorization header");
            StatusCode::UNAUTHORIZED
        })?
        .to_str()
        .map_err(|e| {
            println!("Invalid X-User-Authorization header: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;

    // Validate the JWT token
    let user_claims = match user_jwt::validate_jwt(auth_token) {
        Ok(claims) => claims,
        Err(e) => {
            println!("Invalid JWT token: {:?}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Verify the user ID in the token matches the request user ID
    if user_claims.user_id != user_id {
        println!("Token user ID does not match request user ID");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // init metadata, if plus plan retrieve from link's URL, else use defaults
    let metadata = if user_claims.plan == "plus" {
        get_metadata(State(client.clone()), &url)
            .await
            .map_err(|e| {
                tracing::error!("Error getting metadata: {:?}", e);
            })
            .unwrap_or_else(|_| Metadata {
                title: Some(url.clone()),
                description: None,
                favicon: None,
                mime_type: None,
            })
    } else {
        Metadata {
            title: Some(url.clone()),
            description: None,
            favicon: None,
            mime_type: None,
        }
    };

    // use the user's title, if empty use metadata, metadata will be the URL if metadata is not fetched
    let title = if payload.title.as_deref() == Some("") {
        metadata.title.unwrap_or_else(|| "".to_string())
    } else {
        payload
            .title
            .unwrap_or_else(|| metadata.title.unwrap().clone())
    };

    // grab the favicon, or just pass an empty string
    let favicon = if user_claims.plan == "plus" {
        get_favicon(
            State(client),
            &url,
            metadata.favicon.clone(),
            metadata.mime_type.clone(),
        )
        .await
        .map_err(|e| {
            tracing::error!("Error getting favicon: {:?}", e);
        })
        .unwrap_or_else(|_| "".to_string())
    } else {
        "".to_string()
    };

    // use the user's description, or the metadata description
    let description = if payload.description.as_deref() == Some("") {
        metadata.description.unwrap_or_else(|| "".to_string())
    } else {
        payload
            .description
            .unwrap_or_else(|| metadata.description.unwrap().clone())
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
        tracing::error!("Failed to create link in database: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    tracing::info!("Successfully created link with ID: {}", link.id);
    Ok((StatusCode::CREATED, Json(link)))
}

async fn update_link(
    Extension(user_context): Extension<UserContext>,
    Json(payload): Json<UpdateLinkRequest>,
) -> Result<StatusCode, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    println!("Updating link: {}", payload.id);

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "PUT");
    });

    tracing::info!("Updating link: {}", payload.id);

    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|e| {
        tracing::error!("Error initializing Supabase client: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let mut updates = HashMap::new();
    if let Some(url) = payload.url {
        tracing::info!("Updating URL for link {}", payload.id);
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
        tracing::error!("Failed to update link {}: {:?}", payload.id, e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    tracing::info!("Successfully updated link {}", payload.id);
    Ok(StatusCode::OK)
}

async fn delete_link(
    Path(link_id): Path<String>,
    Extension(user_context): Extension<UserContext>,
) -> Result<StatusCode, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "DELETE");
    });

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

async fn plan_handler(
    Path(plan_id): Path<String>,
    Extension(user_context): Extension<UserContext>,
) -> Result<Json<supabase::Plan>, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "GET");
    });

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

async fn get_user_handler(
    Extension(user_context): Extension<UserContext>,
) -> Result<Json<supabase::User>, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "GET");
    });

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
#[axum::debug_handler]
async fn cancel_handler(
    Extension(user_context): Extension<UserContext>,
    payload: Json<FeedbackRequest>,
) -> Result<StatusCode, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    println!("Cancelling email: {}", user_email);

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "POST");
    });

    println!("Cancelling user id: {}", user_id);
    println!("Feedback: {:?}", payload);
    let feedback = payload.feedback_comment.clone();
    let reasons = payload.reasons.clone();

    // confirm the user email and user id are present and formatted well else throw a 400
    if user_email.is_empty() || user_id.is_empty() {
        println!("User email or user id is empty");
        return Err(StatusCode::BAD_REQUEST);
    }

    // first confirm if user exists, if not throw a 404
    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let _user = supabase
        .get_user(&user_id)
        .await
        .map_err(|e| match e.to_string().as_str() {
            "404" => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    println!("Got user: {:?}", _user);

    // then get the supabase subscription
    let supa_sub = supabase
        .get_user_subscription(&user_id)
        .await
        .map_err(|e| match e.to_string().as_str() {
            "404" => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    println!("Got supabase subscription: {:?}", supa_sub);

    // then confirm the user's subscription is active, if not throw a 401
    let customer = match StripeClient::get_customer(&user_email).await {
        Some(customer) => customer,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    println!("Got customer: {:?}", customer);

    let subscription = match StripeClient::get_subscription(&customer).await {
        Some(sub) => sub,
        None => return Err(StatusCode::UNAUTHORIZED),
    };

    println!("Got subscription: {:?}", subscription);

    if !subscription.status.eq(&stripe::SubscriptionStatus::Active) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // let's try and cancel the subscription with Stripe
    let sub = match StripeClient::cancel_subscription(user_email, feedback, reasons).await {
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
    updates.insert(
        "current_period_end".to_string(),
        json!(Utc::now().to_rfc3339()),
    );

    println!("updates: {:?}", updates);

    if let Err(e) = supabase.update_subscription(&supa_sub.id, updates).await {
        println!("Error occurred updating the sub in supabase: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    // todo - test the subscription flow with this
    let memberships = supabase.get_user_memberships(&user_id).await.map_err(|e| {
        println!("Error getting memberships: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    for membership in memberships {
        if let Err(e) = supabase
            .remove_member(&membership.user_id, &membership.entity_id)
            .await
        {
            println!("Error removing membership: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    return Ok(StatusCode::OK);
}

async fn get_metadata(client: State<reqwest::Client>, url: &str) -> Result<Metadata, StatusCode> {
    tracing::info!("Fetching metadata for URL: {}", url);

    println!("Fetching metadata for URL: {}", url);

    let response = client.get(url).send().await.map_err(|e| {
        tracing::error!("Failed to fetch URL {}: {:?}", url, e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    if !response.status().is_success() {
        return Err(StatusCode::BAD_REQUEST);
    }

    let document = response
        .text()
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let document = scraper::Html::parse_document(&document);

    let title_selector = scraper::Selector::parse("title").unwrap();
    let title = document
        .select(&title_selector)
        .next()
        .map(|t| t.inner_html());

    let description_selector = scraper::Selector::parse("meta[name='description']").unwrap();
    let description = document
        .select(&description_selector)
        .next()
        .and_then(|d| d.value().attr("content"))
        .map(|d| d.to_string());

    let icon_selector = scraper::Selector::parse("link[rel='icon']").unwrap();
    let favicon = document
        .select(&icon_selector)
        .next()
        .and_then(|i| i.value().attr("href"))
        .map(|i| i.to_string());

    let mime_type = document
        .select(&icon_selector)
        .next()
        .and_then(|i| i.value().attr("type"))
        .map(|i| i.to_string());

    println!("favicon source: {:?}", favicon);

    tracing::info!("Successfully fetched metadata for URL: {}", url);
    Ok(Metadata {
        title,
        description,
        favicon,
        mime_type,
    })
}

async fn get_favicon(
    client: State<reqwest::Client>,
    url: &str,
    favicon_source: Option<String>,
    mime_type: Option<String>,
) -> Result<String, StatusCode> {
    let parsed_url = Url::parse(url).expect("Invalid URL");
    let domain = parsed_url.host_str().unwrap_or("").to_string();

    let domain = if !domain.starts_with("https://") {
        format!("https://{}", domain)
    } else {
        domain
    };

    let favicon_urls = vec![
        format!("{}/favicon.ico", domain.trim_end_matches('/')),
        format!("{}/images/favicon.ico", domain.trim_end_matches('/')),
        format!("{}/assets/favicon.ico", domain.trim_end_matches('/')),
        format!("{}/static/favicon.ico", domain.trim_end_matches('/')),
        format!("{}/public/favicon.ico", domain.trim_end_matches('/')),
        format!("{}/icon/favicon.ico", domain.trim_end_matches('/')),
        format!("{}/icons/favicon.ico", domain.trim_end_matches('/')),
        format!("{}/icon.svg", domain.trim_end_matches('/')),
        format!("{}/favicon-32x32.png", domain.trim_end_matches('/')),
    ];

    let mut favicon: Option<String> = None;

    // if we found a source link tag while parsing the page's document, grab that
    // otherwise we just try some backups
    if favicon_source.is_some() {
        let favicon_url = format!(
            "{}{}",
            domain.trim_end_matches('/'),
            &favicon_source.unwrap()
        );
        let fav_response = client.get(favicon_url).send().await;

        if let Ok(fav_response) = fav_response {
            if fav_response.status().is_success() {
                if let Ok(fav_bytes) = fav_response.bytes().await {
                    favicon = Some(format!(
                        "data:{};base64,{}",
                        mime_type.unwrap_or_else(|| "unknown".to_string()),
                        BASE64_STANDARD.encode(fav_bytes)
                    ));
                }
            }
        }
    } else {
        for favicon_url in favicon_urls {
            if let Ok(fav_response) = client.get(&favicon_url).send().await {
                if fav_response.status().is_success() {
                    if let Ok(fav_bytes) = fav_response.bytes().await {
                        favicon = Some(format!(
                            "data:{};base64,{}",
                            mime_type.unwrap_or_else(|| "unknown".to_string()),
                            BASE64_STANDARD.encode(fav_bytes)
                        ));
                        break;
                    }
                }
            }
        }
    }

    if favicon.is_none() {
        favicon = Some("".to_string());
    }

    Ok(favicon.unwrap())
}

async fn suggest_handler(
    Path(query): Path<String>,
    Extension(user_context): Extension<UserContext>,
    headers: HeaderMap,
) -> Result<Json<SuggestionResponse>, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    println!("Suggesting: {}", query);

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "GET");
    });

    // Check for the custom authorization header
    let auth_token = headers
        .get("X-User-Authorization")
        .ok_or_else(|| {
            println!("Missing X-User-Authorization header");
            StatusCode::UNAUTHORIZED
        })?
        .to_str()
        .map_err(|e| {
            println!("Invalid X-User-Authorization header: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;

    // Validate the JWT token
    let user_claims = match user_jwt::validate_jwt(auth_token) {
        Ok(claims) => claims,
        Err(e) => {
            println!("Invalid JWT token: {:?}", e);
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // Verify the user ID in the token matches the request user ID
    if user_claims.user_id != user_id {
        println!("Token user ID does not match request user ID");
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Check if token is for a "plus" plan user or if their plan allows this feature
    if user_claims.plan != "plus" {
        println!("User plan does not allow auto-suggestions");
        return Err(StatusCode::FORBIDDEN);
    }

    println!("Suggesting: {}", query);

    let brave = Brave::new(
        std::env::var("BRAVE_SUGGEST_URL").expect("BRAVE_URL must be set"),
        std::env::var("BRAVE_TEST_KEY").expect("BRAVE_API_KEY must be set"),
    )
    .map_err(|e| {
        println!("Error initializing Brave client: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = brave.get_suggestions(&query).await.map_err(|e| {
        println!("Error getting suggestions: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(SuggestionResponse {
        suggestions: response.results,
    }))
}

async fn feedback_handler(
    Extension(user_context): Extension<UserContext>,
    Json(payload): Json<FeedbackRequest>,
) -> Result<StatusCode, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    println!("Feedback for user: {}", user_id);

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "POST");
    });

    let user_id = user_context.user_id.clone();
    let user_email = user_context.email.clone();
    println!("Feedback for user: {}", user_id);

    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    // Check if the user has sent feedback in the last 24 hours
    let can_send_feedback = supabase
        .check_feedback_timestamp(&user_id)
        .await
        .map_err(|e| {
            println!("Error checking feedback timestamp: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    if !can_send_feedback {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    let resend_service = ResendClient::new();

    let customer_support_email =
        std::env::var("CUSTOMER_SUPPORT_EMAIL").expect("CUSTOMER_SUPPORT_EMAIL must be set");

    let email_body = format!(
        "<p>Feedback from user: {} | {}<br/><br/>Reasons: {:?}<br/><br/>Feedback: {}</p>",
        user_id,
        user_email,
        payload.reasons,
        payload.feedback_comment.unwrap_or_else(|| "".to_string())
    );

    let subject = format!("Feedback from: {}", user_email);

    resend_service
        .send_email(&customer_support_email, &subject, &email_body)
        .await
        .map_err(|e| {
            println!("Error sending email: {:?}", e);
            tracing::error!("Error sending email: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // Create a feedback timestamp record
    supabase
        .create_feedback_timestamp(&user_id, &Utc::now().to_rfc3339())
        .await
        .map_err(|e| {
            println!("Error creating feedback timestamp: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}

async fn create_settings(
    Extension(user_context): Extension<UserContext>,
    Json(payload): Json<UserSettingsRequest>,
) -> Result<StatusCode, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    println!("Creating settings for user: {}", user_id);

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "POST");
    });

    println!("Creating settings for user: {}", user_id);
    println!("Payload: {:?}", payload);

    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let settings = supabase::UserSettings {
        user_id: user_id.clone(),
        settings_blob: json!(payload),
        created_at: Utc::now().to_rfc3339(),
    };

    if let Err(e) = supabase.create_user_settings(settings).await {
        println!("Error creating user settings: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::CREATED)
}

async fn update_settings(
    Extension(user_context): Extension<UserContext>,
    Json(payload): Json<UserSettingsRequest>,
) -> Result<StatusCode, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    println!("Updating settings for user: {}", user_id);

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "PUT");
    });

    println!("Updating settings for user: {}", user_id);
    println!("Payload: {:?}", payload);

    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let mut updates = HashMap::new();
    updates.insert("settings_blob".to_string(), json!(payload));

    if let Err(e) = supabase.update_user_settings(&user_id, updates).await {
        println!("Error updating user settings: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}

async fn get_settings(
    Extension(user_context): Extension<UserContext>,
) -> Result<Json<supabase::UserSettings>, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    println!("Getting settings for user: {}", user_id);

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "GET");
    });

    println!("Getting settings for user: {}", user_id);

    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let settings =
        supabase
            .get_user_settings(&user_id)
            .await
            .map_err(|e| match e.to_string().as_str() {
                "404" => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            })?;

    Ok(Json(settings))
}

async fn cancel_subscription_hook(
    headers: HeaderMap,
    Json(payload): Json<Event>,
) -> Result<StatusCode, StatusCode> {
    println!("Received cancel subscription webhook");

    sentry::configure_scope(|scope| {
        scope.set_tag("http.method", "POST");
    });

    let endpoint_secret =
        env::var("STRIPE_ENDPOINT_SECRET").expect("STRIPE_ENDPOINT_SECRET must be set");
    let verify_signature = env::var("STRIPE_VERIFY_WEBHOOK_SIGNATURE")
        .expect("STRIPE_VERIFY_WEBHOOK_SIGNATURE must be set");

    let signature = headers
        .get("Stripe-Signature")
        .ok_or_else(|| {
            println!("Missing Stripe-Signature header");
            StatusCode::BAD_REQUEST
        })?
        .to_str()
        .map_err(|e| {
            println!("Error parsing Stripe-Signature header: {:?}", e);
            StatusCode::BAD_REQUEST
        })?;

    let payload_str = serde_json::to_string(&payload).map_err(|e| {
        println!("Error serializing payload: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let event = if verify_signature == "true" {
        stripe::Webhook::construct_event(&payload_str, signature, &endpoint_secret).map_err(
            |e| {
                println!("Error constructing Stripe event: {:?}", e);
                StatusCode::BAD_REQUEST
            },
        )?
    } else {
        serde_json::from_str::<Event>(&payload_str).map_err(|e| {
            println!("Error deserializing event: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?
    };

    if event.type_ != EventType::CustomerSubscriptionDeleted {
        println!("Unexpected event type: {:?}", event.type_);
        return Err(StatusCode::BAD_REQUEST);
    }

    let event_data = serde_json::to_value(event.data.object).map_err(|e| {
        println!("Error converting event data to value: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let subscription: Subscription = serde_json::from_value(event_data).map_err(|e| {
        println!("Error deserializing subscription: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let customer_id = subscription.customer.id();

    let user_email = StripeClient::get_customer_email(&customer_id)
        .await
        .map_err(|e| {
            println!(
                "Error retrieving customer email for ID: {:?}, error: {:?}",
                customer_id, e
            );
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    let user_email = match user_email {
        Some(email) => email,
        None => {
            println!("No email found for customer ID: {:?}", customer_id);
            return Err(StatusCode::NOT_FOUND);
        }
    };

    let supabase = Supabase::new(
        env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|e| {
        println!("Error initializing Supabase client: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let user = supabase.get_user_by_email(&user_email).await.map_err(|e| {
        println!("Error retrieving user by email: {:?}", e);
        StatusCode::NOT_FOUND
    })?;

    let mut updates = HashMap::new();
    updates.insert("status".to_string(), json!("cancelled"));
    updates.insert("stripe_subscription_id".to_string(), json!(subscription.id));
    updates.insert(
        "current_period_end".to_string(),
        json!(Utc::now().to_rfc3339()),
    );

    supabase
        .update_subscription(&user.id, updates)
        .await
        .map_err(|e| {
            println!("Error updating subscription: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}

async fn get_user_data_handler(
    Extension(user_context): Extension<UserContext>,
) -> Result<Json<UserDataResponse>, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    println!("Fetching user data!");

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "GET");
    });

    println!("Fetching user data!");
    let user_id = user_context.user_id.clone();
    let user_email = user_context.email.clone();
    tracing::info!("Fetching user data for {}", user_email);

    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|e| {
        tracing::error!("Error initializing Supabase client: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Get or create user
    let user = match supabase.get_user(&user_id).await {
        Ok(user) => {
            tracing::info!("Found existing user: {}", user.email);
            user
        }
        Err(_) => {
            tracing::info!("Creating new user: {}", user_email);
            let new_user = supabase::User {
                id: user_id.clone(),
                email: user_email.clone(),
                created_at: Utc::now().to_rfc3339(),
                auth_token: None,
            };
            supabase.create_user(new_user.clone()).await.map_err(|e| {
                tracing::error!("Failed to create user: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            let create_settings_result = create_user_default_settings(&new_user).await;
            if let Err(e) = create_settings_result {
                if e == StatusCode::INTERNAL_SERVER_ERROR {
                    tracing::error!(
                        "Failed to create user settings for user: {}",
                        new_user.email
                    );
                }
            }
            new_user
        }
    };

    tracing::info!("Fetching subscription info for {}", user_email);
    // Get subscription info
    let subscription = match StripeClient::get_customer(&user_email).await {
        Some(customer) => match StripeClient::get_subscription(&customer).await {
            Some(sub) if sub.status.eq(&stripe::SubscriptionStatus::Active) => {
                let item = sub
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

                let supabase_plan = supabase
                    .get_plan_by_stripe_id(&product_id.to_string())
                    .await
                    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

                Some((
                    SubscriptionResponse {
                        plan_id: supabase_plan.id.clone(),
                        current_period_end: sub.current_period_end,
                    },
                    supabase_plan,
                ))
            }
            _ => None,
        },
        None => None,
    };

    tracing::info!("Fetching user settings for {}", user_id);
    let settings = match supabase.get_user_settings(&user_id).await {
        Ok(settings) => Some(settings),
        Err(_) => match create_user_default_settings(&user).await {
            Ok(new_settings) => Some(new_settings),
            Err(e) => {
                if e == StatusCode::INTERNAL_SERVER_ERROR {
                    tracing::error!("Failed to create user settings for user: {}", user.email);
                }
                None
            }
        },
    };

    tracing::info!("Fetching links for user {}", user_id);
    let links = supabase.get_links(&user_id, "user").await.map_err(|e| {
        tracing::error!("Failed to fetch links: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let free_plan_id = std::env::var("FREE_PLAN_ID").expect("FREE_PLAN_ID must be set");
    let free_plan = if subscription.is_none() {
        Some(
            supabase
                .get_plan(&free_plan_id)
                .await
                .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?,
        )
    } else {
        None
    };

    // Generate JWT token with user ID and plan info
    let plan_name = subscription
        .as_ref()
        .map(|(_, plan)| plan.name.clone())
        .or_else(|| free_plan.as_ref().map(|p| p.name.clone()))
        .unwrap_or_else(|| "free".to_string());

    let auth_token = user_jwt::generate_jwt(&user_id, &plan_name).map_err(|e| {
        tracing::error!("Failed to generate JWT token: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!(
        "Successfully assembled user data response for {}",
        user_email
    );

    // Create response with user data and authorization token
    let mut response = UserDataResponse {
        user,
        subscription: subscription.as_ref().map(|(sub, _)| sub.clone()).or(Some(
            SubscriptionResponse {
                plan_id: free_plan_id.clone(),
                current_period_end: 0,
            },
        )),
        plan: subscription.map(|(_, plan)| plan).or(free_plan),
        settings,
        links,
    };

    // Add the auth_token to the response
    response.user.auth_token = Some(auth_token);

    Ok(Json(response))
}

async fn create_user_default_settings(
    user: &crate::supabase::User,
) -> Result<supabase::UserSettings, StatusCode> {
    println!("Creating default user settings");

    let supabase = Supabase::new(
        std::env::var("SUPABASE_URL").expect("SUPABASE_URL must be set"),
        std::env::var("SUPABASE_KEY").expect("SUPABASE_KEY must be set"),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let settings_blob = UserSettingsRequest {
        search_history: false,
        autosuggest: false,
        jira_api: false,
        confluence_api: false,
        linear_api: false,
        new_tabs: false,
    };

    let user_settings = supabase::UserSettings {
        user_id: user.id.clone(),
        settings_blob: json!(settings_blob),
        created_at: Utc::now().to_rfc3339(),
    };

    let settings = supabase.get_user_settings(&user.id).await.ok();
    if settings.is_none() {
        if let Err(e) = supabase.create_user_settings(user_settings.clone()).await {
            println!("Failed to create user settings: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        return Ok(user_settings);
    } else {
        return Err(StatusCode::FOUND);
    }
}
