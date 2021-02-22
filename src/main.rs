use newsletter::configuration::get_configuration;
use newsletter::startup::run;
use newsletter::telemetry::{get_subscriber, init_subscriber};
use sqlx::PgPool;
use std::net::TcpListener;
use tracing_log::LogTracer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    LogTracer::init().expect("Failed to set logger");
    let subscriber = get_subscriber("newsletter".into(), "info".into());
    init_subscriber(subscriber);

    let config = get_configuration().expect("Failed to read config!");
    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("Failed to connect to Postgres.");

    let address = format!("127.0.0.1:{}", config.application_port);
    let listener = TcpListener::bind(address)?;
    run(listener, connection_pool)?.await?;
    Ok(())
}
