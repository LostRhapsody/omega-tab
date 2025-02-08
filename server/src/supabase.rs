use anyhow::Result;
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Type definitions matching Database.ts
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Team {
    pub id: String,
    pub name: String,
    pub organization_id: Option<String>,
    pub created_at: String,
    pub owner_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub owner_id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Plan {
    pub id: String,
    pub name: String,
    pub max_pins: i32,
    pub features: serde_json::Value,
    pub created_at: Option<String>,
    pub stripe_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Subscription {
    pub id: String,
    pub entity_id: String,
    pub entity_type: String,
    pub plan_id: String,
    pub status: String,
    pub stripe_subscription_id: String,
    pub current_period_end: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMembership {
    pub user_id: String,
    pub entity_id: String,
    pub entity_type: String,
    pub role: String,
    pub created_at: String,
}

pub struct Supabase {
    client: Client,
    url: String,
    api_key: String,
}

#[allow(dead_code)]
impl Supabase {
    pub fn new(url: String, api_key: String) -> Result<Self> {
        let client = Client::new();
        Ok(Self {
            client,
            url,
            api_key,
        })
    }

    fn build_headers(&self) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert("apikey", self.api_key.parse()?);
        headers.insert("Authorization", format!("Bearer {}", self.api_key).parse()?);
        Ok(headers)
    }

    // Users
    pub async fn get_users(&self) -> Result<Vec<User>> {
        let response = self
            .client
            .get(format!("{}/rest/v1/users", self.url))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn get_user(&self, id: &str) -> Result<User> {
        let response = self
            .client
            .get(format!("{}/rest/v1/users?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        let mut users: Vec<User> = response.json().await?;
        users.pop().ok_or_else(|| anyhow::anyhow!("404"))
    }

    pub async fn get_plan_by_stripe_id(&self, stripe_id: &str) -> Result<Plan> {
        let response = self
            .client
            .get(format!(
                "{}/rest/v1/plans?stripe_id=eq.{}",
                self.url, stripe_id
            ))
            .headers(self.build_headers()?)
            .send()
            .await?;

        let mut plans: Vec<Plan> = response.json().await?;
        plans.pop().ok_or_else(|| anyhow::anyhow!("Plan not found"))
    }

    pub async fn create_user(&self, user:User) -> Result<User> {
        // TODO - I think if the user exists in supabase already we get a good
        // error back, we can just check that and handle it correctly, not
        // send a second request
        // Check if user exists
        let existing_user = self.get_user(&user.id).await;
        if existing_user.is_ok() {
            return Err(anyhow::anyhow!("409"));
        }

        println!("Creating user with payload: {:?}", user);

        let response = self
            .client
            .post(format!("{}/rest/v1/users", self.url))
            .headers(self.build_headers()?)
            .json(&user)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to create user: {}", error_text));
        }

        Ok(user)
    }

    pub async fn update_user(
        &self,
        id: &str,
        updates: HashMap<String, serde_json::Value>,
    ) -> Result<User> {
        let response = self
            .client
            .patch(format!("{}/rest/v1/users?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .json(&updates)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn delete_user(&self, id: &str) -> Result<()> {
        self.client
            .delete(format!("{}/rest/v1/users?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(())
    }

    // Teams
    pub async fn get_teams(&self) -> Result<Vec<Team>> {
        let response = self
            .client
            .get(format!("{}/rest/v1/teams", self.url))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn get_team(&self, id: &str) -> Result<Team> {
        let response = self
            .client
            .get(format!("{}/rest/v1/teams?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        let mut teams: Vec<Team> = response.json().await?;
        teams.pop().ok_or_else(|| anyhow::anyhow!("Team not found"))
    }

    pub async fn create_team(
        &self,
        name: &str,
        owner_id: &str,
        plan_id: &str,
        org_id: Option<&str>,
    ) -> Result<Team> {
        let new_team: Team = Team {
            id: uuid::Uuid::new_v4().to_string(),
            name: name.to_string(),
            owner_id: owner_id.to_string(),
            organization_id: org_id.map(|id| id.to_string()),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        let plan_id = plan_id.to_string();

        let mut params = vec![
            ("team_name", &new_team.name),
            ("owner_id", &new_team.owner_id),
            ("plan_id", &plan_id),
            ("created_at", &new_team.created_at),
        ];

        if let Some(_org_id) = org_id {
            params.push((
                "organization_id",
                new_team.organization_id.as_ref().unwrap(),
            ));
        }

        let response = self
            .client
            .post(format!("{}/rest/v1/rpc/create_team", self.url))
            .headers(self.build_headers()?)
            .form(&params)
            .send()
            .await?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!(response.text().await?));
        }

        // Return local version of created team
        Ok(new_team)
    }

    pub async fn update_team(
        &self,
        id: &str,
        updates: HashMap<String, serde_json::Value>,
    ) -> Result<Team> {
        let response = self
            .client
            .patch(format!("{}/rest/v1/teams?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .json(&updates)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn delete_team(&self, id: &str) -> Result<()> {
        self.client
            .delete(format!("{}/rest/v1/teams?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(())
    }

    // Links
    pub async fn get_links(&self, owner_id: &str, owner_type: &str) -> Result<Vec<Link>> {
        println!("get_links");
        println!("owner_id: {:?}", owner_id);
        println!("owner_type: {:?}", owner_type);
        let response = self
            .client
            .get(format!(
                "{}/rest/v1/links?owner_id=eq.{}&owner_type=eq.{}",
                self.url, owner_id, owner_type
            ))
            .headers(self.build_headers()?)
            .send()
            .await?;

        if response.status().is_success() {
            let links: Vec<Link> = response.json().await?;
            Ok(links)
        } else {
            Err(anyhow::anyhow!("500"))
        }
    }

    pub async fn get_link(&self, id: &str, owner_id: &str) -> Result<Link> {
        let response = self
            .client
            .get(format!(
                "{}/rest/v1/links?id=eq.{}&owner_id=eq.{}",
                self.url, id, owner_id
            ))
            .headers(self.build_headers()?)
            .send()
            .await?;

        let mut links: Vec<Link> = response.json().await?;
        links.pop().ok_or_else(|| anyhow::anyhow!("Link not found"))
    }

    pub async fn create_link(&self,link:Link) -> Result<Link> {

        println!("Creating link with payload: {:?}", link);

        let response = self
            .client
            .post(format!("{}/rest/v1/links", self.url))
            .headers(self.build_headers()?)
            .json(&link)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to create link: {}", error_text));
        }

        Ok(link)
    }

    pub async fn update_link(
        &self,
        id: &str,
        updates: HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        let response = self
            .client
            .patch(format!("{}/rest/v1/links?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .json(&updates)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to create link: {}", error_text));
        }

        Ok(())
    }

    pub async fn delete_link(&self, id: &str) -> Result<()> {
        self.client
            .delete(format!("{}/rest/v1/links?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(())
    }

    // Organizations
    pub async fn get_organizations(&self) -> Result<Vec<Organization>> {
        let response = self
            .client
            .get(format!("{}/rest/v1/organizations", self.url))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn create_organization(
        &self,
        org_name: &str,
        owner_id: &str,
    ) -> Result<Organization> {
        let organization: Organization = Organization {
            id: uuid::Uuid::new_v4().to_string(),
            name: org_name.to_string(),
            owner_id: owner_id.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        println!("Creating organization with payload: {:?}", organization);

        let response = self
            .client
            .post(format!("{}/rest/v1/organizations", self.url))
            .headers(self.build_headers()?)
            .json(&organization)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to create organization: {}", error_text));
        }

        Ok(organization)
    }

    pub async fn update_organization(
        &self,
        id: &str,
        updates: HashMap<String, serde_json::Value>,
    ) -> Result<Organization> {
        let response = self
            .client
            .patch(format!("{}/rest/v1/organizations?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .json(&updates)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn delete_organization(&self, id: &str) -> Result<()> {
        self.client
            .delete(format!("{}/rest/v1/organizations?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(())
    }

    // User Memberships
    pub async fn get_user_memberships(&self, user_id: &str) -> Result<Vec<UserMembership>> {
        let response = self
            .client
            .get(format!(
                "{}/rest/v1/user_memberships?user_id=eq.{}",
                self.url, user_id
            ))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn add_member(&self, membership: UserMembership) -> Result<()> {
        let response = self
            .client
            .post(format!("{}/rest/v1/user_memberships", self.url))
            .headers(self.build_headers()?)
            .json(&membership)
            .send()
            .await?;

        // Check if the response status is successful
        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to add member: {}", error_text));
        }

        Ok(())
    }

    pub async fn update_member_role(
        &self,
        user_id: &str,
        entity_id: &str,
        role: &str,
    ) -> Result<UserMembership> {
        let updates = HashMap::from([(
            "role".to_string(),
            serde_json::Value::String(role.to_string()),
        )]);

        let response = self
            .client
            .patch(format!(
                "{}/rest/v1/user_memberships?user_id=eq.{}&entity_id=eq.{}",
                self.url, user_id, entity_id
            ))
            .headers(self.build_headers()?)
            .json(&updates)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn remove_member(&self, user_id: &str, entity_id: &str) -> Result<()> {
        self.client
            .delete(format!(
                "{}/rest/v1/user_memberships?user_id=eq.{}&entity_id=eq.{}",
                self.url, user_id, entity_id
            ))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(())
    }

    // Plans
    pub async fn get_plans(&self) -> Result<Vec<Plan>> {
        let response = self
            .client
            .get(format!("{}/rest/v1/plans", self.url))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn get_plan(&self, id: &str) -> Result<Plan> {
        let response = self
            .client
            .get(format!("{}/rest/v1/plans?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        let mut plans: Vec<Plan> = response.json().await?;
        plans.pop().ok_or_else(|| anyhow::anyhow!("404"))
    }

    // Subscriptions
    pub async fn get_user_subscription(&self, user_id: &str) -> Result<Subscription> {
        println!("get_user_subscription");
        let response = self
            .client
            .get(format!(
                "{}/rest/v1/subscriptions?entity_id=eq.{}&entity_type=eq.user",
                self.url, user_id
            ))
            .headers(self.build_headers()?)
            .send()
            .await?;

        println!("response: {:?}", response);
        let mut subscriptions: Vec<Subscription> = response.json().await?;
        println!("subscriptions: {:?}", subscriptions);
        subscriptions
            .pop()
            .ok_or_else(|| anyhow::anyhow!("404"))
    }
    pub async fn create_subscription(
        &self,
        entity_id: &str,
        entity_type: &str,
        plan_id: &str,
        status: &str,
        stripe_subscription_id: &str,
        current_period_end: &str,
    ) -> Result<Subscription> {
        let subscription: Subscription = Subscription {
            id: uuid::Uuid::new_v4().to_string(),
            entity_id: entity_id.to_string(),
            entity_type: entity_type.to_string(),
            plan_id: plan_id.to_string(),
            status: status.to_string(),
            stripe_subscription_id: stripe_subscription_id.to_string(),
            current_period_end: current_period_end.to_string(),
            created_at: chrono::Utc::now().to_rfc3339(),
        };

        println!("Creating subscription with payload: {:?}", subscription);

        let response = self
            .client
            .post(format!("{}/rest/v1/subscriptions", self.url))
            .headers(self.build_headers()?)
            .json(&subscription)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to create subscription: {}", error_text));
        }

        Ok(subscription)
    }

    pub async fn update_subscription(
        &self,
        id: &str,
        updates: HashMap<String, serde_json::Value>,
    ) -> Result<()> {
        let response = self
            .client
            .patch(format!("{}/rest/v1/subscriptions?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .json(&updates)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to update subscription: {}", error_text));
        }

        Ok(())
    }

    pub async fn create_feedback_timestamp(
        &self,
        user_id: &str,
        created_at: &str,
    ) -> Result<()> {
        let response = self
            .client
            .post(format!("{}/rest/v1/feedback_timestamps", self.url))
            .headers(self.build_headers()?)
            .json(&serde_json::json!({
                "user_id": user_id,
                "created_at": created_at,
            }))
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(anyhow::anyhow!("Failed to create feedback timestamp: {}", error_text));
        }

        Ok(())
    }

    pub async fn check_feedback_timestamp(
        &self,
        user_id: &str
    ) -> Result<bool> {
        let response = self
            .client
            .get(format!(
                "{}/rest/v1/feedback_timestamps?user_id=eq.{}",
                self.url, user_id
            ))
            .headers(self.build_headers()?)
            .send()
            .await?;

        let timestamps: Vec<serde_json::Value> = response.json().await?;
        println!("timestamps: {:?}", timestamps);
        if timestamps.is_empty() {
            return Ok(true);
        }

        let timestamp = timestamps[0]["created_at"].as_str().ok_or_else(|| anyhow::anyhow!("Invalid timestamp"))?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(timestamp)?;

        if chrono::Utc::now().signed_duration_since(timestamp) < chrono::Duration::hours(24) {
            return Ok(false);
        }

        // Delete the record from the DB
        self.client
            .delete(format!("{}/rest/v1/feedback_timestamps?user_id=eq.{}", self.url, user_id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(true)
    }
}
