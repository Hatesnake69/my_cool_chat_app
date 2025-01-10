use actix_web::{delete, get, post, web, HttpResponse, Responder};
use sqlx::PgPool;
pub use crate::db::message_crud::Message;
pub use crate::models::message::NewMessage;

#[utoipa::path(
    get,
    path = "/messages",
    responses(
        (status = 200, description = "List of messages", body = [Message])
    )
)]
#[get("/messages")]
pub async fn get_messages(pool: web::Data<PgPool>) -> impl Responder {
    match Message::get_all(pool.get_ref()).await {
        Ok(messages) => HttpResponse::Ok().json(messages),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching messages"),
    }
}

#[utoipa::path(
    post,
    path = "/messages",
    request_body = NewMessage,
    responses(
        (status = 201, description = "Message created", body = Message)
    )
)]
#[post("/messages")]
pub async fn post_message(
    pool: web::Data<PgPool>,
    new_message: web::Json<NewMessage>,
) -> impl Responder {
    match Message::add(
        pool.get_ref(),
        &new_message.username,
        &new_message.content,
    )
        .await
    {
        Ok(message) => HttpResponse::Created().json(message),
        Err(_) => HttpResponse::InternalServerError().body("Error adding message"),
    }
}

#[utoipa::path(
    get,
    path = "/messages/{id}",
    params(
        ("id" = i32, Path, description = "Message ID")
    ),
    responses(
        (status = 200, description = "Message found", body = Message),
        (status = 404, description = "Message not found")
    )
)]
#[get("/messages/{id}")]
pub async fn get_message_by_id(
    pool: web::Data<PgPool>,
    message_id: web::Path<i32>,
) -> impl Responder {
    match Message::get_by_id(pool.get_ref(), *message_id).await {
        Ok(Some(message)) => HttpResponse::Ok().json(message),
        Ok(None) => HttpResponse::NotFound().body("Message not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error fetching message"),
    }
}

#[utoipa::path(
    delete,
    path = "/messages/{id}",
    params(
        ("id" = i32, Path, description = "Message ID")
    ),
    responses(
        (status = 204, description = "Message deleted"),
        (status = 404, description = "Message not found")
    )
)]
#[delete("/messages/{id}")]
pub async fn delete_message_by_id(
    pool: web::Data<PgPool>,
    message_id: web::Path<i32>,
) -> impl Responder {
    match Message::delete(pool.get_ref(), *message_id).await {
        Ok(rows_affected) if rows_affected > 0 => HttpResponse::NoContent().finish(),
        Ok(_) => HttpResponse::NotFound().body("Message not found"),
        Err(_) => HttpResponse::InternalServerError().body("Error deleting message"),
    }
}
