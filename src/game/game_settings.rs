use crate::{config::CONF, engine::Player};

pub struct GameSettings {
    pub play_move_timeout: f32,
    pub game_start_countdown: f32,
    pub starting_player: Player,
}

impl Default for GameSettings {
    fn default() -> Self {
        GameSettings {
            play_move_timeout: CONF.play_move_timeout,
            game_start_countdown: CONF.game_start_countdown,
            starting_player: Player::A,
        }
    }
}
