use super::state::State;
use std::thread;
use std::time::Duration;
use tide::Server;

const SLEEP_FOR_SECS: u64 = 10;

pub fn run(app: &Server<State>) {
    let state = app.state();

    let directions = state.directions.clone();
    let game = state.game.clone();

    std::thread::spawn(move || loop {
        {
            let mut locked_game = game.lock().unwrap();
            let mut locked_directions = directions.lock().unwrap();

            locked_game.run(locked_directions.clone());
            locked_directions.clear();
        }
        thread::sleep(Duration::from_secs(SLEEP_FOR_SECS));
    });
}
