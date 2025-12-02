use crate::utils::snowflake::SnowflakeGenerator;

struct AuthUser {
    pub username: String,
    pub user_id: String,
    pub email: String,
    pub password_hash: String,
    pub email_verified: bool,
    pub pending_email: Option<String>,
    pub pending_email_until: u64,
}

impl AuthUser {
    pub fn created_at(&self) -> f64 {
        SnowflakeGenerator::parse(self.user_id.parse().expect("Wrong ID type")).0
    }
}

struct User {}
