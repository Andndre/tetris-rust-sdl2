pub struct ScoreManager {
    pub value: i32,
}

impl ScoreManager {
    pub fn new() -> Self {
        ScoreManager { value: 0 }
    }
    pub fn hard_drop(&mut self, _distance: i16) {
        todo!("Handle scoring!");
    }
}
