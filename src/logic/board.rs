use crate::utils::xy_to_v;

use super::piece::{Piece, PieceAgent};

pub struct Board {
	pub array: [Option<Piece>; Self::SIZE]
}

impl Board {
	const WIDTH: usize = 10;
	const HEIGHT: usize = 20;
	const SIZE: usize = Self::WIDTH + Self::HEIGHT;

	pub fn blank() -> Self {
		Self {array: [None; Self::SIZE]}
	}

	pub fn hard_drop_apply_only(&mut self, piece: &mut PieceAgent) -> i16 {
		let dinstance = piece
			.template
			.offsets
			.iter()
			.map(|val| -> i16 {
				let x = val.x + piece.position.x;
				let mut y = val.y + piece.position.y;
				let mut distance = 0;
				while y < 20 {
					let value = self.array[xy_to_v(x, y)];
					if value.is_some() {
						break;
					}
					y += 1;
					distance += 1;
				};
				distance
			}).min().expect("FATAL: Cannot find minimum.. ?");
		
		piece.template.offsets.iter().for_each(|val| {
			let x = val.x + piece.position.x;
			let y = val.y + piece.position.y;
			let index = xy_to_v(x, y + dinstance);
			self.array[index] = Some(Piece::new(piece.template.piece_type))
		});
		
		dinstance
	}
}
