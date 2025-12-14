use crate::utils::snowflake::SnowflakeGenerator;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AuthUser {
    pub username: String,
    pub user_id: String,
    pub email: String,
    #[serde(skip_serializing)]
    pub password_hash: Option<String>,
    pub email_verified: Option<bool>,
    pub pending_email: Option<String>,
    pub pending_email_until: Option<i64>,
}

impl AuthUser {
    pub fn created_at(&self) -> f64 {
        SnowflakeGenerator::parse(self.user_id.parse().expect("Wrong ID type")).0
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct User {}
