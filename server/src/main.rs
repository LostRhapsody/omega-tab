// Hide console window on Windows in release builds
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod assets;
mod brave;
mod dashboard_icons;
mod database;
mod middleware;
mod resend;
mod tray;
mod user_jwt;

use axum::{
    Router,
    extract::{Extension, Json, Path, State},
    http::{HeaderMap, HeaderValue, Method, StatusCode},
    routing::{delete, get, post},
};
use base64::prelude::*;
use brave::Brave;
use chrono::Utc;
use database::Database;
use dotenv::dotenv;
use middleware::{UserContext, authenticate_user};
use resend::ResendClient;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, env, sync::mpsc, thread};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::prelude::*;
use tray::TrayMessage;
use url::Url;

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
    icon: Option<String>,
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

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct UserSettingsRequest {
    search_history: bool,
    autosuggest: bool,
    jira_api: bool,
    confluence_api: bool,
    linear_api: bool,
    new_tabs: bool,
    metadata: bool,
}

#[derive(Deserialize, Debug)]
pub struct FeedbackRequest {
    reasons: Option<String>,
    feedback_comment: Option<String>,
}

#[derive(Serialize)]
pub struct UserDataResponse {
    user: database::User,
    settings: Option<database::UserSettings>,
    links: Vec<database::Link>,
}

// Authentication request/response structs
#[derive(Deserialize, Debug)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Deserialize, Debug)]
pub struct RegisterRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
pub struct AuthResponse {
    token: String,
    user: database::User,
}

// New struct for staging login request
#[derive(Deserialize, Debug)]
pub struct StagingLoginRequest {
    password: String,
}

#[derive(Clone)]
pub struct AppState {
    pub client: reqwest::Client,
    pub database: Database,
}

fn main() {
    dotenv().ok();

    // Initialize Sentry
    let sample_rate = std::env::var("TRACING_SAMPLE_RATE")
        .unwrap_or_else(|_| "0.2".to_string())
        .parse::<f32>()
        .unwrap_or(0.2);
    let _guard = sentry::init((
        "https://dacfc75c4bbf7f8a70134067d078c21a@o4508773394153472.ingest.us.sentry.io/4508773395857408",
        sentry::ClientOptions {
            release: sentry::release_name!(),
            traces_sample_rate: sample_rate,
            ..sentry::ClientOptions::default()
        },
    ));

    tracing_subscriber::Registry::default()
        .with(sentry::integrations::tracing::layer())
        .init();

    // Create channel for shutdown signal
    let (shutdown_tx, shutdown_rx) = mpsc::channel();

    // Spawn tokio runtime in a separate thread
    let server_handle = thread::spawn(move || {
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async {
                runtime(shutdown_rx).await;
            });
    });

    // Create system tray on main thread (required for macOS)
    match tray::create_tray() {
        Ok((_tray, rx)) => {
            println!("System tray created successfully");
            tracing::info!("System tray created successfully");

            // Wait for exit message from tray
            loop {
                match rx.recv() {
                    Ok(TrayMessage::Exit) => {
                        println!("Exit requested from tray");
                        tracing::info!("Exit requested from tray");
                        let _ = shutdown_tx.send(());
                        break;
                    }
                    Err(_) => {
                        // Channel closed, tray thread exited
                        break;
                    }
                }
            }
        }
        Err(e) => {
            // If tray creation fails (e.g., headless environment), just run the server
            eprintln!("Failed to create system tray: {:?}", e);
            tracing::warn!("Failed to create system tray: {:?}", e);
            println!("Running in headless mode. Press Ctrl+C to exit.");

            // Wait for Ctrl+C
            let _ = ctrlc_channel();
            let _ = shutdown_tx.send(());
        }
    }

    // Wait for server to finish
    let _ = server_handle.join();

    println!("Omega Tab shutting down");
    tracing::info!("Omega Tab shutting down");
}

