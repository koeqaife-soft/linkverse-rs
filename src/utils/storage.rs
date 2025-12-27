const PUBLIC_PATH: &str = "https://storage.sharinflame.com";

/// Normalize URL for giving it to frontend
pub fn normalize_url(url: Option<String>) -> Option<String> {
    match url {
        Some(u) if !u.contains("://") => Some(format!("{}/{}", PUBLIC_PATH, u)),
        Some(u) => Some(u.clone()),
        None => None,
    }
}
