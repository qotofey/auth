#![warn(clippy::all, clippy::pedantic)]

pub mod config;
pub mod di;
pub mod providers;

fn main() {
    dotenvy::dotenv().ok();
    let conf = config::Config::init();
    println!("DATABASE_URL={}", conf.database_url);
}

