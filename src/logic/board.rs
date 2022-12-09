use super::piece::{Piece, PieceAgent};

pub struct Board {
	pub grid: Vec<Vec<Option<Piece>>>
}

impl Board {
	pub const WIDTH: usize = 10;
	pub const HEIGHT: usize = 20;

	pub fn blank() -> Self {
		Self {grid: vec![vec![None; Self::WIDTH];Self::HEIGHT]}
	}

	pub fn line_clear(&mut self) -> u8 {
		let mut res: u8 = 0;

		loop {
			let clear = self.grid.iter().last().unwrap().iter().all(|item| item.is_some());
			if clear {
				self.grid.remove(Self::HEIGHT-1);
				self.grid.insert(0, vec![None;Self::WIDTH]);
				res += 1;
				continue;
			} 
			break;
		};

		res
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
					let value = self.grid[y as usize][x as usize];
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
			self.grid[y as usize][x as usize] = Some(Piece::new(piece.template.piece_type))
		});
		
		dinstance
	}
}
