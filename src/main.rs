use actix_web::HttpRequest;
pub use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Serialize, Deserialize};
pub use set::Set;
use std::ops::Deref;
pub use std::{fs, io::Result, sync::Mutex};
// use std::fs::File::file_stream;

mod card;
mod character;
mod game;

pub trait Readable {
    fn from_path(path: String) -> Result<Self>
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



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct DeckName {
    name: String,
}

#[get("/game/connect")]
async fn game_connect(data: web::Data<Mutex<game::GameState>>, json: web::Json<DeckName>) -> impl Responder {
    let mut game = data.lock().unwrap();
    game.add_player(&json.name);
    match fs::read_to_string("www/game/new.html") {
        Ok(file) => HttpResponse::Ok().content_type("text/html").body(file),
        Err(..) => HttpResponse::Forbidden().body("404"),
    }
}


#[actix_web::main]
async fn main() -> Result<()> {
    let mut data = web::Data::new(Mutex::new(game::GameState::new()));
    HttpServer::new(move || 
        App::new()
        .app_data(data.clone())
        .service(game_controller)
    )
    .bind(("127.0.0.1", 9999))?
    .run()
    .await
}
