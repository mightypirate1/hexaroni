use crate::config::CONF;
use crate::engine::{
    moves::{Effect, Move},
    statuses::Status,
    Board, Object, Player,
};
use crate::geometry::{HexCoord, ScreenCoord};
use itertools::Itertools;
use macroquad::prelude::*;
use std::time::Instant;

use super::{GameSettings, GameState};

pub struct GameController {
    pub board: Board,
    pub game_state: GameState,
    pub game_settings: GameSettings,
}

impl Default for GameController {
    fn default() -> Self {
        GameController {
            board: Board::test_square(),
            game_state: GameState::Waiting,
            game_settings: GameSettings::default(),
        }
    }
}

impl GameController {
    pub fn new() -> GameController {
        GameController {
            board: Board::test_square(),
            ..Default::default()
        }
    }

    pub fn start_game(&mut self) {
        if let GameState::Waiting = self.game_state {
            self.game_state = GameState::Playing {
                current_player: self.game_settings.starting_player,
                move_start: Instant::now(),
            }
        } else {
            panic!("attempted to start game from state: {:?}", self.game_state);
        }
    }

    pub fn apply_move(&mut self, r#move: &Move, time: f32, move_duration: f32) {
        if !(self.game_state.allows_moves() && r#move.object.owned_by(&self.current_player())) {
            return;
        }
        self.move_to(&r#move.object, r#move.target(), time, move_duration);
        for effect in &r#move.effects {
            match effect {
                Effect::Kill { object } => {
                    if let Some(obj) = self.get_obj_mut(object) {
                        let killer_coord = ScreenCoord::from_hexcoord(&r#move.object.coord);
                        let obj_coord = ScreenCoord::from_hexcoord(&object.coord);
                        obj.props.dead = true;
                        obj.statuses.push(Status::Killed {
                            knockback: obj_coord.as_vec() - killer_coord.as_vec(),
                            start_time: time,
                            duration: CONF.kill_duration,
                        });
                    }
                }
            }
        }
        let opponents_is_dead = !self
            .board
            .objects
            .iter()
            .any(|o| o.owned_by(&self.current_player().opponent()) && !o.props.dead);
        if opponents_is_dead {
            self.game_state = GameState::GameOver {
                winner: self.current_player(),
            };
        } else {
            self.game_state = self.game_state.on_apply_move();
        }
    }

    pub fn current_player(&self) -> Player {
        match self.game_state {
            GameState::Playing { current_player, .. } => current_player,
            GameState::GameOver { winner } => winner,
            _ => Player::God,
        }
    }

    pub fn get_obj_mut(&mut self, object: &Object) -> Option<&mut Object> {
        self.board
            .objects
            .iter_mut()
            .find(|o| o.props.oid == object.props.oid)
    }

    pub fn get_object_at_pos(&self, pos: &ScreenCoord) -> Option<Object> {
        self.get_close_from_vec(pos, &self.board.objects)
    }

    pub fn get_tile_at_pos(&self, pos: &ScreenCoord) -> Option<Object> {
        self.get_close_from_vec(pos, &self.board.tiles)
    }

    /**
    run on start of game loop tick
    - cleans up dead objects
    - resets expired animations to none
     */
    pub fn on_tick_start(&mut self, time: f32) {
        let mut kills = vec![];
        self.board
            .tiles
            .iter_mut()
            .chain(self.board.objects.iter_mut())
            .for_each(|o| {
                if o.props.dead && !o.statuses.iter().any(|s| !s.is_expired(time)) {
                    kills.push(o.clone());
                }
                o.statuses = o
                    .statuses
                    .iter()
                    .filter(|s| !s.is_expired(time))
                    .cloned()
                    .collect();
            });
        for obj in kills {
            self.board.remove_object(&obj);
        }

        if let GameState::Playing { move_start, .. } = self.game_state {
            if move_start.elapsed().as_secs_f32() > self.game_settings.play_move_timeout {
                self.game_state = self.game_state.on_apply_move();
            }
        }
    }

    pub fn screen_size(&self) -> f32 {
        f32::min(
            0.33 * screen_width() / self.board.size as f32,
            0.58 * screen_height() / (1 + self.board.size) as f32,
        )
    }

    fn move_to(&mut self, object: &Object, to: &HexCoord, time: f32, duration: f32) {
        if let Some(obj) = self.get_obj_mut(object) {
            obj.statuses.push(Status::Move {
                from: ScreenCoord::from_hexcoord(&obj.coord),
                to: ScreenCoord::from_hexcoord(to),
                start_time: time,
                duration,
            });
            obj.set_coord(to);
        }
    }

    /**
     * Gets the closest object out of the ones that are closer than the size of the object
     */
    fn get_close_from_vec(&self, pos: &ScreenCoord, objects: &[Object]) -> Option<Object> {
        let screen_size = self.screen_size();
        let with_distances: Vec<(&Object, f32)> = objects
            .iter()
            .map(|o| (o, pos.dist_from(&o.get_screen_coord())))
            .collect();
        let detection = with_distances
            .iter()
            .filter(|(o, d)| *d < screen_size * o.props.size)
            .sorted_by(|(_, d1), (_, d2)| f32::total_cmp(d1, d2))
            .map(|(o, _)| o)
            .next();

        detection.map(|o| o.to_owned().to_owned())
    }
}
