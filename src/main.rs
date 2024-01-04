use actix_web::{get, Responder, HttpServer, App, HttpResponse, web};
use std::{io::Result, fs, cell::RefCell, sync::Mutex};
// use std::fs::File::file_stream;

mod card;
mod game;

#[get("/game")]
async fn game_controller(data: web::Data<game::Game>) -> impl Responder {
    data.add();
    println!("{}", data.get());
    match fs::read_to_string("www/game/index.html") {
        Ok(file) => {
            HttpResponse::Ok()
            .content_type("text/html")
            .body(file)
        },
        Err(..) => {
            HttpResponse::Forbidden()
            .body("404")
        },
    }
    
} 

fn init() {
    card::get_deck("ew".to_string());
}

#[actix_web::main]
async fn main() -> Result<()> {
    init();
    let data = web::Data::new(game::Game {
            count: Mutex::new(5),
        });
    HttpServer::new(move || {
        App::new()
            .app_data(data.clone())
            .service(game_controller)
    })
    .bind(("127.0.0.1", 9999))?
    .run()
    .await
}