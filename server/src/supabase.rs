use anyhow::Result;
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use stripe::generated::billing::subscription;
use std::collections::HashMap;
use sqlx::postgres::PgPoolOptions;
use sqlx::postgres::PgPool;
use sqlx::FromRow;
use chrono::{DateTime, Utc};
use sqlx::Row;

// Type definitions matching Database.ts
#[derive(Serialize, Deserialize, Debug, Clone, FromRow)]
pub struct User {
    pub id: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    #[sqlx(skip)]
    pub auth_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Link {
    #[sqlx(try_from = "sqlx::types::Uuid")]
    pub id: String,
    pub title: String,
    pub url: String,
    pub icon: Option<String>,
    pub order_index: i32,
    pub owner_type: String,
    pub owner_id: String,
    pub created_at: DateTime<Utc>,
    pub description: Option<String>,
    pub column_type: String,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct Plan {    
    #[sqlx(try_from = "sqlx::types::Uuid")]
    pub id: String,
    pub name: String,
    pub max_pins: i32,
    pub features: serde_json::Value,
    pub created_at: DateTime<Utc>,
    pub stripe_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default, Clone, FromRow)]
pub struct Subscription {
    #[sqlx(try_from = "sqlx::types::Uuid")]
    pub id: String,
    pub entity_id: String,
    pub entity_type: String,
    #[sqlx(try_from = "sqlx::types::Uuid")]
    pub plan_id: String,
    pub status: String,
    pub stripe_subscription_id: String,
    pub current_period_end: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct UserMembership {
    pub user_id: String,
    pub entity_id: String,
    pub entity_type: String,
    pub role: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct UserSettings {
    pub user_id: String,
    pub settings_blob: serde_json::Value,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserData {
    pub user: User,
    pub links: Vec<Link>,
    pub subscription: Option<Subscription>,
    pub settings: Option<UserSettings>,
}

#[derive(Clone)]
pub struct Supabase {
    pub url: String,
    pub api_key: String,
    pub client: Client,
    pool: PgPool,
}

#[allow(dead_code)]
impl Supabase {
    pub async fn new(postgres_url: String,) -> Result<Self> {

        let pool = PgPoolOptions::new()
        .max_connections(50)
        .connect(&postgres_url).await?;

        Ok(Self {
            url: "asd".to_string(),
            api_key: "ase".to_string(),
            client: Client::new(),
            pool,
        })

    }

    fn build_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert("apikey", self.api_key.parse()?);
        headers.insert("Authorization", format!("Bearer {}", self.api_key).parse()?);
        Ok(headers)
    }

    pub async fn get_user(&self, id: &str) -> Result<User> {
        tracing::info!("Fetching user by ID from database: {}", id);
        println!("Fetching user by ID from database: {}", id);
        
        let user = sqlx::query_as::<_, User>("SELECT * FROM USERS WHERE id = $1")
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
            },
            None => {
                tracing::info!("User not found: {}", id);
                Err(anyhow::anyhow!("404"))
            }
        }
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User> {
        tracing::info!("Fetching user by email: {}", email);
        
        let user = sqlx::query_as::<_, User>("SELECT * FROM USERS WHERE email = $1")
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
            },
            None => {
                tracing::info!("User not found for email: {}", email);
                Err(anyhow::anyhow!("404"))
            }
        }
    }

    pub async fn get_plan_by_stripe_id(&self, stripe_id: &str) -> Result<Plan> {
        tracing::info!("Fetching plan by stripe ID: {}", stripe_id);
        
        let plan = sqlx::query_as::<_, Plan>("SELECT * FROM plans WHERE stripe_id = $1")
            .bind(stripe_id)
            .fetch_optional(&self.pool)
            .await?;

        match plan {
            Some(plan) => {
                tracing::info!("Successfully fetched plan by stripe ID");
                println!("Successfully fetched plan by stripe ID");
                Ok(plan)
            },
            None => {
                tracing::info!("Plan not found for stripe ID: {}", stripe_id);
                println!("Plan not found for stripe ID: {}", stripe_id);
                Err(anyhow::anyhow!("Plan not found"))
            }
        }
    }

    pub async fn create_user(&self, user: User) -> Result<User> {
        tracing::info!("Creating new user: {}", user.email);
        println!("Creating new user: {}", user.email);    
        println!("User id: {}", user.id);
        println!("User created_at: {}", user.created_at);
        
        // Insert new user
        let result = sqlx::query(
            "INSERT INTO USERS (id, email, created_at) VALUES ($1, $2, $3)"
        )
        .bind(&user.id)
        .bind(&user.email)
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
        
        let result = sqlx::query("DELETE FROM USERS WHERE id = $1")
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
            "SELECT * FROM links WHERE owner_id = $1 AND owner_type = $2"
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
        
        let link = sqlx::query_as::<_, Link>(
            "SELECT * FROM links WHERE id = $1 AND owner_id = $2"
        )
        .bind(id)
        .bind(owner_id)
        .fetch_optional(&self.pool)
        .await?;
        
        match link {
            Some(link) => {
                tracing::info!("Successfully fetched link");
                Ok(link)
            },
            None => {
                tracing::info!("Link not found: {}", id);
                Err(anyhow::anyhow!("Link not found"))
            }
        }
    }

