use actix_web::{middleware::Logger, web, App, HttpServer};
use rust_auth_server::{
    config::Config, infrastructures::database::postgres::DbPostgres,
    interfaces::api::handlers::configure,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    let config = Config::from_env();
    let db = DbPostgres::new(&config.db_url).await.unwrap();

    tracing::info!("Starting server at {}:{}", "localhost", config.port);

    HttpServer::new(move || {
        App::new()
            .wrap(Logger::default())
            .app_data(web::Data::new(db.pool.clone()))
            .configure(configure)
    })
    .bind(("0.0.0.0", config.port))?
    .run()
    .await
}
