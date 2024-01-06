pub use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
pub use set::Set;
pub use std::{fs, io::Result, sync::Mutex};
// use std::fs::File::file_stream;

mod card;
mod character;
mod game;

pub trait Readable {
    fn from_name(path: String) -> Result<Self>
    where
        Self: std::marker::Sized;
}

#[get("/game")]
async fn game_controller(data: web::Data<Mutex<game::Game>>) -> impl Responder {
    let game = data.lock().unwrap();
    match fs::read_to_string("www/game/index.html") {
        Ok(file) => HttpResponse::Ok().content_type("text/html").body(file),
        Err(..) => HttpResponse::Forbidden().body("404"),
    }
}

fn init() {
    let _ = character::Character::from_name("characters".to_string()).unwrap();
}

#[actix_web::main]
async fn main() -> Result<()> {
    init();
    let data = web::Data::new(Mutex::new(game::GameState::new()));
    HttpServer::new(move || App::new().app_data(data.clone()).service(game_controller))
        .bind(("127.0.0.1", 9999))?
        .run()
        .await
}
