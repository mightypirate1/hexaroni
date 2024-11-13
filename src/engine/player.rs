#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Player {
    A,
    B,
    God, // owns non-player objects
}

impl Player {
    pub fn opponent(&self) -> Player {
        match self {
            Player::A => Player::B,
            Player::B => Player::A,
            Player::God => panic!("No one opposes god"),
        }
    }
}
