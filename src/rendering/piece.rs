use crate::logic::piece::Piece;

use super::Renderer;

pub trait Render {
	fn render(self, renderer: &mut Renderer) -> ();
}

impl Render for Piece {
	fn render(self, _renderer: &mut Renderer) {
		todo!("Implement this")
	}
}
