use super::game::Game;

pub fn render(game: &Game) -> String {
    let mut ascii_art = format!("Score: {} \r\n", game.score);
    let mut grid = vec![vec!["░"; game.width as usize]; game.height as usize];
    for point in game.snake.body.iter() {
        grid[point.y as usize][point.x as usize] = "█";
    }

    if let Some(food) = game.food {
        grid[food.y as usize][food.x as usize] = "®";
    }

    for col in 0..game.width {
        for row in 0..game.height {
            ascii_art.push_str(&format!(" {} ", grid[col as usize][row as usize]));
        }
        ascii_art.push_str("\r\n");
    }

    ascii_art
}
