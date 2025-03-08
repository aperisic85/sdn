use actix_web::{App, HttpServer, web};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().route("/", web::get().to(hello_world)))
        .bind("127.0.0.1:8081")?
        .run()
        .await
}

async fn hello_world() -> &'static str {
    "Hello, there!"
}
