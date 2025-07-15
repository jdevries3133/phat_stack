//! Core data-models for the application.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;

#[derive(Clone, Debug)]
pub struct AppState {
    pub db: PgPool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
}

/// Utility struct, typically used on `insert ... returning id, created_at`
pub struct IdCreatedAt {
    pub id: i32,
    pub created_at: DateTime<Utc>,
}
