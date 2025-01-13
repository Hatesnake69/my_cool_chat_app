use sqlx::{PgPool, Result};
pub use crate::models::topic::Topic;

impl Topic {
    /// Получить все сообщения, отсортированные по дате создания
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>> {
        // Преобразуем поле `created_at` в строку в формате RFC3339 при извлечении из базы
        let messages = sqlx::query_as::<_, Topic>(
            "SELECT id, username, name, TO_CHAR(created_at, 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"') AS created_at
            FROM topics
            ORDER BY id DESC"
        )
            .fetch_all(pool)
            .await?;

        Ok(messages)
    }

    /// Добавить новое топик
    pub async fn add(pool: &PgPool, username: &str, name: &str) -> Result<Self> {
        let message = sqlx::query_as::<_, Topic>(
            "INSERT INTO messages (username, name, created_at)
             VALUES ($1, $2, NOW())
             RETURNING id, username, name, TO_CHAR(created_at, 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"') AS created_at"
        )
            .bind(username)
            .bind(name)
            .fetch_one(pool)
            .await?;

        Ok(message)
    }

    /// Удалить топик по ID
    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64> {
        let rows_affected = sqlx::query("DELETE FROM messages WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();

        Ok(rows_affected)
    }

    /// Получить топик по ID
    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>> {
        let message = sqlx::query_as::<_, Topic>(
            "SELECT id, username, content, TO_CHAR(created_at, 'YYYY-MM-DD\"T\"HH24:MI:SS\"Z\"') AS created_at
             FROM messages WHERE id = $1"
        )
            .bind(id)
            .fetch_optional(pool)
            .await?;

        Ok(message)
    }
}
