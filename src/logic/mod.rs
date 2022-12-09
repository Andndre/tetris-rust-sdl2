use cgmath::Vector2;
use rand::{rngs::ThreadRng, thread_rng, seq::SliceRandom};

use self::{board::Board, piece::{PieceTemplate, PieceAgent, PieceType}, score::ScoreManager};

pub mod board;
pub mod piece;
pub mod score;


pub struct TetrisGame {
	pub board: Board,
	pub templates: Vec<PieceTemplate>,
	pub nexts: Vec<PieceTemplate>,
	pub active_piece: Option<PieceAgent>,
	pub rng: ThreadRng,
	pub score: ScoreManager,
}

impl TetrisGame {
	pub fn new() -> Self {
		TetrisGame {
			board: Board::blank(),
			templates: vec![
				PieceTemplate::new(vec![
					Vector2 {x: 0, y: 0},
					Vector2 {x: 1, y: 0},
					Vector2 {x: 0, y: -1},
					Vector2 {x: 1, y: -1},
				], PieceType::Square),
				PieceTemplate::new(vec![
					Vector2 {x: 0, y: -1},
					Vector2 {x: 0, y: 0},
					Vector2 {x: 0, y: 1},
					Vector2 {x: 0, y: 2},
				], PieceType::Line),
				PieceTemplate::new(vec![
					Vector2 {x: 1, y: -1},
					Vector2 {x: 0, y: -1},
					Vector2 {x: 0, y: 0},
					Vector2 {x: -1, y: 0},
				], PieceType::Z),				
				PieceTemplate::new(vec![
					Vector2 {x: 0, y: -1},
					Vector2 {x: -1, y: 0},
					Vector2 {x: 0, y: 0},
					Vector2 {x: 1, y: 0},
				], PieceType::T),
				PieceTemplate::new(vec![
					Vector2 {x: 0, y: -1},
					Vector2 {x: 0, y: 0},
					Vector2 {x: 0, y: 1},
					Vector2 {x: 1, y: 1},
				], PieceType::L),
				PieceTemplate::new(vec![
					Vector2 {x: 0, y: -1},
					Vector2 {x: 0, y: 0},
					Vector2 {x: 0, y: 1},
					Vector2 {x: -1, y: 1},
				], PieceType::J),
				PieceTemplate::new(vec![
					Vector2 {x: -1, y: -1},
					Vector2 {x: 0, y: -1},
					Vector2 {x: 0, y: 0},
					Vector2 {x: 1, y: 0},
				], PieceType::S),
			],
			rng: thread_rng(),
			active_piece: None,
			score: ScoreManager::new(),
			nexts: Vec::new()
		}
	}

	fn pick_template_copy(&mut self) -> PieceTemplate {
		self.board.array[0];
		self
			.templates
			.choose(&mut self.rng)
			.clone().expect("Cannot pick piece template").clone()
	}

	pub fn spawn_piece(&mut self, position: Vector2<i16>) {
		self.active_piece = Some(PieceAgent::new(self.pick_template_copy(), position));
	}

	pub fn hard_drop (&mut self) {
		if self.active_piece.is_none() {
			return;
		}
		let agent = self.active_piece.as_mut().unwrap();
		let distance = self.board.hard_drop_apply_only(agent);
		self.score.hard_drop(distance);
	}
}
