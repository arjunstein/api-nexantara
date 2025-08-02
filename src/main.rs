use actix_web::{get, App, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    "Hello, from Nexantara API"
}

#[actix_web::main]
pub async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new().service(index)
    })
    .bind("127.0.0.1:8081")?
    .run()
    .await
}
