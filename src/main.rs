use actix_web::{App, HttpServer, web};
use dotenv::dotenv;

mod db;
mod models;
mod users;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let pool = db::init_db().await;
    let pool = web::Data::new(pool); //wrap to actix smart

    HttpServer::new(move || {
        App::new().app_data(pool.clone())
        // Add your routes here
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
