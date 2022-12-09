use cgmath::Vector2;

#[derive(Debug, Clone, Copy)]
pub struct Piece {
	pub color: PieceType
}

impl Piece {
	pub fn new(color: PieceType) -> Self {
		Self { color }
	}
}

#[derive(Debug, Clone, Copy)]
pub enum PieceType {
	Square, Line, L, Z, S, J, T
}

pub struct PieceAgent {
	pub template: PieceTemplate,
	pub position: Vector2<i16>
}

impl PieceAgent {
	pub fn new(template: PieceTemplate, position: Vector2<i16>) -> Self {
		PieceAgent { template, position }
	}

	pub fn rotate_right(&mut self) {

	}

	pub fn rotate_left(&mut self) {
		
	}
}

#[derive(Debug, Clone)]
pub struct PieceTemplate {
	pub offsets: Vec<Vector2<i16>>,
	pub piece_type: PieceType
}

impl PieceTemplate {
	pub fn new(offsets: Vec<Vector2<i16>>, piece_type: PieceType) -> Self {
		Self { offsets, piece_type }
	}
}