    pub async fn create_link(&self, link: Link) -> Result<Link> {
        tracing::info!("Creating new link for owner {}: {}", link.owner_id, link.url);

        let result = sqlx::query(
            "INSERT INTO links (id, title, url, icon, order_index, owner_type, owner_id, created_at, description, column_type) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)"
        )
        .bind(uuid::Uuid::parse_str(&link.id).expect("Invalid UUID format"))
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

    pub async fn update_link(
        &self,
        link: Link,
    ) -> Result<()> {
        tracing::info!("Updating link: {}", link.id);
        
        let result = sqlx::query(
            "UPDATE links 
            SET title = $1, url = $2, icon = $3, 
            order_index = $4, description = $5, column_type = $6 
            WHERE id = $7"
        )
        .bind(&link.title)
        .bind(&link.url)
        .bind(&link.icon)
        .bind(&link.order_index)
        .bind(&link.description)
        .bind(&link.column_type)
        .bind(uuid::Uuid::parse_str(&link.id).expect("Invalid UUID format"))
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
        
        let result = sqlx::query("DELETE FROM links WHERE id = $1")
            .bind(uuid::Uuid::parse_str(&id).expect("Invalid UUID format"))
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
            "SELECT * FROM user_memberships WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_all(&self.pool)
        .await?;
        
        tracing::info!("Successfully fetched {} memberships", memberships.len());
        Ok(memberships)
    }

    pub async fn add_member(&self, membership: UserMembership) -> Result<()> {
        tracing::info!("Adding member: {} to entity: {}", membership.user_id, membership.entity_id);
        
        let result = sqlx::query(
            "INSERT INTO user_memberships (user_id, entity_id, entity_type, role, created_at) 
             VALUES ($1, $2, $3, $4, $5)"
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
        tracing::info!("Updating role for user: {} in entity: {}", user_id, entity_id);
        
        let result = sqlx::query(
            "UPDATE user_memberships SET role = $1 WHERE user_id = $2 AND entity_id = $3"
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
            "SELECT * FROM user_memberships WHERE user_id = $1 AND entity_id = $2"
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
        
        let result = sqlx::query(
            "DELETE FROM user_memberships WHERE user_id = $1 AND entity_id = $2"
        )
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
        
        let plan = sqlx::query_as::<_, Plan>("SELECT * FROM plans WHERE id = $1")
            .bind(uuid::Uuid::parse_str(id).expect("Invalid UUID format"))
            .fetch_optional(&self.pool)
            .await?;
            
        match plan {
            Some(plan) => {
                tracing::info!("Successfully fetched plan");
                Ok(plan)
            },
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
            "SELECT * FROM subscriptions WHERE entity_id = $1 AND entity_type = 'user'"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;
        
        match subscription {
            Some(subscription) => {
                tracing::info!("Successfully fetched subscription for user");
                println!("Successfully fetched subscription for user");
                Ok(subscription)
            },
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
        stripe_subscription_id: &str,
        current_period_end: DateTime<Utc>,
    ) -> Result<Subscription> {
        let sub_uuid = uuid::Uuid::new_v4();
        let subscription: Subscription = Subscription {
            id: sub_uuid.to_string(),
            entity_id: entity_id.to_string(),
            entity_type: entity_type.to_string(),
            plan_id: plan_id.to_string(),
            status: status.to_string(),
            stripe_subscription_id: stripe_subscription_id.to_string(),
            current_period_end: current_period_end,
            created_at: chrono::Utc::now(),
        };

        tracing::info!("Creating subscription with payload: {:?}", subscription);
        
        let result = sqlx::query(
            "INSERT INTO subscriptions (id, entity_id, entity_type, plan_id, status, stripe_subscription_id, current_period_end, created_at) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
        )
        .bind(&sub_uuid)
        .bind(&subscription.entity_id)
        .bind(&subscription.entity_type)
        .bind(uuid::Uuid::parse_str(&subscription.plan_id).expect("Invalid UUID format"))
        .bind(&subscription.status)
        .bind(&subscription.stripe_subscription_id)
        .bind(&subscription.current_period_end)
        .bind(&subscription.created_at)
        .execute(&self.pool)
        .await?;
        
        if result.rows_affected() == 0 {
            tracing::error!("Failed to create subscription");
            return Err(anyhow::anyhow!("Failed to create subscription: database error"));
        }
        
        tracing::info!("Successfully created subscription: {}", subscription.id);
        Ok(subscription)
    }

    pub async fn update_subscription(
        &self,
        subscription: Subscription,
    ) -> Result<()> {
        tracing::info!("Updating subscription: {}", subscription.id);
        
        let result = sqlx::query(
            "UPDATE subscriptions 
            SET entity_id = $1, entity_type = $2, plan_id = $3, 
            status = $4, stripe_subscription_id = $5, current_period_end = $6 
            WHERE id = $7"
        )
        .bind(&subscription.entity_id)
        .bind(&subscription.entity_type)
        .bind(&subscription.plan_id)
        .bind(&subscription.status)
        .bind(&subscription.stripe_subscription_id)
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
        
        let result = sqlx::query(
            "INSERT INTO feedback_timestamps (user_id, created_at) VALUES ($1, $2)"
        )
        .bind(user_id)
        .bind(created_at)
        .execute(&self.pool)
        .await?;
        
        if result.rows_affected() == 0 {
            tracing::error!("Failed to create feedback timestamp");
            return Err(anyhow::anyhow!("Failed to create feedback timestamp: database error"));
        }
        
        tracing::info!("Successfully created feedback timestamp");
        Ok(())
    }

    pub async fn check_feedback_timestamp(
        &self,
        user_id: &str
    ) -> Result<bool> {
        tracing::info!("Checking feedback timestamp for user: {}", user_id);
        
        let timestamp = sqlx::query(
            "SELECT created_at FROM feedback_timestamps WHERE user_id = $1",
        )
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
            let result = sqlx::query("DELETE FROM feedback_timestamps WHERE user_id = $1")
                .bind(user_id)
                .execute(&self.pool)
                .await?;
                
            tracing::info!("Deleted old feedback timestamp, rows affected: {}", result.rows_affected());
        }
        
        Ok(true)
    }

    // User Settings
    pub async fn get_user_settings(&self, user_id: &str) -> Result<UserSettings> {
        tracing::info!("Fetching settings for user: {}", user_id);
        
        let settings = sqlx::query_as::<_, UserSettings>(
            "SELECT * FROM user_settings WHERE user_id = $1"
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await?;
        
        match settings {
            Some(settings) => {
                tracing::info!("Successfully fetched user settings");
                Ok(settings)
            },
            None => {
                tracing::info!("No settings found for user: {}", user_id);
                Err(anyhow::anyhow!("404"))
            }
        }
    }

    pub async fn create_user_settings(&self, user_settings: UserSettings) -> Result<UserSettings> {
        tracing::info!("Creating settings for user: {}", user_settings.user_id);
        
        let result = sqlx::query(
            "INSERT INTO user_settings (user_id, settings_blob, created_at) VALUES ($1, $2, $3)"
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
            let result = sqlx::query(
                "UPDATE user_settings SET settings_blob = $1 WHERE user_id = $2"
            )
            .bind(settings_blob)
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
        
        let result = sqlx::query("DELETE FROM user_settings WHERE user_id = $1")
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
        let rows = sqlx::query("
        select
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
            s.stripe_subscription_id as subscription_stripe_subscription_id,
            s.current_period_end as subscription_current_period_end,
            s.created_at as subscription_created_at,
            us.settings_blob as settings_blob,
            us.created_at as settings_created_at            
            from users u
            left join links l on u.id = l.owner_id
            left join subscriptions s on u.id = s.entity_id
            left join user_settings us on u.id = us.user_id
            where u.id = $1
            order by l.order_index asc;"
        ).bind(user_id)
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
        };
        let settings_blob: serde_json::Value = first_row.get("settings_blob");
        let settings = Some(UserSettings {
            user_id: first_row.get("id"),
            settings_blob,
            created_at: first_row.get("settings_created_at"),
        });
        let subscription = if let Ok(sub_id) = first_row.try_get::<uuid::Uuid, _>("subscription_id") {
                Some(Subscription {
                    id: sub_id.to_string(),
                    entity_id: first_row
                        .get::<Option<String>, _>("subscription_entity_id")
                        .unwrap_or_default(),
                    entity_type: first_row
                        .get::<Option<String>, _>("subscription_entity_type")
                        .unwrap_or_default(),
                    plan_id: first_row.get::<uuid::Uuid, _>("subscription_plan_id").to_string(),
                    status: first_row.get("subscription_status"),
                    stripe_subscription_id: first_row.get("subscription_stripe_subscription_id"),
                    current_period_end: first_row.get("subscription_current_period_end"),
                    created_at: first_row.get("subscription_created_at"),
                })
        } else {
            None
        };
        let links = rows
            .iter()
            .filter_map(|row| {
                if let Ok(link_id) = row.try_get::<uuid::Uuid, _>("link_id") {
                    Some(Link {
                        id: link_id.to_string(),
                        title: row.get("link_title"),
                        url: row.get("link_url"),
                        icon: row.get("link_icon"),
                        order_index: row.get("link_order_index"),
                        owner_type: row.get("link_owner_type"),
                        owner_id: row.get("link_owner_id"),
                        created_at: row.get("link_created_at"),
                        description: row.get("link_description"),
                        column_type: row.get("link_column_type"),
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
}
