use std::collections::HashMap;
use std::path::PathBuf;

use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{DateTime, Utc};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use sqlx::{
    sqlite::{SqlitePool, SqlitePoolOptions},
    FromRow, Row,
};

// Type definitions matching Database.ts
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub created_at: String,
    #[sqlx(skip)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Link {
    pub id: String,
    pub title: String,
    pub url: String,
    pub icon: Option<String>,
    pub order_index: i32,
    pub owner_type: String,
    pub owner_id: String,
    pub created_at: String,
    pub description: Option<String>,
    pub column_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub max_pins: i32,
    pub features: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, FromRow)]
pub struct Subscription {
    pub id: String,
    pub entity_id: String,
    pub entity_type: String,
    pub plan_id: String,
    pub status: String,
    pub current_period_end: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserMembership {
    pub user_id: String,
    pub entity_id: String,
    pub entity_type: String,
    pub role: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct UserSettings {
    pub user_id: String,
    pub settings_blob: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: User,
    pub links: Vec<Link>,
    pub subscription: Option<Subscription>,
    pub settings: Option<UserSettings>,
}

#[derive(Clone)]
pub struct Database {
    pub client: Client,
    pool: SqlitePool,
}

/// Get the platform-appropriate data directory for storing the database
fn get_data_dir() -> PathBuf {
    if let Some(data_dir) = dirs::data_local_dir() {
        data_dir.join("betternewtab")
    } else {
        // Fallback to current directory
        PathBuf::from(".")
    }
}

impl Database {
    pub async fn new(_database_url: String) -> Result<Self> {
        // Create data directory if it doesn't exist
        let data_dir = get_data_dir();
        std::fs::create_dir_all(&data_dir)?;

        let db_path = data_dir.join("data.db");
        let db_url = format!("sqlite:{}?mode=rwc", db_path.display());

        tracing::info!("Connecting to SQLite database at: {}", db_path.display());

        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(&db_url)
            .await?;

        // Run migrations
        sqlx::migrate!("./migrations_sqlite")
            .run(&pool)
            .await?;

        tracing::info!("Database migrations completed successfully");

        Ok(Self {
            client: Client::new(),
            pool,
        })
    }

    pub async fn get_user(&self, id: &str) -> Result<User> {
        tracing::info!("Fetching user by ID from database: {}", id);

        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .map(|mut user| {
                user.auth_token = None;
                user
            });

        match user {
            Some(user) => {
                tracing::info!("Successfully fetched user: {}", user.email);
                Ok(user)
            }
            None => {
                tracing::info!("User not found: {}", id);
                Err(anyhow::anyhow!("404"))
            }
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User> {
        tracing::info!("Fetching user by email: {}", email);

        let user = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = ?")
            .bind(email)
            .fetch_optional(&self.pool)
            .await?
            .map(|mut user| {
                user.auth_token = None;
                user
            });

        match user {
            Some(user) => {
                tracing::info!("Successfully fetched user by email");
                Ok(user)
            }
            None => {
                tracing::info!("User not found for email: {}", email);
                Err(anyhow::anyhow!("404"))
            }
        }
    }

    // Password authentication methods
    pub async fn register_user(&self, email: &str, password: &str) -> Result<User> {
        tracing::info!("Registering new user: {}", email);

        // Check if user already exists
        if self.get_user_by_email(email).await.is_ok() {
            tracing::warn!("User already exists: {}", email);
            return Err(anyhow::anyhow!("User already exists"));
        }

        // Hash password
        let password_hash = hash(password, DEFAULT_COST)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;

        // Generate user ID
        let user_id = format!("user_{}", uuid::Uuid::new_v4());
        let created_at = Utc::now().to_rfc3339();

        // Insert user
        let result = sqlx::query(
            "INSERT INTO users (id, email, password_hash, created_at) VALUES (?, ?, ?, ?)",
        )
        .bind(&user_id)
        .bind(email)
        .bind(&password_hash)
        .bind(&created_at)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            tracing::error!("Failed to create user: {}", email);
            return Err(anyhow::anyhow!("Failed to create user"));
        }

        tracing::info!("Successfully registered user: {}", email);

        Ok(User {
            id: user_id,
            email: email.to_string(),
            password_hash,
            created_at,
            auth_token: None,
        })
    }

    pub async fn verify_password(&self, email: &str, password: &str) -> Result<User> {
        tracing::info!("Verifying password for user: {}", email);

        let user = self.get_user_by_email(email).await?;

        let is_valid = verify(password, &user.password_hash)
            .map_err(|e| anyhow::anyhow!("Failed to verify password: {}", e))?;

        if !is_valid {
            tracing::warn!("Invalid password for user: {}", email);
            return Err(anyhow::anyhow!("Invalid credentials"));
        }

        tracing::info!("Successfully verified password for user: {}", email);
        Ok(user)
    }

    pub async fn create_user(&self, user: User) -> Result<User> {
        tracing::info!("Creating new user: {}", user.email);

        // Insert new user
        let result = sqlx::query(
            "INSERT INTO users (id, email, password_hash, created_at) VALUES (?, ?, ?, ?)",
        )
        .bind(&user.id)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(&user.created_at)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            tracing::error!("Failed to create user: {}", user.email);
            return Err(anyhow::anyhow!("Failed to create user: database error"));
        }

        tracing::info!("Successfully created user: {}", user.email);
        Ok(user)
    }

    pub async fn delete_user(&self, id: &str) -> Result<()> {
        tracing::info!("Deleting user: {}", id);

        let result = sqlx::query("DELETE FROM users WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            tracing::info!("No user found to delete with ID: {}", id);
            return Err(anyhow::anyhow!("User not found"));
        }

        tracing::info!("Successfully deleted user: {}", id);
        Ok(())
    }

    // Links
    pub async fn get_links(&self, owner_id: &str, owner_type: &str) -> Result<Vec<Link>> {
        tracing::info!("Fetching links for owner {}: {}", owner_type, owner_id);

        let links = sqlx::query_as::<_, Link>(
            "SELECT * FROM links WHERE owner_id = ? AND owner_type = ?",
        )
        .bind(owner_id)
        .bind(owner_type)
        .fetch_all(&self.pool)
        .await?;

        tracing::info!("Successfully fetched {} links", links.len());
        Ok(links)
    }

    pub async fn get_link(&self, id: &str, owner_id: &str) -> Result<Link> {
        tracing::info!("Fetching link: {} for owner: {}", id, owner_id);

        let link = sqlx::query_as::<_, Link>("SELECT * FROM links WHERE id = ? AND owner_id = ?")
            .bind(id)
            .bind(owner_id)
            .fetch_optional(&self.pool)
            .await?;

        match link {
            Some(link) => {
                tracing::info!("Successfully fetched link");
                Ok(link)
            }
            None => {
                tracing::info!("Link not found: {}", id);
                Err(anyhow::anyhow!("Link not found"))
            }
        }
    }

    pub async fn create_link(&self, link: Link) -> Result<Link> {
        tracing::info!(
            "Creating new link for owner {}: {}",
            link.owner_id,
            link.url
        );

        let result = sqlx::query(
            "INSERT INTO links (id, title, url, icon, order_index, owner_type, owner_id, created_at, description, column_type)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&link.id)
        .bind(&link.title)
        .bind(&link.url)
        .bind(&link.icon)
        .bind(&link.order_index)
        .bind(&link.owner_type)
        .bind(&link.owner_id)
        .bind(&link.created_at)
        .bind(&link.description)
        .bind(&link.column_type)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            tracing::error!("Failed to create link");
            return Err(anyhow::anyhow!("Failed to create link: database error"));
        }

        tracing::info!("Successfully created link: {}", link.id);
        Ok(link)
    }

    pub async fn update_link(&self, link: Link) -> Result<()> {
        tracing::info!("Updating link: {}", link.id);

        let result = sqlx::query(
            "UPDATE links
            SET title = ?, url = ?, icon = ?,
            order_index = ?, description = ?, column_type = ?
            WHERE id = ?",
        )
        .bind(&link.title)
        .bind(&link.url)
        .bind(&link.icon)
        .bind(&link.order_index)
        .bind(&link.description)
        .bind(&link.column_type)
        .bind(&link.id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Link not found or update failed"));
        }

        tracing::info!("Successfully updated link: {}", link.id);
        Ok(())
    }

    pub async fn delete_link(&self, id: &str) -> Result<()> {
        tracing::info!("Deleting link: {}", id);

        let result = sqlx::query("DELETE FROM links WHERE id = ?")
            .bind(id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            tracing::info!("No link found to delete with ID: {}", id);
            return Err(anyhow::anyhow!("Link not found"));
        }

        tracing::info!("Successfully deleted link: {}", id);
        Ok(())
    }

    // User Memberships
    pub async fn get_user_memberships(&self, user_id: &str) -> Result<Vec<UserMembership>> {
        tracing::info!("Fetching memberships for user: {}", user_id);

        let memberships = sqlx::query_as::<_, UserMembership>(
            "SELECT * FROM user_memberships WHERE user_id = ?",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        tracing::info!("Successfully fetched {} memberships", memberships.len());
        Ok(memberships)
    }

    pub async fn add_member(&self, membership: UserMembership) -> Result<()> {
        tracing::info!(
            "Adding member: {} to entity: {}",
            membership.user_id,
            membership.entity_id
        );

        let result = sqlx::query(
            "INSERT INTO user_memberships (user_id, entity_id, entity_type, role, created_at)
             VALUES (?, ?, ?, ?, ?)",
        )
        .bind(&membership.user_id)
        .bind(&membership.entity_id)
        .bind(&membership.entity_type)
        .bind(&membership.role)
        .bind(&membership.created_at)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            tracing::error!("Failed to add member");
            return Err(anyhow::anyhow!("Failed to add member: database error"));
        }

        tracing::info!("Successfully added member");
        Ok(())
    }

    pub async fn update_member_role(
        &self,
        user_id: &str,
        entity_id: &str,
        role: &str,
    ) -> Result<UserMembership> {
        tracing::info!(
            "Updating role for user: {} in entity: {}",
            user_id,
            entity_id
        );

        let result = sqlx::query(
            "UPDATE user_memberships SET role = ? WHERE user_id = ? AND entity_id = ?",
        )
        .bind(role)
        .bind(user_id)
        .bind(entity_id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Membership not found or update failed"));
        }

        // Fetch and return the updated membership
        let membership = sqlx::query_as::<_, UserMembership>(
            "SELECT * FROM user_memberships WHERE user_id = ? AND entity_id = ?",
        )
        .bind(user_id)
        .bind(entity_id)
        .fetch_one(&self.pool)
        .await?;

        tracing::info!("Successfully updated member role");
        Ok(membership)
    }

    pub async fn remove_member(&self, user_id: &str, entity_id: &str) -> Result<()> {
        tracing::info!("Removing member: {} from entity: {}", user_id, entity_id);

        let result =
            sqlx::query("DELETE FROM user_memberships WHERE user_id = ? AND entity_id = ?")
                .bind(user_id)
                .bind(entity_id)
                .execute(&self.pool)
                .await?;

        if result.rows_affected() == 0 {
            tracing::info!("No membership found to delete");
            return Err(anyhow::anyhow!("Membership not found"));
        }

        tracing::info!("Successfully removed member");
        Ok(())
    }

    // Plans
    pub async fn get_plans(&self) -> Result<Vec<Plan>> {
        tracing::info!("Fetching all plans");

        let plans = sqlx::query_as::<_, Plan>("SELECT * FROM plans")
            .fetch_all(&self.pool)
            .await?;

        tracing::info!("Successfully fetched {} plans", plans.len());
        Ok(plans)
    }

    pub async fn get_plan(&self, id: &str) -> Result<Plan> {
        tracing::info!("Fetching plan: {}", id);

        let plan = sqlx::query_as::<_, Plan>("SELECT * FROM plans WHERE id = ?")
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;

        match plan {
            Some(plan) => {
                tracing::info!("Successfully fetched plan");
                Ok(plan)
            }
            None => {
                tracing::info!("Plan not found: {}", id);
                Err(anyhow::anyhow!("404"))
            }
        }
    }

    // Subscriptions
    pub async fn get_user_subscription(&self, user_id: &str) -> Result<Subscription> {
        tracing::info!("Fetching subscription for user: {}", user_id);

        let subscription = sqlx::query_as::<_, Subscription>(
            "SELECT * FROM subscriptions WHERE entity_id = ? AND entity_type = 'user'",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;

        match subscription {
            Some(subscription) => {
                tracing::info!("Successfully fetched subscription for user");
                println!("Successfully fetched subscription for user");
                Ok(subscription)
            }
            None => {
                tracing::info!("No subscription found for user: {}", user_id);
                println!("No subscription found for user: {}", user_id);
                Err(anyhow::anyhow!("404"))
            }
        }
    }

    pub async fn create_subscription(
        &self,
        entity_id: &str,
        entity_type: &str,
        plan_id: &str,
        status: &str,
        current_period_end: DateTime<Utc>,
    ) -> Result<Subscription> {
        let sub_uuid = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now().to_rfc3339();
        let period_end = current_period_end.to_rfc3339();

        let subscription = Subscription {
            id: sub_uuid.clone(),
            entity_id: entity_id.to_string(),
            entity_type: entity_type.to_string(),
            plan_id: plan_id.to_string(),
            status: status.to_string(),
            current_period_end: period_end.clone(),
            created_at: created_at.clone(),
        };

        tracing::info!("Creating subscription with payload: {:?}", subscription);

        let result = sqlx::query(
            "INSERT INTO subscriptions (id, entity_id, entity_type, plan_id, status, current_period_end, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(&sub_uuid)
        .bind(entity_id)
        .bind(entity_type)
        .bind(plan_id)
        .bind(status)
        .bind(&period_end)
        .bind(&created_at)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            tracing::error!("Failed to create subscription");
            return Err(anyhow::anyhow!(
                "Failed to create subscription: database error"
            ));
        }

        tracing::info!("Successfully created subscription: {}", subscription.id);
        Ok(subscription)
    }

    pub async fn update_subscription(&self, subscription: Subscription) -> Result<()> {
        tracing::info!("Updating subscription: {}", subscription.id);

        let result = sqlx::query(
            "UPDATE subscriptions
            SET entity_id = ?, entity_type = ?, plan_id = ?,
            status = ?, current_period_end = ?
            WHERE id = ?",
        )
        .bind(&subscription.entity_id)
        .bind(&subscription.entity_type)
        .bind(&subscription.plan_id)
        .bind(&subscription.status)
        .bind(&subscription.current_period_end)
        .bind(&subscription.id)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(anyhow::anyhow!("Subscription not found or update failed"));
        }

        tracing::info!("Successfully updated subscription: {}", subscription.id);

        Ok(())
    }

    pub async fn create_feedback_timestamp(
        &self,
        user_id: &str,
        created_at: &DateTime<Utc>,
    ) -> Result<()> {
        tracing::info!("Creating feedback timestamp for user: {}", user_id);

        let created_at_str = created_at.to_rfc3339();

        let result =
            sqlx::query("INSERT INTO feedback_timestamps (user_id, created_at) VALUES (?, ?)")
                .bind(user_id)
                .bind(&created_at_str)
                .execute(&self.pool)
                .await?;

        if result.rows_affected() == 0 {
            tracing::error!("Failed to create feedback timestamp");
            return Err(anyhow::anyhow!(
                "Failed to create feedback timestamp: database error"
            ));
        }

        tracing::info!("Successfully created feedback timestamp");
        Ok(())
    }

    pub async fn check_feedback_timestamp(&self, user_id: &str) -> Result<bool> {
        tracing::info!("Checking feedback timestamp for user: {}", user_id);

        let timestamp =
            sqlx::query("SELECT created_at FROM feedback_timestamps WHERE user_id = ?")
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await?;

        if let Some(record) = timestamp {
            let created_at: String = record.try_get("created_at")?;
            tracing::info!("Found feedback timestamp: {}", created_at);

            let timestamp = chrono::DateTime::parse_from_rfc3339(&created_at)?;
            if chrono::Utc::now().signed_duration_since(timestamp) < chrono::Duration::hours(24) {
                return Ok(false);
            }

            // Delete the record if it's older than 24 hours
            let result = sqlx::query("DELETE FROM feedback_timestamps WHERE user_id = ?")
                .bind(user_id)
                .execute(&self.pool)
                .await?;

            tracing::info!(
                "Deleted old feedback timestamp, rows affected: {}",
                result.rows_affected()
            );
        }

        Ok(true)
    }

    // User Settings
    pub async fn get_user_settings(&self, user_id: &str) -> Result<UserSettings> {
        tracing::info!("Fetching settings for user: {}", user_id);

        let settings =
            sqlx::query_as::<_, UserSettings>("SELECT * FROM user_settings WHERE user_id = ?")
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await?;

        match settings {
            Some(settings) => {
                tracing::info!("Successfully fetched user settings");
                Ok(settings)
            }
            None => {
                tracing::info!("No settings found for user: {}", user_id);
                Err(anyhow::anyhow!("404"))
            }
        }
    }

    pub async fn create_user_settings(&self, user_settings: UserSettings) -> Result<UserSettings> {
        tracing::info!("Creating settings for user: {}", user_settings.user_id);

        let result = sqlx::query(
            "INSERT INTO user_settings (user_id, settings_blob, created_at) VALUES (?, ?, ?)",
        )
        .bind(&user_settings.user_id)
        .bind(&user_settings.settings_blob)
        .bind(&user_settings.created_at)
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            tracing::error!("Failed to create user settings");
            return Err(anyhow::anyhow!("500"));
        }

        tracing::info!("Successfully created user settings");
        Ok(user_settings)
    }

    pub async fn update_user_settings(
        &self,
        user_id: &str,
        updates: HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        tracing::info!("Updating settings for user: {}", user_id);

        // Since user_settings typically only has one JSON blob column, simplify the update
        if let Some(settings_blob) = updates.get("settings_blob") {
            let settings_str = settings_blob.to_string();
            let result =
                sqlx::query("UPDATE user_settings SET settings_blob = ? WHERE user_id = ?")
                    .bind(&settings_str)
                    .bind(user_id)
                    .execute(&self.pool)
                    .await?;

            if result.rows_affected() == 0 {
                return Err(anyhow::anyhow!("User settings not found or update failed"));
            }
        } else {
            return Err(anyhow::anyhow!("No settings_blob provided for update"));
        }

        tracing::info!("Successfully updated user settings");
        Ok(())
    }

    pub async fn delete_user_settings(&self, user_id: &str) -> Result<()> {
        tracing::info!("Deleting settings for user: {}", user_id);

        let result = sqlx::query("DELETE FROM user_settings WHERE user_id = ?")
            .bind(user_id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            tracing::info!("No user settings found to delete");
            return Err(anyhow::anyhow!("User settings not found"));
        }

        tracing::info!("Successfully deleted user settings");
        Ok(())
    }

    pub async fn get_user_data(&self, user_id: &str) -> Result<UserData> {
        let rows = sqlx::query(
            "
        SELECT
            u.*,
            l.id as link_id,
            l.title as link_title,
            l.url as link_url,
            l.icon as link_icon,
            l.order_index as link_order_index,
            l.owner_type as link_owner_type,
            l.owner_id as link_owner_id,
            l.created_at as link_created_at,
            l.description as link_description,
            l.column_type as link_column_type,
            s.id as subscription_id,
            s.entity_id as subscription_entity_id,
            s.entity_type as subscription_entity_type,
            s.plan_id as subscription_plan_id,
            s.status as subscription_status,
            s.current_period_end as subscription_current_period_end,
            s.created_at as subscription_created_at,
            us.settings_blob as settings_blob,
            us.created_at as settings_created_at
            FROM users u
            LEFT JOIN links l ON u.id = l.owner_id
            LEFT JOIN subscriptions s ON u.id = s.entity_id
            LEFT JOIN user_settings us ON u.id = us.user_id
            WHERE u.id = ?
            ORDER BY l.order_index ASC;",
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;

        if rows.is_empty() {
            return Err(anyhow::anyhow!("User not found"));
        }

        let first_row = &rows[0];
        let user = User {
            id: first_row.get("id"),
            email: first_row.get("email"),
            created_at: first_row.get("created_at"),
            auth_token: None,
            password_hash: first_row.get::<String, _>("password_hash"),
        };

        let settings_blob: Option<String> = first_row.try_get("settings_blob").ok();
        let settings_created_at: Option<String> = first_row.try_get("settings_created_at").ok();
        let settings = if let (Some(blob), Some(created)) = (settings_blob, settings_created_at) {
            Some(UserSettings {
                user_id: first_row.get("id"),
                settings_blob: blob,
                created_at: created,
            })
        } else {
            None
        };

        let subscription = if let Ok(sub_id) = first_row.try_get::<String, _>("subscription_id") {
            Some(Subscription {
                id: sub_id,
                entity_id: first_row
                    .try_get::<String, _>("subscription_entity_id")
                    .unwrap_or_default(),
                entity_type: first_row
                    .try_get::<String, _>("subscription_entity_type")
                    .unwrap_or_default(),
                plan_id: first_row
                    .try_get::<String, _>("subscription_plan_id")
                    .unwrap_or_default(),
                status: first_row
                    .try_get::<String, _>("subscription_status")
                    .unwrap_or_default(),
                current_period_end: first_row
                    .try_get::<String, _>("subscription_current_period_end")
                    .unwrap_or_default(),
                created_at: first_row
                    .try_get::<String, _>("subscription_created_at")
                    .unwrap_or_default(),
            })
        } else {
            None
        };

        let links = rows
            .iter()
            .filter_map(|row| {
                if let Ok(link_id) = row.try_get::<String, _>("link_id") {
                    Some(Link {
                        id: link_id,
                        title: row.try_get("link_title").unwrap_or_default(),
                        url: row.try_get("link_url").unwrap_or_default(),
                        icon: row.try_get("link_icon").ok(),
                        order_index: row.try_get("link_order_index").unwrap_or(0),
                        owner_type: row.try_get("link_owner_type").unwrap_or_default(),
                        owner_id: row.try_get("link_owner_id").unwrap_or_default(),
                        created_at: row.try_get("link_created_at").unwrap_or_default(),
                        description: row.try_get("link_description").ok(),
                        column_type: row.try_get("link_column_type").unwrap_or_default(),
                    })
                } else {
                    None
                }
            })
            .collect::<Vec<Link>>();

        Ok(UserData {
            user,
            links,
            subscription,
            settings,
        })
    }

    // Organization and Team functions (reimplemented from PL/pgSQL)
    pub async fn create_organization(
        &self,
        org_name: &str,
        owner_id: &str,
        plan_id: &str,
    ) -> Result<String> {
        tracing::info!("Creating organization: {} for owner: {}", org_name, owner_id);

        // Start a transaction
        let mut tx = self.pool.begin().await?;

        let org_id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now().to_rfc3339();

        // Create the organization
        sqlx::query("INSERT INTO organizations (id, name, owner_id, created_at) VALUES (?, ?, ?, ?)")
            .bind(&org_id)
            .bind(org_name)
            .bind(owner_id)
            .bind(&created_at)
            .execute(&mut *tx)
            .await?;

        // Create subscription for the organization
        let sub_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO subscriptions (id, entity_id, entity_type, plan_id, status, created_at) VALUES (?, ?, 'organization', ?, 'active', ?)"
        )
        .bind(&sub_id)
        .bind(&org_id)
        .bind(plan_id)
        .bind(&created_at)
        .execute(&mut *tx)
        .await?;

        // Add owner as admin member
        sqlx::query(
            "INSERT INTO user_memberships (user_id, entity_id, entity_type, role, created_at) VALUES (?, ?, 'organization', 'admin', ?)"
        )
        .bind(owner_id)
        .bind(&org_id)
        .bind(&created_at)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        tracing::info!("Successfully created organization: {}", org_id);
        Ok(org_id)
    }

    pub async fn create_team(
        &self,
        team_name: &str,
        owner_id: &str,
        plan_id: &str,
        org_id: Option<&str>,
    ) -> Result<String> {
        tracing::info!("Creating team: {} for owner: {}", team_name, owner_id);

        // Start a transaction
        let mut tx = self.pool.begin().await?;

        let team_id = uuid::Uuid::new_v4().to_string();
        let created_at = Utc::now().to_rfc3339();

        // Create the team
        sqlx::query("INSERT INTO teams (id, name, owner_id, organization_id, created_at) VALUES (?, ?, ?, ?, ?)")
            .bind(&team_id)
            .bind(team_name)
            .bind(owner_id)
            .bind(org_id)
            .bind(&created_at)
            .execute(&mut *tx)
            .await?;

        // Create subscription for the team
        let sub_id = uuid::Uuid::new_v4().to_string();
        sqlx::query(
            "INSERT INTO subscriptions (id, entity_id, entity_type, plan_id, status, created_at) VALUES (?, ?, 'team', ?, 'active', ?)"
        )
        .bind(&sub_id)
        .bind(&team_id)
        .bind(plan_id)
        .bind(&created_at)
        .execute(&mut *tx)
        .await?;

        // Add owner as admin member
        sqlx::query(
            "INSERT INTO user_memberships (user_id, entity_id, entity_type, role, created_at) VALUES (?, ?, 'team', 'admin', ?)"
        )
        .bind(owner_id)
        .bind(&team_id)
        .bind(&created_at)
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;

        tracing::info!("Successfully created team: {}", team_id);
        Ok(team_id)
    }
}
