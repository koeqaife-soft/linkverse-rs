use deadpool_postgres::Transaction;
use serde::Deserialize;
use tokio_postgres::Row;

use crate::{database::conn::LazyConn, entities::user::User, utils::storage::normalize_url};

/// Private function for converting Row to User
fn row_to_user(row: Row) -> User {
    User {
        user_id: row.get("user_id"),
        username: row.get("username"),
        role_id: row.get("role_id"),
        display_name: row.get("display_name"),
        avatar_url: normalize_url(row.get("avatar_url")),
        banner_url: normalize_url(row.get("banner_url")),
        bio: row.get("bio"),
        badges: row.get("badges"),
        languages: row.get("languages"),
        following_count: row.get("following_count"),
        followers_count: row.get("followers_count"),
    }
}

/// Get minimized user from database
pub async fn get_min_user(user_id: &String, conn: &mut LazyConn) -> Option<User> {
    let db = conn.get_client().await.unwrap();
    let sql = "
        SELECT u.user_id, u.username, p.display_name, u.role_id,
               ac.objects[1] as avatar_url
        FROM users u
        LEFT JOIN user_profiles p ON u.user_id = p.user_id
        LEFT JOIN files ac ON ac.context_id = p.avatar_context_id
        WHERE u.user_id = $1;
    ";
    let row = db.query_opt(sql, &[user_id]).await.unwrap();
    row.map(row_to_user)
}

/// Get full user from database
pub async fn get_user(user_id: &String, conn: &mut LazyConn) -> Option<User> {
    let db = conn.get_client().await.unwrap();
    let sql = "
        SELECT u.user_id, u.username, p.display_name, u.role_id,
               ac.objects[1] as avatar_url,
               bc.objects[1] as banner_url, p.bio, p.badges, p.languages,
               u.following_count, u.followers_count
        FROM users u
        LEFT JOIN user_profiles p ON u.user_id = p.user_id
        LEFT JOIN files ac ON ac.context_id = p.avatar_context_id
        LEFT JOIN files bc ON bc.context_id = p.banner_context_id
        WHERE u.user_id = $1;
    ";
    let row = db.query_opt(sql, &[user_id]).await.unwrap();
    row.map(row_to_user)
}

#[derive(Default, Debug)]
pub struct UserProfileUpdate {
    pub display_name: Option<String>,
    pub avatar_context_id: Option<String>,
    pub banner_context_id: Option<String>,
    pub bio: Option<String>,
    pub languages: Option<Vec<String>>,
}

/// Updates user profile
pub async fn update_user_profile(
    user_id: &str,
    update: UserProfileUpdate,
    tx: &mut Transaction<'_>,
) -> bool {
    let mut set_clauses = Vec::new();
    let mut values: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = Vec::new();

    if let Some(ref name) = update.display_name {
        values.push(name);
        set_clauses.push(format!("display_name = ${}", values.len() + 1));
    }
    if let Some(ref avatar) = update.avatar_context_id {
        values.push(avatar);
        set_clauses.push(format!("avatar_context_id = ${}", values.len() + 1));
    }
    if let Some(ref banner) = update.banner_context_id {
        values.push(banner);
        set_clauses.push(format!("banner_context_id = ${}", values.len() + 1));
    }
    if let Some(ref bio) = update.bio {
        values.push(bio);
        set_clauses.push(format!("bio = ${}", values.len() + 1));
    }
    if let Some(ref langs) = update.languages {
        values.push(langs);
        set_clauses.push(format!("languages = ${}", values.len() + 1));
    }

    if set_clauses.is_empty() {
        return false;
    }

    let query = format!(
        "UPDATE user_profiles SET {} WHERE user_id = $1",
        set_clauses.join(", ")
    );

    let mut params: Vec<&(dyn tokio_postgres::types::ToSql + Sync)> = vec![&user_id];
    params.extend(values);

    tx.execute(query.as_str(), &params).await.unwrap();
    true
}
