use actix_web::{delete, get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
pub use crate::db::topic_crud::Topic;
pub use crate::models::topic::NewTopic;

#[utoipa::path(
    get,
    path = "/topics",
    responses(
        (status = 200, description = "List of topics", body = [Topic])
    )
)]
#[get("/topics")]
pub async fn get_topics(pool: web::Data<PgPool>) -> impl Responder {
    match Topic::get_all(pool.get_ref()).await {
        Ok(topics) => HttpResponse::Ok().json(topics),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching topics"),
    }
}

#[utoipa::path(
    post,
    path = "/topics",
    request_body = NewTopic,
    responses(
        (status = 201, description = "Topic created", body = Topic)
    )
)]
#[post("/topics")]
pub async fn post_topic(
    pool: web::Data<PgPool>,
    new_topic: web::Json<NewTopic>,
) -> impl Responder {
    match Topic::add(
        pool.get_ref(),
        &new_topic.username,
        &new_topic.name,
    )
        .await
    {
        Ok(topic) => HttpResponse::Created().json(topic),
        Err(_) => HttpResponse::InternalServerError().body("Error adding topic"),
    }
}

#[utoipa::path(
    get,
    path = "/topics/{id}",
    params(
        ("id" = i32, Path, description = "Topic ID")
    ),
    responses(
        (status = 200, description = "Topic found", body = Topic),
        (status = 404, description = "Topic not found")
    )
)]
#[get("/topics/{id}")]
pub async fn get_topic_by_id(
    pool: web::Data<PgPool>,
    topic_id: web::Path<i32>,
) -> impl Responder {
    match Topic::get_by_id(pool.get_ref(), *topic_id).await {
        Ok(Some(topic)) => HttpResponse::Ok().json(topic),
        Ok(None) => HttpResponse::NotFound().body("Topic not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching topic"),
    }
}

#[utoipa::path(
    delete,
    path = "/topics/{id}",
    params(
        ("id" = i32, Path, description = "Topic ID")
    ),
    responses(
        (status = 204, description = "Topic deleted"),
        (status = 404, description = "Topic not found")
    )
)]
#[delete("/topics/{id}")]
pub async fn delete_topic_by_id(
    pool: web::Data<PgPool>,
    topic_id: web::Path<i32>,
) -> impl Responder {
    match Topic::delete(pool.get_ref(), *topic_id).await {
        Ok(rows_affected) if rows_affected > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().body("Topic not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting topic"),
    }
}
