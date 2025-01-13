use serde::{Deserialize, Serialize};
use utoipa::ToSchema;


#[derive(sqlx::FromRow, Serialize, Deserialize, Debug, ToSchema)]
pub struct Topic {
    pub id: i32,
    pub username: String,
    pub name: String,
    pub created_at: String,
}
#[derive(Deserialize, ToSchema)]
pub struct NewTopic {
    pub username: String,
    pub name: String,
}