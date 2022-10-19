use argon2::{self, Config, ThreadMode, Variant, Version};
use once_cell::sync::Lazy;

static PASSWORD_SECRET_KEY: Lazy<String> =
    Lazy::new(|| std::env::var("PASSWORD_SECRET_KEY").expect("PASSWORD_SECRET_KEY must be set"));

const SALT: &[u8] = b"saltissalty";

pub fn hash_password(password: &str) -> Result<String, Box<dyn std::error::Error>> {
    let config = Config {
        variant: Variant::Argon2id,
        version: Version::Version13,
        lanes: 4,
        thread_mode: ThreadMode::Parallel,
        secret: PASSWORD_SECRET_KEY.as_bytes(),
        ..Default::default()
    };

    Ok(argon2::hash_encoded(password.as_bytes(), SALT, &config)?)
}

pub fn verify(hash: &str, password: &str) -> Result<bool, Box<dyn std::error::Error>> {
    Ok(argon2::verify_encoded_ext(
        hash,
        password.as_bytes(),
        PASSWORD_SECRET_KEY.as_bytes(),
        &[],
    )?)
}
