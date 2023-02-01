use super::ascii_art;
use super::directions::Direction;
use super::state::State;
use tide::log::LogMiddleware;
use tide::prelude::*;
use tide::{Request, Response, Result, Server, StatusCode};

pub fn init() -> Server<State> {
    femme::start();

    let mut app = tide::with_state(State::new());

    app.with(LogMiddleware::new());
    app.at("/snake").get(req_get_snake);
    app.at("/snake").post(req_post_snake_direction);
    app
}

async fn req_get_snake(req: Request<State>) -> Result {
    let state = req.state();
    let game = state.game.clone();
    let locked_game = game.lock().unwrap();

    let response = Response::builder(StatusCode::Ok)
        .body(ascii_art::render(&locked_game))
        .content_type("text/plain;charset=utf-8")
        .build();

    Ok(response)
}

#[derive(Deserialize)]
#[serde(default)]
struct Params {
    direction: String,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            direction: String::from("up"),
        }
    }
}

async fn req_post_snake_direction(req: Request<State>) -> Result {
    let state = req.state();
    let params: Params = req.query()?;
    let direction = match Direction::from_str(&params.direction) {
        Some(d) => d,
        None => panic!("Not found"),
    };

    let result = state.directions.clone();
    let mut directions = result.lock().unwrap();

    directions.push(direction);

    let response = Response::builder(StatusCode::Ok)
        .body(format!(
            "You sent direction: {direction:?} \n State directions: {directions:?}"
        ))
        .content_type("text/plain;charset=utf-8")
        .build();

    Ok(response)
}
