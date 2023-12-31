//! Core data-models for the application.

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
}
