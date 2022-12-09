use flow::GameFlow;
use logic::TetrisGame;

// #[allow(dead_code)]
mod logic;
// #[allow(dead_code)]
mod flow;
// #[allow(dead_code)]
mod rendering;
// #[allow(dead_code)]
mod utils;

fn main() {
    let engine = &mut TetrisGame::new();
    GameFlow::run(engine);
}
