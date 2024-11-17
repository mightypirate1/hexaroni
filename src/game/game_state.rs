use crate::engine::Player;
use std::time::Instant;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum GameState {
    Editing, // anticipating the map-editor
    Waiting,
    Countdown {
        started_at: Instant,
    },
    Playing {
        current_player: Player,
        move_start: Instant,
        move_nr: usize,
    },
    GameOver {
        winner: Player,
    },
}

impl GameState {
    pub fn allows_moves(&self) -> bool {
        !matches!(self, GameState::Waiting | GameState::Countdown { .. })
    }

    pub fn on_apply_move(&self) -> GameState {
        match self {
            GameState::Playing {
                current_player,
                move_nr,
                ..
            } => GameState::Playing {
                current_player: current_player.opponent(),
                move_start: Instant::now(),
                move_nr: move_nr + 1,
            },
            _ => *self,
        }
    }

    pub fn winner(&self) -> Option<Player> {
        if let GameState::GameOver { winner } = self {
            Some(*winner)
        } else {
            None
        }
    }
}
