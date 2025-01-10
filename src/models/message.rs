use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, ToSchema)]
pub struct Message {
    pub id: i32,
    pub username: String,
    pub content: String,
    #[schema(example = "2023-12-31T23:59:59")]
    pub created_at: String,
}
#[derive(Deserialize, ToSchema)]
pub struct NewMessage {
    pub username: String,
    pub content: String,
}