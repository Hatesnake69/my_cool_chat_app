use sqlx::{PgPool, Result};
pub use crate::models::message::Message;


impl Message {
    /// Получить все сообщения, отсортированные по дате создания
    pub async fn get_all(pool: &PgPool) -> Result<Vec<Self>> {
        let messages = sqlx::query_as::<_, Message>("SELECT * FROM messages ORDER BY created_at DESC")
            .fetch_all(pool)
            .await?;
        Ok(messages)
    }

    /// Добавить новое сообщение
    pub async fn add(pool: &PgPool, username: &str, content: &str) -> Result<Self> {
        let message = sqlx::query_as::<_, Message>(
            "INSERT INTO messages (username, content)
             VALUES ($1, $2)
             RETURNING *",
        )
            .bind(username)
            .bind(content)
            .fetch_one(pool)
            .await?;
        Ok(message)
    }

    /// Удалить сообщение по ID
    pub async fn delete(pool: &PgPool, id: i32) -> Result<u64> {
        let rows_affected = sqlx::query("DELETE FROM messages WHERE id = $1")
            .bind(id)
            .execute(pool)
            .await?
            .rows_affected();
        Ok(rows_affected)
    }

    /// Получить сообщение по ID
    pub async fn get_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>> {
        let message = sqlx::query_as::<_, Message>("SELECT * FROM messages WHERE id = $1")
            .bind(id)
            .fetch_optional(pool)
            .await?;
        Ok(message)
    }
}
