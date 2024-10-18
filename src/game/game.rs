use super::Board;


pub struct Game {
    pub board: Board
}

impl Game {
    pub fn new() -> Game {
        Game {board: Board::new(7)}
    }
}
