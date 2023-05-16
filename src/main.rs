use sqlx::PgPool;
use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("failed to read config file.");

    let connection_pool = PgPool::connect(&config.database.connection_string())
        .await
        .expect("failed to connect to database");

    let addr = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(addr)?;
    run(listener, connection_pool)?.await
}
