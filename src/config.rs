use dotenv::dotenv;
use std::env;

pub fn load_environment() {
    dotenv().ok(); // Загружаем переменные из .env
    if env::var("DATABASE_URL").is_err() {
        eprintln!("Error: DATABASE_URL is not set");
        std::process::exit(1);
    }
}
