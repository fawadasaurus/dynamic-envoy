use actix_web::{web, middleware, App, HttpRequest, HttpServer, Responder};
use std::env;

async fn hostname(req: HttpRequest) -> impl Responder {
    let conn_info = req.connection_info().clone();
    let host: Vec<&str> = conn_info.host().split(":").collect();
    format!("{}\n", host[0])
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(hostname))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}