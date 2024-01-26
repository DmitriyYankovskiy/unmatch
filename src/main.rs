use actix_web::HttpRequest;
pub use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use reqwest::Response;
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
    file_response("game/index.html".to_string())
}



#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    name: String,
    character_name: String,
}

fn file_response(path: String) -> HttpResponse {
    match fs::read_to_string(format!("www/{}", path)) {
        Ok(file) => HttpResponse::Ok().content_type("text/html").body(file),
        Err(..) => HttpResponse::Forbidden().body("404"),
    }
}

#[get("/game/connect")]
async fn game_connect(data: web::Data<Mutex<game::GameState>>, json: web::Json<Player>) -> impl Responder {
    let mut game = data.lock().unwrap();
    match game.add_player(json.0) {
        Ok(id) => {
            file_response("game/index.html".to_string())
        },
        Err(..) => {
            HttpResponse::InternalServerError().body("character not found")
        }
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
