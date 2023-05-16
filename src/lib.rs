use actix_web::{App, HttpResponse, HttpServer, Responder, web};
use actix_web::dev::Server;

async fn health_check() -> impl Responder {
    HttpResponse::Ok()
}

pub fn run(addr: &str) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(|| App::new().route("/health_check", web::get().to(health_check)))
        .bind(addr)?
        .run();

    Ok(server)
}