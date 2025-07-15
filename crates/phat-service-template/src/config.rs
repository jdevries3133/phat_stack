/// This is for all authentication sessions; users will need to log in again
/// every 30 days, since we basically have JWT authentication.
pub const SESSION_EXPIRY_TIME_DAYS: i64 = 30;

/// Password reset links will expire after 15 minutes.
pub const RESET_TOKEN_TIMEOUT_MINUTES: i64 = 15;

#[cfg(not(feature = "localhost_base_url"))]
pub const BASE_URL: &str = "https://beancount.bot";

#[cfg(feature = "localhost_base_url")]
pub const BASE_URL: &str = "http://localhost:8000";
