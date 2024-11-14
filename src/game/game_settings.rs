use crate::{config::CONF, engine::Player};

pub struct GameSettings {
    pub play_move_timeout: f32,
    pub starting_player: Player,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            play_move_timeout: CONF.play_move_timeout,
            starting_player: Player::A,
        }
    }
}
