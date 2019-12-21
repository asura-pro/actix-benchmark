use actix_web::get;
use actix_web::{App, HttpResponse, HttpServer, Responder};

fn main() {
    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8080")
        .unwrap()
        .run()
        .unwrap();
}

#[get("/")]
fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}
