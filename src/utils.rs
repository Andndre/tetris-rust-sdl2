use crate::logic::board::Board;

pub fn xy_to_v (x: i16, y:i16) -> usize {
	(y * (Board::WIDTH as i16) + x) as usize
}
