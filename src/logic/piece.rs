use crate::utils::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub color: PieceType,
}

impl Piece {
    pub fn new(color: PieceType) -> Self {
        Self { color }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
    Square,
    Line,
    L,
    Z,
    S,
    J,
    T,
}

pub struct PieceAgent {
    pub template: PieceTemplate,
    pub position: Vec2,
}

impl PieceAgent {
    pub fn new(template: PieceTemplate, position: Vec2) -> Self {
        PieceAgent { template, position }
    }

    pub fn rotate_right(&mut self) {
        for i in self.template.offsets.iter_mut() {
            let temp = i.y;
            i.y = -i.x;
            i.x = temp;
        }
    }

    pub fn rotate_left(&mut self) {
        for i in self.template.offsets.iter_mut() {
            let temp = i.y;
            i.y = i.x;
            i.x = -temp;
        }
    }
}

#[derive(Debug, Clone)]
pub struct PieceTemplate {
    pub offsets: [Vec2; 4],
    pub piece_type: PieceType,
}

impl PieceTemplate {
    pub fn new(offsets: [Vec2; 4], piece_type: PieceType) -> Self {
        Self {
            offsets,
            piece_type,
        }
    }
}
