use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use actix_web::http::header::ContentType;
mod dto;

#[post("/read")]
async fn read() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/write")]
async fn write(req_body: web::Json<dto::Event>) -> impl Responder {
    let state: dto::Event = dto::Event {
        event_id: req_body.event_id.to_string(),
        event_type: req_body.event_id.to_string(),
        event_time: req_body.event_time.to_string(),
    };
    
    HttpResponse::Ok().content_type("application/json").body(web::Json(state.event_id).to_string())
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(read)
            .service(write)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}