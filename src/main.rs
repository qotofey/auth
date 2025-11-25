#![warn(clippy::all, clippy::pedantic)]

pub mod config;
pub mod di;
pub mod providers;
pub mod app;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let conf = config::Config::init();
    println!("DATABASE_URL={}", conf.database_url);

    let argon2_provider = providers::Argon2Provider::new();
    let container = di::Container::new(argon2_provider);
    println!("Password hash = {}", container.create_password_hash_command.call("Qwerty123!".to_string()).await.unwrap());
}

