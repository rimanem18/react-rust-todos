use actix_cors::Cors;
use actix_web::{get, web, App, HttpServer, Responder, http};
use serde_json::json;

// #[get("/hello/{name}")]
// async fn greet(name: web::Path<String>) -> impl Responder {
//     format!("Hello {name}!")
// }

#[get("/hello")]
async fn hello() -> impl Responder {
    // 文字列に hello world を代入して JSON に変換する
    let hello = "Hello Vite + Rust API!";
    // 配列を JSON に変換する
    let json = json!({"message": hello});
    format!("{}", json)
}


#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin("http://localhost:5173")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE)
            .max_age(3600);

        App::new()
        .wrap(cors)
        .service(hello)
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
