use base64::Engine;
use pbkdf2::pbkdf2_hmac;
use rand::TryRngCore;
use rand::rngs::OsRng;
use sha2::Sha256;

fn generate_key(length: usize) -> String {
    let mut bytes = vec![0u8; length];
    OsRng.try_fill_bytes(&mut bytes).unwrap();
    base64::engine::general_purpose::STANDARD.encode(&bytes)
}

fn generate_salt() -> [u8; 16] {
    let mut salt = [0u8; 16];
    OsRng.try_fill_bytes(&mut salt).unwrap();
    salt
}

fn hash_password(password: &str, salt: &[u8]) -> Vec<u8> {
    let mut hash = vec![0u8; 32];
    pbkdf2_hmac::<Sha256>(password.as_bytes(), salt, 10_000, &mut hash);
    hash
}

fn store_password(password: &str) -> String {
    let salt = generate_salt();
    let hashed = hash_password(password, &salt);
    format!("{}${}", hex::encode(salt), hex::encode(hashed))
}

fn check_password(stored: &str, password: &str) -> bool {
    let parts: Vec<&str> = stored.split('$').collect();
    if parts.len() != 2 {
        return false;
    }
    let salt = hex::decode(parts[0]).unwrap();
    let stored_hash = hex::decode(parts[1]).unwrap();
    let new_hash = hash_password(password, &salt);
    new_hash == stored_hash
}