/// Create a simple channel that receives when Ctrl+C is pressed
fn ctrlc_channel() -> mpsc::Receiver<()> {
    let (tx, rx) = mpsc::channel();
    ctrlc::set_handler(move || {
        let _ = tx.send(());
    })
    .expect("Error setting Ctrl+C handler");
    rx
}

async fn runtime(shutdown_rx: mpsc::Receiver<()>) {
    tracing::info!("Starting Omega Tab server");

    let cors = {
        let environment =
            std::env::var("ENVIRONMENT").unwrap_or_else(|_| "development".to_string());

        match environment.as_str() {
            "production" => CorsLayer::new()
                .allow_origin(
                    "https://omega-tab.evanrobertson.dev"
                        .parse::<HeaderValue>()
                        .unwrap(),
                )
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers(Any),
            "staging" => CorsLayer::new()
                .allow_origin(
                    "https://staging.omega-tab.evanrobertson.dev"
                        .parse::<HeaderValue>()
                        .unwrap(),
                )
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::DELETE,
                    Method::OPTIONS,
                ])
                .allow_headers(Any),
            _ => {
                // Development mode - also allow localhost for single binary
                CorsLayer::new()
                    .allow_origin([
                        "http://localhost:5173".parse::<HeaderValue>().unwrap(),
                        "http://localhost:3000".parse::<HeaderValue>().unwrap(),
                        "http://127.0.0.1:3000".parse::<HeaderValue>().unwrap(),
                    ])
                    .allow_methods([
                        Method::GET,
                        Method::POST,
                        Method::PUT,
                        Method::DELETE,
                        Method::OPTIONS,
                    ])
                    .allow_headers(Any)
            }
        }
    };

    let client = reqwest::Client::new();

    // DATABASE_URL is optional now - we use SQLite with a platform-appropriate path
    let database_url = std::env::var("DATABASE_URL").unwrap_or_default();
    let database = match Database::new(database_url).await {
        Ok(database) => database,
        Err(e) => {
            tracing::error!("Error initializing database connection: {:?}", e);
            eprintln!("Error initializing database: {:?}", e);
            return;
        }
    };

    let app_state = AppState { client, database };

    // Build API router with /api prefix
    let api_routes = Router::new()
        // Authentication routes (public - no middleware)
        .route("/register", post(register_handler))
        .route("/login", post(login_handler))
        .route("/health", get(health_check))
        // create and update links
        .route("/link", post(create_link).put(update_link))
        // read links
        .route("/user/links", get(links_handler))
        // delete link
        .route(
            "/link/{link_id}",
            delete(move |state: State<AppState>, path, user_context| {
                delete_link(state, path, user_context)
            }),
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
        .route("/user_data", get(get_user_data_handler))
        // Add staging login route - doesn't need authentication
        .route("/staging_login", post(staging_login_handler))
        // Dashboard icons search
        .route("/icons/search/{query}", get(search_icons_handler))
        .with_state(app_state)
        .layer(axum::middleware::from_fn(authenticate_user));

    // Main router with API routes nested under /api and static file fallback
    let app = Router::new()
        .nest("/api", api_routes)
        .layer(cors)
        // Fallback to static file serving for SPA
        .fallback(assets::serve_static);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("Server running on http://127.0.0.1:3000");
    tracing::info!("Server running on http://127.0.0.1:3000");

    // Serve with graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(async move {
            // Wait for shutdown signal in a tokio-compatible way
            tokio::task::spawn_blocking(move || {
                let _ = shutdown_rx.recv();
            })
            .await
            .ok();
        })
        .await
        .unwrap();
}

// Health check endpoint
async fn health_check() -> StatusCode {
    StatusCode::OK
}

