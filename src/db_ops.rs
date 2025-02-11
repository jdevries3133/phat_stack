//! Database operations; squirrel code lives here.

use super::models;
use anyhow::Result;
use chrono::{DateTime, Utc};
use sqlx::{
    postgres::{PgPool, PgPoolOptions},
    query, query_as,
};

pub async fn create_pg_pool() -> Result<sqlx::Pool<sqlx::Postgres>> {
    let db_url = &std::env::var("DATABASE_URL")
        .expect("database url to be defined in the environment")[..];

    Ok(PgPoolOptions::new()
        // Postgres default max connections is 100, and we'll take 'em
        // https://www.postgresql.org/docs/current/runtime-config-connection.html
        .max_connections(80)
        .connect(db_url)
        .await?)
}

pub trait GetModel<GetQuery>: Send + Sync {
    /// Get exactly one object from the database, matching the query. WIll
    /// return an error variant if the item does not exist.
    async fn get(db: &PgPool, query: &GetQuery) -> Result<Self>
    where
        Self: Sized;
}

pub trait SaveModel: Send + Sync {
    /// Persist the object to the database
    async fn save(&self, db: &PgPool) -> Result<()>;
}

pub enum UserIdentifer<'a> {
    Id(i32),
    Identifier(&'a str),
}

pub struct GetUserQuery<'a> {
    /// `identifier` can be a users username _or_ email
    pub identifier: UserIdentifer<'a>,
}

struct Qres {
    id: i32,
    username: String,
    email: String,
    created_at: DateTime<Utc>,
}
fn map_into_user(row: Qres) -> models::User {
    models::User {
        id: row.id,
        username: row.username,
        email: row.email,
        created_at: row.created_at,
    }
}

impl<'a> GetModel<GetUserQuery<'a>> for models::User {
    async fn get(db: &PgPool, query: &GetUserQuery<'a>) -> Result<Self> {
        Ok(match query.identifier {
            UserIdentifer::Id(id) => {
                query_as!(
                    Qres,
                    "select
                        id,
                        username,
                        email,
                        created_at
                    from users
                    where id = $1",
                    id
                )
                .map(map_into_user)
                .fetch_one(db)
                .await?
            }
            UserIdentifer::Identifier(ident) => {
                query_as!(
                    Qres,
                    "select
                        id,
                        username,
                        email,
                        created_at
                    from users
                    where username = $1 or email = $1",
                    ident
                )
                .map(map_into_user)
                .fetch_one(db)
                .await?
            }
        })
    }
}

impl SaveModel for models::User {
    async fn save(&self, db: &PgPool) -> Result<()> {
        query!(
            "update users set
                username = $1,
                email = $2
            where id = $3
                ",
            self.username,
            self.email,
            self.id
        )
        .execute(db)
        .await?;
        Ok(())
    }
}
