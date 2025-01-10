use actix_web::{middleware, web, App, HttpServer};
use actix_web::body::MessageBody;
use clap::Command;
use db::initialize_database;
use sqlx::migrate::Migrator;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod config;
pub mod db;
pub mod models;
pub mod routes;

use crate::config::load_environment;
use crate::routes::messages;

static MIGRATOR: Migrator = sqlx::migrate!("./migrations");

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::routes::messages::get_messages,
        crate::routes::messages::post_message,
        crate::routes::messages::get_message_by_id,
        crate::routes::messages::delete_message_by_id
    ),
    components(schemas(crate::models::message::Message, crate::models::message::NewMessage)),
    tags(
        (name = "Messages", description = "Message management endpoints")
    )
)]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    load_environment();

    // Настройка CLI с помощью `Command`
    let matches = Command::new("My Cool Chat App")
        .version("1.0")
        .author("Your Name")
        .about("Chat application backend")
        .subcommand(
            Command::new("migrate")
                .about("Apply database migrations"),
        )
        .get_matches();

    match matches.subcommand() {
        Some(("migrate", _)) => {
            apply_migrations().await;
        }
        _ => {
            start_server().await?;
        }
    }

    Ok(())
}

async fn apply_migrations() {
    // Переиспользуем существующую инициализацию БД
    let db_pool = initialize_database()
        .await
        .expect("Failed to initialize the database");

    println!("Applying migrations...");
    MIGRATOR
        .run(&db_pool)
        .await
        .expect("Failed to apply migrations");
    println!("Migrations applied successfully.");
}


async fn start_server() -> std::io::Result<()> {
    // Инициализируем соединение с БД
    let db_pool = initialize_database()
        .await
        .expect("Failed to initialize the database");

    println!("Starting server...");
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default()) // Добавляем логгер для запросов
            .app_data(web::Data::new(db_pool.clone()))
            .service(messages::get_messages)
            .service(messages::post_message)
            .service(messages::get_message_by_id)
            .service(messages::delete_message_by_id)
            .service(setup_swagger_ui())
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}


fn setup_swagger_ui() -> impl actix_web::dev::HttpServiceFactory {
    // Swagger UI настраиваем через web::scope
    web::scope("")
        .service(SwaggerUi::new("/swagger-ui/{_:.*}").url("/api-doc/openapi.json", ApiDoc::openapi()))
}