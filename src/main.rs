use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde_json::json;

#[actix_web::main]
async fn main() -> Result<(), actix_web::Error> {
    let _ = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(hello))
            .route("/echo", web::post().to(echo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await;

    Ok(())
}

async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!!!")
}

async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().json(json!(req_body))
}
