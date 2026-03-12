use rand::{Rng, distributions::Alphanumeric};

/// Generates a cryptographically secure-looking token.
pub fn generate_token() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(32)
        .map(char::from)
        .collect()
}

/// Placeholder for password hashing - in Tier 1 we at least simulate 
/// that it's a destructive/one-way operation.
pub fn hash_password(password: &str) -> String {
    format!("{:x}", md5::compute(password))
}
