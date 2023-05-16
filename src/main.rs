use std::net::TcpListener;
use zero2prod::configuration::get_configuration;
use zero2prod::startup;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let config = get_configuration().expect("failed to read config file.");

    let addr = format!("127.0.0.1:{}", config.application_port);

    let listener = TcpListener::bind(addr)?;
    startup::run(listener)?.await
}
