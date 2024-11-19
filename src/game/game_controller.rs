use crate::config::CONF;
use crate::engine::{
    statuses::{Effect, Status},
    Board, Object, ObjectType, Player,
};
use crate::game::moves::Move;
use crate::geometry::{HexCoord, ScreenCoord};
use itertools::Itertools;
use macroquad::prelude::*;
use std::time::Instant;

use super::GameState;

pub struct GameController {
    pub board: Board,
    pub game_state: GameState,
}

impl Default for GameController {
    fn default() -> Self {
        GameController {
            board: Board::test_square(),
            game_state: GameState::Waiting,
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
            self.game_state = GameState::Countdown {
                started_at: Instant::now(),
            }
        } else {
            panic!("attempted to start game from state: {:?}", self.game_state);
        }
    }

    pub fn apply_move(&mut self, r#move: &Move, time: f32, move_duration: f32) {
        if !self.game_state.allows_moves() || !r#move.object.owned_by(&self.current_player()) {
            return;
        }
        self.move_to(&r#move.object, r#move.target(), time, move_duration);
        let mut effects_to_apply = r#move.effects.clone();
        if let GameState::Playing { move_nr, .. } = self.game_state {
            // get object effects
            effects_to_apply.extend(self.tick_objects(move_nr, time));
        }
        // run game logic to setup next move
        self.apply_effects(&effects_to_apply, time);

        if let Some(winner) = self.winner() {
            self.game_state = GameState::GameOver { winner };
        } else {
            self.game_state = self.game_state.on_apply_move();
        }
    }

    pub fn current_player(&self) -> Player {
        match self.game_state {
            GameState::Playing { current_player, .. } => current_player,
            GameState::GameOver { winner } => winner,
            _ => Player::A,
        }
    }

    pub fn get_piece_at_pos(&self, pos: &ScreenCoord) -> Option<Object> {
        self.get_close_from_vec(pos, &self.board.pieces())
    }

    pub fn get_tile_at_pos(&self, pos: &ScreenCoord) -> Option<Object> {
        self.get_close_from_vec(pos, &self.board.tiles())
    }

    /**
    run on start of game loop tick
    - cleans up dead objects
    - resets expired animations to none
    - swap player's turn if the move time is up
     */
    pub fn tick(&mut self, time: f32) {
        let mut kills = vec![];
        self.board.objects_mut().iter_mut().for_each(|o| {
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

        match self.game_state {
            GameState::Countdown { started_at } => {
                if started_at.elapsed().as_secs_f32() > CONF.game_start_countdown {
                    self.game_state = GameState::Playing {
                        current_player: CONF.starting_player,
                        move_start: Instant::now(),
                        move_nr: 0,
                    }
                }
            }
            GameState::Playing {
                move_start,
                move_nr,
                ..
            } => {
                if move_start.elapsed().as_secs_f32() > CONF.play_move_timeout {
                    self.game_state = self.game_state.on_apply_move();
                    let effects = self.tick_objects(move_nr, time);
                    self.apply_effects(&effects, time);
                }
            }
            _ => {}
        }

        if let Some(winner) = self.winner() {
            self.game_state = GameState::GameOver { winner };
        }
    }

    fn tick_objects(&mut self, move_nr: usize, time: f32) -> Vec<Effect> {
        self.board
            .objects_mut()
            .iter_mut()
            .flat_map(|o| o.tick(move_nr, time))
            .collect()
    }

    fn move_to(&mut self, object: &Object, to: &HexCoord, time: f32, duration: f32) {
        if let Some(obj) = self.board.get_as_mut(object) {
            let height = if obj.otype == ObjectType::Jumper {
                0.5
            } else {
                0.0
            };
            obj.statuses.push(Status::new_move(
                ScreenCoord::from_hexcoord(&obj.coord),
                ScreenCoord::from_hexcoord(to),
                time,
                duration,
                height,
            ));
            obj.set_coord(to);
        }
    }

    fn winner(&self) -> Option<Player> {
        let mut player_a_alive = false;
        let mut player_b_alive = false;
        for p in self.board.pieces().iter().filter(|p| !p.props.dead) {
            if p.owned_by(&Player::A) {
                player_a_alive = true;
            }
            if p.owned_by(&Player::B) {
                player_b_alive = true;
            }
        }
        match (player_a_alive, player_b_alive) {
            (false, false) => Some(Player::God),
            (true, false) => Some(Player::A),
            (false, true) => Some(Player::B),
            (true, true) => None,
        }
    }

    fn apply_effects(&mut self, effects: &Vec<Effect>, time: f32) {
        for effect in effects {
            match effect {
                Effect::Kill { victim, .. } => {
                    if let Some(v) = self.board.get_as_mut(victim) {
                        let status = effect.applying_status(time);
                        v.set_killed(status.as_ref());
                    }
                }
                Effect::KillAallOn { coord, .. } => {
                    let status = effect.applying_status(time);
                    self.board.kill_all_at(coord, status);
                }
                Effect::SetStatus { object, .. } => {
                    let status = effect.applying_status(time);
                    if let Some(o) = self.board.get_as_mut(object) {
                        o.add_status(&status.expect("SetStatus without a status"));
                    }
                }
                Effect::NoOp => {}
            }
        }
    }

    /**
     * Gets the closest object out of the ones that are closer than the size of the object
     */
    fn get_close_from_vec(&self, pos: &ScreenCoord, objects: &[&Object]) -> Option<Object> {
        let with_distances: Vec<(&Object, f32)> = objects
            .iter()
            .map(|o| (*o, pos.dist_from(&o.get_screen_coord())))
            .collect();
        let detection = with_distances
            .iter()
            .filter(|(o, d)| *d < o.props.size)
            .sorted_by(|(_, d1), (_, d2)| f32::total_cmp(d1, d2))
            .map(|(o, _)| *o)
            .next();

        detection.cloned()
    }
}
