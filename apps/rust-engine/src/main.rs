// main.rs
use std::sync::Arc;
use actix_web::{web, App, HttpServer};
use sqlx::postgres::PgPoolOptions;
use env_logger::Env;
use std::fs;
use crate::{utils::config};



mod error;
mod middleware;
mod providers;
mod routes;
mod schemas;
mod utils;

use crate::{
    config::Config,
    middleware::{Auth, Cache, Logging, SecurityHeaders},
    providers::ProviderFactory,
    routes::{
        accounts::get_accounts,
        auth::{exchange_token, refresh_token_handler},
        connections::{delete_connection, get_connections},
        institutions::{get_institution, get_institutions, update_institution_usage},
        transactions::get_transactions,
        health::health_check,
    },
};

async fn run_migrations(pool: &sqlx::PgPool) -> Result<(), Box<dyn std::error::Error>> {
    let migration_sql = fs::read_to_string("migrations/20231228_init.sql")?;
    sqlx::query(&migration_sql)
        .execute(pool)
        .await?;
    Ok(())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment
    dotenv::dotenv().ok();
    env_logger::init_from_env(Env::default().default_filter_or("info"));

    // Load config
    let config = Arc::new(Config::from_env().expect("Failed to load config"));
    let port = config.port;

    // Initialize database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to Postgres");

    // Run migrations
    run_migrations(&pool)
        .await
        .expect("Failed to run database migrations");

    // Initialize provider factory, passing in the config
    let provider_factory = Arc::new(ProviderFactory::new(config.clone()));

    // Start HTTP server
    HttpServer::new(move || {
        App::new()
            .wrap(Logging::new())
            .wrap(SecurityHeaders::new())
            .wrap(Auth::new())
            .wrap(Cache)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(provider_factory.clone()))
            .app_data(web::Data::new(config.clone()))
            .service(
                web::scope("/api/v1")
                    .service(health_check)
                    .service(get_accounts)
                    .service(exchange_token)
                    .service(refresh_token_handler)
                    .service(get_connections)
                    .service(delete_connection)
                    .service(get_institutions)
                    .service(get_institution)
                    .service(update_institution_usage)
                    .service(get_transactions),
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
