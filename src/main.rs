use actix_web::{get, Responder, HttpServer, App, HttpResponse};
use std::{io::Result, fs};
// use std::fs::File::file_stream;

#[get("/game")]
async fn game() -> impl Responder {
    match fs::read_to_string("www/game/index.html") {
        Ok(file) => {
            println!("{file}");
            HttpResponse::Ok()
            .content_type("text/html")
            .body(file)
        },
        Err(..) => {
            println!("error");
            HttpResponse::Forbidden()
            .body("404")
        },
    }
    
} 

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(game)
    })
    .bind(("127.0.0.1", 9999))?
    .run()
    .await
}