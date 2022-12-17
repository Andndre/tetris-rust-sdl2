use rand::{rngs::ThreadRng, seq::SliceRandom, thread_rng};

use crate::utils::Vec2;

use self::{
    board::Board,
    piece::{PieceAgent, PieceTemplate, PieceType},
    score::ScoreManager,
};

pub mod board;
pub mod piece;
pub mod score;

pub struct TetrisGame {
    pub board: Board,
    templates: [PieceTemplate; 7],
    pub nexts: Vec<PieceTemplate>,
    pub active_piece: Option<PieceAgent>,
    pub rng: ThreadRng,
    pub score: ScoreManager,
}

impl TetrisGame {
    pub fn new() -> Self {
        TetrisGame {
            board: Board::blank(),
            templates: [
                PieceTemplate::new(
                    [
                        Vec2 { x: 0, y: 0 },
                        Vec2 { x: 1, y: 0 },
                        Vec2 { x: 0, y: -1 },
                        Vec2 { x: 1, y: -1 },
                    ],
                    PieceType::Square,
                ),
                PieceTemplate::new(
                    [
                        Vec2 { x: 0, y: -1 },
                        Vec2 { x: 0, y: 0 },
                        Vec2 { x: 0, y: 1 },
                        Vec2 { x: 0, y: 2 },
                    ],
                    PieceType::Line,
                ),
                PieceTemplate::new(
                    [
                        Vec2 { x: 1, y: -1 },
                        Vec2 { x: 0, y: -1 },
                        Vec2 { x: 0, y: 0 },
                        Vec2 { x: -1, y: 0 },
                    ],
                    PieceType::Z,
                ),
                PieceTemplate::new(
                    [
                        Vec2 { x: 0, y: -1 },
                        Vec2 { x: -1, y: 0 },
                        Vec2 { x: 0, y: 0 },
                        Vec2 { x: 1, y: 0 },
                    ],
                    PieceType::T,
                ),
                PieceTemplate::new(
                    [
                        Vec2 { x: 0, y: -1 },
                        Vec2 { x: 0, y: 0 },
                        Vec2 { x: 0, y: 1 },
                        Vec2 { x: 1, y: 1 },
                    ],
                    PieceType::L,
                ),
                PieceTemplate::new(
                    [
                        Vec2 { x: 0, y: -1 },
                        Vec2 { x: 0, y: 0 },
                        Vec2 { x: 0, y: 1 },
                        Vec2 { x: -1, y: 1 },
                    ],
                    PieceType::J,
                ),
                PieceTemplate::new(
                    [
                        Vec2 { x: -1, y: -1 },
                        Vec2 { x: 0, y: -1 },
                        Vec2 { x: 0, y: 0 },
                        Vec2 { x: 1, y: 0 },
                    ],
                    PieceType::S,
                ),
            ],
            rng: thread_rng(),
            active_piece: None,
            score: ScoreManager::new(),
            nexts: Vec::new(),
        }
    }

    pub fn init_game(&mut self) {
        for _i in 0..6 {
            let new_template = self.pick_template_copy();
            self.nexts.push(new_template);
        }
    }

    fn pick_template_copy(&mut self) -> PieceTemplate {
        self.templates
            .choose(&mut self.rng)
            .expect("Cannot pick piece template")
            .clone()
    }

    pub fn spawn_piece(&mut self) {
        let template = self.nexts.remove(5);
        let new_template = self.pick_template_copy();
        self.nexts.insert(0, new_template);
        self.active_piece = Some(PieceAgent::new(
            template,
            Vec2 {
                x: Board::WIDTH as i16 / 2,
                y: 3,
            },
        ));
    }

    pub fn hard_drop(&mut self) {
        if self.active_piece.is_none() {
            return;
        }
        let agen = self.active_piece.as_mut().unwrap();
        let distance = self.board.hard_drop_apply_only(agen);
        self.score.hard_drop(distance);
    }
}
