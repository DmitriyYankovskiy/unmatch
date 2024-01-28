use actix_web::{web::Json, HttpRequest};
pub use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
pub use handlebars::Handlebars;
use handlebars::JsonValue;
use handlebars::{HelperDef, HelperResult, JsonRender};
use reqwest::Response;
use serde::{Serialize, Deserialize};
use serde_json::json;
pub use set::Set;
use std::ops::Deref;
pub use std::{fs, io::Result, sync::Mutex, collections::HashMap};
// use std::fs::File::file_stream;

mod card;
mod character;
mod game;

pub trait Readable {
    fn from_path(path: String) -> Result<Self>
    where
        Self: std::marker::Sized;
}


#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PlayerInfo<T> {
    name: String,
    info: T,
}

fn file_to_string(path: String) -> String {
    match fs::read_to_string(format!("www/{}", path)) {
        Ok(file) => file,
        Err(..) => "Error(dont find file)".to_string(),
    }
} 

fn file_in_layout_response(layout_path: String, options: JsonValue, hbs_data: web::Data<Handlebars<'_>>) -> HttpResponse {
    match fs::read_to_string(format!("www/layouts/{}.html", layout_path)) {
        Ok(file) => HttpResponse::Ok().content_type("text/html")
        .body(match hbs_data.render_template(&file, &options) {
            Ok(file) => file,
            Err(..) => return HttpResponse::Forbidden().body("403"),
        }),
        Err(..) => HttpResponse::Forbidden().body("404"),
    }
}



// ------ Controllers ------

#[get("/game")]
async fn game_controller(hbs_data: web::Data<Handlebars<'_>>) -> impl Responder {
    file_in_layout_response("main".to_string(), json!({"title": "Game", "page": "game/index.html"}), hbs_data)
}

#[get("/game/connect")]
async fn game_connect(game_data: web::Data<Mutex<game::GameState>>, hbs_data: web::Data<Handlebars<'_>>, json: web::Json<PlayerInfo<String>>) -> impl Responder {
    let mut game = game_data.lock().unwrap();
    match game.add_player(json.0) {
        Ok(id) => {
            file_in_layout_response("main".to_string(), json!({"title": "Game", "page": "game/index.html"}), hbs_data)
        },
        Err(..) => {
            HttpResponse::InternalServerError().body("character not found")
        }
    }
}

fn hbs_init(hbs: &mut Handlebars) {
    hbs.register_helper("partial", Box::new(
        |h: &handlebars::Helper, hbs: &Handlebars, ctx: &handlebars::Context, rc: &mut handlebars::RenderContext, out: &mut dyn handlebars::Output| -> HelperResult {
            let name =
            h.param(0).ok_or(handlebars::RenderErrorReason::ParamNotFoundForIndex("closure-helper", 0))?;

            out.write(file_to_string(name.value().render()).as_str())?;
            Ok(())
        }
    ));
}

#[actix_web::main]
async fn main() -> Result<()> {
    let game_data = web::Data::new(Mutex::new(game::GameState::new()));
    let mut hbs: Handlebars<'_> = Handlebars::new();

    hbs_init(&mut hbs);

    let hbs_data: web::Data<Handlebars<'_>> = web::Data::new(hbs);
    HttpServer::new(move || 
        App::new()
        .app_data(game_data.clone())
        .app_data(hbs_data.clone())
        .service(game_controller)
    )    
    .bind(("127.0.0.1", 9999))?
    .run()
    .await
}