// Register handler
async fn register_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    tracing::info!("Processing registration request for: {}", payload.email);

    let database = &app_state.database;

    // Register the user (this will hash the password)
    let mut user = database
        .register_user(&payload.email, &payload.password)
        .await
        .map_err(|e| {
            tracing::error!("Registration failed: {:?}", e);
            if e.to_string().contains("already exists") {
                StatusCode::CONFLICT
            } else {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        })?;

    // Generate JWT token
    let token = user_jwt::generate_jwt(&user.id, &user.email).map_err(|e| {
        tracing::error!("Failed to generate JWT: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Set auth_token in user object
    user.auth_token = Some(token.clone());

    tracing::info!("Successfully registered user: {}", user.email);

    Ok(Json(AuthResponse { token, user }))
}

// Login handler
async fn login_handler(
    State(app_state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, StatusCode> {
    tracing::info!("Processing login request for: {}", payload.email);

    let database = &app_state.database;

    // Verify password
    let mut user = database
        .verify_password(&payload.email, &payload.password)
        .await
        .map_err(|e| {
            tracing::warn!("Login failed for {}: {:?}", payload.email, e);
            StatusCode::UNAUTHORIZED
        })?;

    // Generate JWT token
    let token = user_jwt::generate_jwt(&user.id, &user.email).map_err(|e| {
        tracing::error!("Failed to generate JWT: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Set auth_token in user object
    user.auth_token = Some(token.clone());

    tracing::info!("Successfully logged in user: {}", user.email);

    Ok(Json(AuthResponse { token, user }))
}

// Staging login handler
async fn staging_login_handler(
    Json(payload): Json<StagingLoginRequest>,
) -> Result<StatusCode, StatusCode> {
    println!("Processing staging login request");

    // Get the staging password from environment variables
    let staging_password = match env::var("STAGING_PASSWORD") {
        Ok(pwd) => pwd,
        Err(_) => {
            println!("STAGING_PASSWORD environment variable not set");
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    // Simple password validation
    if payload.password == staging_password {
        println!("Staging login successful");
        return Ok(StatusCode::OK);
    } else {
        println!("Invalid staging password provided");
        return Err(StatusCode::FORBIDDEN);
    }
}

async fn create_user_handler(
    State(app_state): State<AppState>,
    Extension(user_context): Extension<UserContext>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<Json<database::User>, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    let database = &app_state.database;

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

    let user = database::User {
        id: payload.user_id,
        email: payload.email,
        created_at: Utc::now().to_rfc3339(),
        password_hash: String::new(), // Legacy endpoint - password not used
        auth_token: None,
    };

    match database.create_user(user.clone()).await {
        Ok(_) => Ok(Json(user)),
        Err(e) => {
            tracing::error!("Failed to create user: {}", e);
            match e.to_string().as_str() {
                "409" => Err(StatusCode::BAD_REQUEST),
                _ => Err(StatusCode::INTERNAL_SERVER_ERROR),
            }
        }
    }
}

async fn links_handler(
    State(app_state): State<AppState>,
    Extension(user_context): Extension<UserContext>,
) -> Result<Json<Vec<database::Link>>, StatusCode> {
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

    // Use app_state's database instance
    let database = &app_state.database;

    let links = database.get_links(&user_id, "user").await.map_err(|e| {
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
    State(app_state): State<AppState>,
    Extension(user_context): Extension<UserContext>,
    headers: HeaderMap,
    Json(payload): Json<CreateLinkRequest>,
) -> Result<(StatusCode, Json<database::Link>), StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    let client = &app_state.client;
    let database = &app_state.database;

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

    let metadata_on = headers
        .get("X-Fetch-Metadata")
        .and_then(|m| m.to_str().ok())
        .map(|s| s.to_lowercase() == "true")
        .unwrap_or(false);

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

    // init metadata, retrieve from link's URL, else use defaults
    let metadata = if metadata_on {
        match get_metadata(State(client.clone()), &url).await {
            Ok(metadata) => metadata,
            Err(StatusCode::BAD_GATEWAY) => {
                // If we get BAD_GATEWAY from get_metadata, return it directly to the client
                return Err(StatusCode::BAD_GATEWAY);
            }
            Err(_) => {
                // For any other errors, use default metadata
                Metadata {
                    title: Some(url.clone()),
                    description: None,
                    favicon: None,
                    mime_type: None,
                }
            }
        }
    } else {
        Metadata {
            title: Some(url.clone()),
            description: None,
            favicon: None,
            mime_type: None,
        }
    };

    // use the user's title, if empty use metadata, metadata will be
    // the URL if metadata is not fetched
    let title = if payload.title.as_deref() == Some("") {
        metadata.title.unwrap_or_else(|| "".to_string())
    } else {
        payload
            .title
            .unwrap_or_else(|| metadata.title.unwrap().clone())
    };

    // grab the favicon if meta data is turned on the user
    // did not provide one or just pass an empty string
    let favicon = if payload.icon.is_none() && metadata_on {
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
    } else if payload.icon.is_some() {
        payload.icon.unwrap()
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

    let link = database::Link {
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

    if let Err(e) = database.create_link(link.clone()).await {
        tracing::error!("Failed to create link in database: {:?}", e);
        println!("Failed to create link in database: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    tracing::info!("Successfully created link with ID: {}", link.id);
    Ok((StatusCode::CREATED, Json(link)))
}

async fn update_link(
    State(app_state): State<AppState>,
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

    // Use app_state's database instance
    let database = &app_state.database;

    let link = database::Link {
        id: payload.id.clone(),
        url: payload.url.clone().unwrap_or_else(|| "".to_string()),
        description: payload.description.clone(),
        title: payload.title.clone().unwrap(),
        icon: payload.icon.clone(),
        column_type: payload.column_type.clone().unwrap(),
        created_at: Utc::now().to_rfc3339(),
        order_index: 0,
        owner_type: "".to_string(),
        owner_id: "".to_string(),
    };

    if let Err(e) = database.update_link(link).await {
        tracing::error!("Failed to update link: {:?}", e);
        println!("Failed to update link: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    tracing::info!("Successfully updated link {}", payload.id);
    Ok(StatusCode::OK)
}

async fn delete_link(
    State(app_state): State<AppState>,
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

    // Use app_state's database instance
    let database = &app_state.database;

    // First, verify that the link exists and belongs to the user
    let link = database.get_link(&link_id, &user_id).await.map_err(|e| {
        tracing::warn!("Link not found or unauthorized: {:?}", e);
        if e.to_string().contains("404") {
            StatusCode::NOT_FOUND
        } else {
            StatusCode::FORBIDDEN
        }
    })?;

    // Verify ownership
    if link.owner_id != user_id {
        tracing::warn!(
            "User {} attempted to delete link {} owned by {}",
            user_id,
            link_id,
            link.owner_id
        );
        return Err(StatusCode::FORBIDDEN);
    }

    // Delete the link
    if let Err(e) = database.delete_link(&link_id).await {
        tracing::error!("Error deleting link: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    tracing::info!("Successfully deleted link: {}", link_id);
    Ok(StatusCode::NO_CONTENT)
}

async fn get_user_handler(
    State(app_state): State<AppState>,
    Extension(user_context): Extension<UserContext>,
) -> Result<Json<database::User>, StatusCode> {
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

    // Use app_state's database instance
    let database = &app_state.database;

    let user = database
        .get_user(&user_id)
        .await
        .map_err(|e| match e.to_string().as_str() {
            "404" => StatusCode::NOT_FOUND,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        })?;

    Ok(Json(user))
}

async fn get_metadata(client: State<reqwest::Client>, url: &str) -> Result<Metadata, StatusCode> {
    tracing::info!("Fetching metadata for URL: {}", url);

    println!("Fetching metadata for URL: {}", url);

    // Attempt to fetch the URL with proper error handling
    let response = match client.get(url).send().await {
        Ok(response) => response,
        Err(e) => {
            tracing::info!("Failed to fetch URL {}: {:?}", url, e);
            println!("Error fetching metadata: {:?}", e);
            return Err(StatusCode::BAD_GATEWAY);
        }
    };

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
    client: State<&reqwest::Client>,
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

    println!("Suggesting: {}", query);

    let brave = Brave::new(
        std::env::var("BRAVE_SUGGEST_URL").expect("BRAVE_URL must be set"),
        std::env::var("BRAVE_API_KEY").expect("BRAVE_API_KEY must be set"),
    )
    .map_err(|e| {
        println!("Error initializing Brave client: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let response = brave.get_suggestions(&query).await.map_err(|e| {
        // Check for rate limit error specifically
        if e.to_string().contains("429") {
            println!("Rate limit exceeded for Brave API");
            return StatusCode::TOO_MANY_REQUESTS;
        }
        println!("Error getting suggestions: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(SuggestionResponse {
        suggestions: response.results,
    }))
}

async fn search_icons_handler(
    Path(query): Path<String>,
    Extension(user_context): Extension<UserContext>,
) -> Result<Json<dashboard_icons::IconSearchResponse>, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    println!("Searching icons for query: {}", query);

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "GET");
    });

    tracing::info!("Searching dashboard icons for query: {}", query);

    let dashboard_icons = dashboard_icons::DashboardIcons::new();
    let response = dashboard_icons.search(&query).await.map_err(|e| {
        tracing::error!("Error searching icons: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Json(response))
}

async fn feedback_handler(
    State(app_state): State<AppState>,
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

    // Use app_state's database instance
    let database = &app_state.database;

    // Check if the user has sent feedback in the last 24 hours
    let can_send_feedback = database
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
    database
        .create_feedback_timestamp(&user_id, &Utc::now())
        .await
        .map_err(|e| {
            println!("Error creating feedback timestamp: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    Ok(StatusCode::OK)
}

async fn create_settings(
    State(app_state): State<AppState>,
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

    // Use app_state's database instance
    let database = &app_state.database;

    let settings = database::UserSettings {
        user_id: user_id.clone(),
        settings_blob: serde_json::to_string(&payload).unwrap_or_default(),
        created_at: Utc::now().to_rfc3339(),
    };

    if let Err(e) = database.create_user_settings(settings).await {
        println!("Error creating user settings: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::CREATED)
}

async fn update_settings(
    State(app_state): State<AppState>,
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

    // Use app_state's database instance
    let database = &app_state.database;

    let mut updates = HashMap::new();
    updates.insert("settings_blob".to_string(), json!(payload));

    if let Err(e) = database.update_user_settings(&user_id, updates).await {
        println!("Error updating user settings: {:?}", e);
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    }

    Ok(StatusCode::OK)
}

async fn get_settings(
    State(app_state): State<AppState>,
    Extension(user_context): Extension<UserContext>,
) -> Result<Json<database::UserSettings>, StatusCode> {
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

    // Use app_state's database instance
    let database = &app_state.database;

    let settings =
        database
            .get_user_settings(&user_id)
            .await
            .map_err(|e| match e.to_string().as_str() {
                "404" => StatusCode::NOT_FOUND,
                _ => StatusCode::INTERNAL_SERVER_ERROR,
            })?;

    Ok(Json(settings))
}

async fn get_user_data_handler(
    State(app_state): State<AppState>,
    Extension(user_context): Extension<UserContext>,
) -> Result<Json<UserDataResponse>, StatusCode> {
    let user_email = user_context.email.clone();
    let user_id = user_context.user_id.clone();
    let mut new_user_created = false;
    let mut settings: Option<database::UserSettings> = None;
    println!("Fetching user data for {}", user_email);
    tracing::info!("Fetching user data for {}", user_email);

    sentry::configure_scope(|scope| {
        scope.set_user(Some(sentry::User {
            email: Some(user_email.clone()),
            id: Some(user_id.clone()),
            ..Default::default()
        }));
        scope.set_tag("http.method", "GET");
    });

    let database = &app_state.database;

    // Get or create user
    let user = match database.get_user(&user_id).await {
        Ok(user) => {
            tracing::info!("Found existing user: {}", user.email);
            println!("Found existing user: {}", user.email);
            user
        }
        Err(e) => {
            println!("Error fetching user: {:?}", e);
            tracing::info!("Creating new user: {}", user_email);
            println!("Creating new user: {}", user_email);
            let new_user = database::User {
                id: user_id.clone(),
                email: user_email.clone(),
                created_at: Utc::now().to_rfc3339(),
                auth_token: None,
                password_hash: String::new(),
            };
            database.create_user(new_user.clone()).await.map_err(|e| {
                tracing::error!("Failed to create user: {:?}", e);
                println!("Failed to create user: {:?}", e);
                StatusCode::INTERNAL_SERVER_ERROR
            })?;
            new_user_created = true;
            let create_settings_result = create_user_default_settings(&app_state, &new_user).await;
            if let Err(e) = &create_settings_result {
                if *e == StatusCode::INTERNAL_SERVER_ERROR {
                    tracing::error!(
                        "Failed to create user settings for user: {}",
                        new_user.email
                    );
                    println!(
                        "Failed to create user settings for user: {}",
                        new_user.email
                    );
                }
            } else {
                settings = Some(create_settings_result.unwrap());
            }
            new_user
        }
    };

    tracing::info!("Fetching user settings for {}", user_id);
    println!("Fetching user settings for {}", user_id);
    if !new_user_created {
        settings = match database.get_user_settings(&user_id).await {
            Ok(settings) => Some(settings),
            Err(_) => match create_user_default_settings(&app_state, &user).await {
                Ok(new_settings) => Some(new_settings),
                Err(e) => {
                    if e == StatusCode::INTERNAL_SERVER_ERROR {
                        tracing::error!("Failed to create user settings for user: {}", user.email);
                        println!("Failed to create user settings for user: {}", user.email);
                    }
                    None
                }
            },
        };
    }

    tracing::info!("Fetching links for user {}", user_id);
    println!("Fetching links for user {}", user_id);
    let links = database.get_links(&user_id, "user").await.map_err(|e| {
        tracing::error!("Failed to fetch links: {:?}", e);
        println!("Failed to fetch links: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    // Generate JWT token with user ID
    let auth_token = user_jwt::generate_jwt(&user_id, &user_email).map_err(|e| {
        tracing::error!("Failed to generate JWT token: {:?}", e);
        println!("Failed to generate JWT token: {:?}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    tracing::info!(
        "Successfully assembled user data response for {}",
        user_email
    );
    println!(
        "Successfully assembled user data response for {}",
        user_email
    );

    // Create response with user data
    let mut response = UserDataResponse {
        user,
        settings,
        links,
    };

    response.user.auth_token = Some(auth_token);

    Ok(Json(response))
}

async fn create_user_default_settings(
    app_state: &AppState,
    user: &crate::database::User,
) -> Result<database::UserSettings, StatusCode> {
    println!("Creating default user settings");

    // Use app_state's database instance
    let database = &app_state.database;

    let settings_blob = UserSettingsRequest {
        search_history: false,
        autosuggest: false,
        jira_api: false,
        confluence_api: false,
        linear_api: false,
        new_tabs: false,
        metadata: false,
    };

    let user_settings = database::UserSettings {
        user_id: user.id.clone(),
        settings_blob: serde_json::to_string(&settings_blob).unwrap_or_default(),
        created_at: Utc::now().to_rfc3339(),
    };

    let settings = database.get_user_settings(&user.id).await.ok();
    if settings.is_none() {
        if let Err(e) = database.create_user_settings(user_settings.clone()).await {
            println!("Failed to create user settings: {:?}", e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
        return Ok(user_settings);
    } else {
        return Err(StatusCode::FOUND);
    }
}
