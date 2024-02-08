use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "This is actix from next-rust-practice !"
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(index))
        .bind(("backend", 3000))?
        .run()
        .await
}
