pub mod message_crud;

use sqlx::{migrate::Migrator, PgPool};
use std::env;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

pub async fn initialize_database() -> Result<PgPool, sqlx::Error> {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await?;

    // Применение миграций
    MIGRATOR.run(&pool).await?;
    Ok(pool)
}