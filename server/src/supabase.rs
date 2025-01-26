use anyhow::Result;
use reqwest::{header::HeaderMap, Client};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Type definitions matching Database.ts
#[derive(Debug, Serialize, Deserialize)]
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
    pub owner_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Organization {
    pub id: String,
    pub name: String,
    pub created_at: String,
    pub owner_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
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
    pub stripe_subscription_id: Option<String>,
    pub current_period_end: Option<String>,
    pub created_at: Option<String>,
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
        headers.insert(
            "Authorization",
            format!("Bearer {}", self.api_key).parse()?,
        );
        Ok(headers)
    }

    // Users
    pub async fn get_users(&self) -> Result<Vec<User>> {
        let response = self.client
            .get(format!("{}/rest/v1/users", self.url))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn get_user(&self, id: &str) -> Result<User> {
        let response = self.client
            .get(format!("{}/rest/v1/users?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        let mut users: Vec<User> = response.json().await?;
        users.pop().ok_or_else(|| anyhow::anyhow!("User not found"))
    }

    pub async fn get_user_by_email(&self, email: &str) -> Result<User> {
        let response = self.client
            .get(format!("{}/rest/v1/users?email=eq.{}", self.url, email))
            .headers(self.build_headers()?)
            .send()
            .await?;

        let mut users: Vec<User> = response.json().await?;
        users.pop().ok_or_else(|| anyhow::anyhow!("User not found"))
    }

    pub async fn get_plan_by_stripe_id(&self, stripe_id: &str) -> Result<Plan> {
        let response = self.client
            .get(format!("{}/rest/v1/plans?stripe_id=eq.{}", self.url, stripe_id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        let mut plans: Vec<Plan> = response.json().await?;
        plans.pop().ok_or_else(|| anyhow::anyhow!("Plan not found"))
    }

    pub async fn create_user(&self, email: &str) -> Result<User> {
        let user = serde_json::json!({
            "email": email,
        });

        let response = self.client
            .post(format!("{}/rest/v1/users", self.url))
            .headers(self.build_headers()?)
            .json(&user)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn update_user(&self, id: &str, updates: HashMap<String, serde_json::Value>) -> Result<User> {
        let response = self.client
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
        let response = self.client
            .get(format!("{}/rest/v1/teams", self.url))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn get_team(&self, id: &str) -> Result<Team> {
        let response = self.client
            .get(format!("{}/rest/v1/teams?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        let mut teams: Vec<Team> = response.json().await?;
        teams.pop().ok_or_else(|| anyhow::anyhow!("Team not found"))
    }

    pub async fn create_team(&self, name: &str, owner_id: &str, plan_id: &str, org_id: Option<&str>) -> Result<Team> {
        let mut params = vec![
            ("team_name", name.to_string()),
            ("owner_id", owner_id.to_string()),
            ("plan_id", plan_id.to_string()),
        ];

        if let Some(org_id) = org_id {
            params.push(("org_id", org_id.to_string()));
        }

        let response = self.client
            .post(format!("{}/rest/v1/rpc/create_team", self.url))
            .headers(self.build_headers()?)
            .form(&params)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn update_team(&self, id: &str, updates: HashMap<String, serde_json::Value>) -> Result<Team> {
        let response = self.client
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
        let response = self.client
            .get(format!("{}/rest/v1/links?owner_id=eq.{}&owner_type=eq.{}",
                self.url, owner_id, owner_type))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn create_link(&self, link: Link) -> Result<Link> {
        let response = self.client
            .post(format!("{}/rest/v1/links", self.url))
            .headers(self.build_headers()?)
            .json(&link)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn update_link(&self, id: &str, updates: HashMap<String, serde_json::Value>) -> Result<Link> {
        let response = self.client
            .patch(format!("{}/rest/v1/links?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .json(&updates)
            .send()
            .await?;

        Ok(response.json().await?)
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
        let response = self.client
            .get(format!("{}/rest/v1/organizations", self.url))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn create_organization(&self, org_name: &str, owner_id: &str, plan_id: &str) -> Result<Organization> {
        let params = vec![
            ("org_name", org_name.to_string()),
            ("owner_id", owner_id.to_string()),
            ("plan_id", plan_id.to_string()),
        ];

        let response = self.client
            .post(format!("{}/rest/v1/rpc/create_organization", self.url))
            .headers(self.build_headers()?)
            .form(&params)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn update_organization(&self, id: &str, updates: HashMap<String, serde_json::Value>) -> Result<Organization> {
        let response = self.client
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
        let response = self.client
            .get(format!("{}/rest/v1/user_memberships?user_id=eq.{}", self.url, user_id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn add_member(&self, membership: UserMembership) -> Result<()> {
        let response = self.client
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

    pub async fn update_member_role(&self, user_id: &str, entity_id: &str, role: &str) -> Result<UserMembership> {
        let updates = HashMap::from([
            ("role".to_string(), serde_json::Value::String(role.to_string())),
        ]);

        let response = self.client
            .patch(format!("{}/rest/v1/user_memberships?user_id=eq.{}&entity_id=eq.{}",
                self.url, user_id, entity_id))
            .headers(self.build_headers()?)
            .json(&updates)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn remove_member(&self, user_id: &str, entity_id: &str) -> Result<()> {
        self.client
            .delete(format!("{}/rest/v1/user_memberships?user_id=eq.{}&entity_id=eq.{}",
                self.url, user_id, entity_id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(())
    }

    // Plans
    pub async fn get_plans(&self) -> Result<Vec<Plan>> {
        let response = self.client
            .get(format!("{}/rest/v1/plans", self.url))
            .headers(self.build_headers()?)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn get_plan(&self, id: &str) -> Result<Plan> {
        let response = self.client
            .get(format!("{}/rest/v1/plans?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        let mut plans: Vec<Plan> = response.json().await?;
        plans.pop().ok_or_else(|| anyhow::anyhow!("Plan not found"))
    }

    // Subscriptions
    pub async fn get_user_subscription(&self, user_id: &str) -> Result<Subscription> {
        println!("get_user_subscription");
        let response = self.client
            .get(format!("{}/rest/v1/subscriptions?entity_id=eq.{}&entity_type=eq.user",
                self.url, user_id))
            .headers(self.build_headers()?)
            .send()
            .await?;

        println!("response: {:?}", response);
        let mut subscriptions: Vec<Subscription> = response.json().await?;
        println!("subscriptions: {:?}", subscriptions);
        subscriptions.pop().ok_or_else(|| anyhow::anyhow!("Subscription not found"))
    }

    pub async fn create_subscription(&self, subscription: Subscription) -> Result<Subscription> {
        let response = self.client
            .post(format!("{}/rest/v1/subscriptions", self.url))
            .headers(self.build_headers()?)
            .json(&subscription)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn update_subscription(&self, id: &str, updates: HashMap<String, serde_json::Value>) -> Result<Subscription> {
        let response = self.client
            .patch(format!("{}/rest/v1/subscriptions?id=eq.{}", self.url, id))
            .headers(self.build_headers()?)
            .json(&updates)
            .send()
            .await?;

        Ok(response.json().await?)
    }

}